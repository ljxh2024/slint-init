use std::{
    env,
    fs::{self, File},
    io::Write,
    process::ExitCode,
};

const VERSION: &str = "slint-init 0.1.1 (2025-09-12)";

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<String>>();

    // print help
    if args.len() == 1 || args[1] == "-h" {
        println!("{VERSION}");
        println!(
            r#"A command-line tool for quickly initializing Slint projects

Usage: slint-init [OPTIONS] PROJECT_NAME

OPTIONS:
  -v    Print version
  -h    Print help
"#
        );
        return ExitCode::SUCCESS;
    }

    // print version
    if args[1] == "-v" {
        println!("{VERSION}");
        return ExitCode::SUCCESS;
    }

    if args.len() > 2 {
        eprintln!("Invalid pattern");
        return ExitCode::FAILURE;
    }

    if let Err(e) = init_project(&args[1]) {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

// Initialize a new Slint project in the specified directory
fn init_project(dir: &str) -> std::io::Result<()> {
    // create directory: src and ui
    fs::create_dir_all(dir.to_string() + "/src")?;
    fs::create_dir(dir.to_string() + "/ui")?;

    // main.rs
    File::create(dir.to_string() + "/src/main.rs")?.write_all(
        r#"#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    window.run()
}"#
        .as_bytes(),
    )?;

    // Cargo.toml
    File::create(dir.to_string() + "/Cargo.toml")?.write_all(
        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
slint = "1.13.1"

[build-dependencies]
slint-build = "1.13.1"
"#,
            dir
        )
        .as_bytes(),
    )?;

    // app-window.slint
    File::create(dir.to_string() + "/ui/app-window.slint")?.write_all(
        r#"export component MainWindow inherits Window {
    Text {
        text: "hello world!";
    }
}"#
        .as_bytes(),
    )?;

    // build.rs
    File::create(dir.to_string() + "/build.rs")?.write_all(
        r#"fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}"#
        .as_bytes(),
    )?;

    // .gitignore
    File::create(dir.to_string() + "/.gitignore")?.write_all(b"/target")?;
    // readme.md
    File::create(dir.to_string() + "/README.md")?.write_all(format!("# {dir}").as_bytes())
}
