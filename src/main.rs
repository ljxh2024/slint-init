use std::{
    env,
    fs::{self, File},
    io::Write,
    process::ExitCode,
};

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Invalid parameter");
        return ExitCode::FAILURE;
    }

    if let Err(e) = create_file(&args[1]) {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn create_file(dir: &str) -> std::io::Result<()> {
    // create directory: src and ui
    fs::create_dir_all(dir.to_string() + "/src")?;
    fs::create_dir(dir.to_string() + "/ui")?;

    // main.rs
    let mut main_file = File::create(dir.to_string() + "/src/main.rs")?;
    let main_rs = r#"#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    window.run()
}"#;
    main_file.write_all(main_rs.as_bytes())?;

    // Cargo.toml
    let mut cargo_toml_file = File::create(dir.to_string() + "/Cargo.toml")?;
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
slint = "1.13.0"

[build-dependencies]
slint-build = "1.13.0""#,
        dir
    );
    cargo_toml_file.write_all(cargo_toml.as_bytes())?;

    // app-window.slint
    let mut slint_file = File::create(dir.to_string() + "/ui/app-window.slint")?;
    let slint = r#"export component MainWindow inherits Window {
    Text {
        text: "hello world!";
    }
}"#;
    slint_file.write_all(slint.as_bytes())?;

    // build.rs
    let mut build_rs_file = File::create(dir.to_string() + "/build.rs")?;
    let build_rs = r#"fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}"#;
    build_rs_file.write_all(build_rs.as_bytes())?;

    // .gitignore
    let mut gitignore_file = File::create(dir.to_string() + "/.gitignore")?;
    gitignore_file.write_all(b"/target")?;

    // readme.md
    let mut readme_file = File::create(dir.to_string() + "/README.md")?;
    readme_file.write_all(format!("# {dir}").as_bytes())?;

    Ok(())
}
