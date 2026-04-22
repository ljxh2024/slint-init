use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
    process::ExitCode,
};

const VERSION: &str = "slint-init 0.1.3 (2026-04-22)";
const SLINT_VERSION: &str = "1.16.0";

const HELP_TEXT: &str = r#"A command-line tool for quickly initializing Slint projects

Usage: slint-init [OPTIONS] PROJECT_NAME

OPTIONS:
  -v    Print version
  -h    Print help
"#;

const MAIN_RS: &str = r#"#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    window.run()
}"#;

const APP_WINDOW_SLINT: &str = r#"export component MainWindow inherits Window {
    Text {
        text: "hello world!";
    }
}"#;

const BUILD_RS: &str = r#"fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}"#;

const GITIGNORE: &str = "/target";

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<String>>();

    match args.get(1).map(|s| s.as_str()) {
        None | Some("-h") => {
            println!("{VERSION}\n{HELP_TEXT}");
            ExitCode::SUCCESS
        },
        Some("-v") => {
            println!("{VERSION}");
            ExitCode::SUCCESS
        },
        Some(arg) if args.len() == 2 => {
            if let Err(e) = init_project(arg) {
                eprintln!("Error: {e}");
                ExitCode::FAILURE
            } else {
                println!("Project '{}' created successfully!", arg);
                ExitCode::SUCCESS
            }
        },
        _ => {
            eprintln!("Error: Invalid arguments\n{HELP_TEXT}");
            ExitCode::FAILURE
        }
    }
}

fn init_project(dir: &str) -> std::io::Result<()> {
    let path = Path::new(dir);

    if !is_valid_project_name(dir) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid project name.",
        ));
    }

    // Create directory structure
    fs::create_dir_all(path.join("src"))?;
    fs::create_dir_all(path.join("ui"))?;

    // Write files
    write_file(path.join("src/main.rs"), MAIN_RS.as_bytes())?;
    write_file(path.join("build.rs"), BUILD_RS.as_bytes())?;
    write_file(path.join("ui/app-window.slint"), APP_WINDOW_SLINT.as_bytes())?;
    write_file(path.join("Cargo.toml"), generate_cargo_toml(dir).as_bytes())?;
    write_file(path.join(".gitignore"), GITIGNORE.as_bytes())?;
    write_file(path.join("README.md"), format!("# {}", dir).as_bytes())?;

    Ok(())
}

fn is_valid_project_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    
    // Crate names must be valid Rust identifiers
    name.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '_' || c == '-'
    }) && !name.starts_with('-')
}

fn write_file<P: AsRef<Path>>(path: P, content: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(path.as_ref())?;
    file.write_all(content)?;
    Ok(())
}

fn generate_cargo_toml(project_name: &str) -> String {
    format!(
        r#"[package]
name = "{project_name}"
version = "0.1.0"
edition = "2024"

[dependencies]
slint = "{SLINT_VERSION}"

[build-dependencies]
slint-build = "{SLINT_VERSION}"
"#
    )
}