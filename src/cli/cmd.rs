use super::args;
use super::tui;
use std::fs;
use std::env;
use std::path::PathBuf;

const ROCKET_TEMPLATE: &str = include_str!("../templates/frameworks/rocket.rs");

fn get_absolute_path(relative_path: &str) -> PathBuf {
    let mut dir = env::current_dir().expect("Failed to get current directory");
    dir.push(relative_path);
    dir
}

pub fn generate_rocket_file(project_name: &str) -> std::io::Result<()> {
    let project_dir = get_absolute_path(project_name);
    fs::create_dir_all(&project_dir.join("src"))?;

    fs::write(&project_dir.join("src/main.rs"), ROCKET_TEMPLATE)?;
    Ok(())
}

fn generate_cargo_toml(project_name: &str) -> std::io::Result<()> {
    let project_dir = get_absolute_path(project_name);
    let mut cargo_template = include_str!("../templates/Cargo.toml").to_string();

    cargo_template = cargo_template.replace("name = \"\"", &format!("name = \"{}\"", project_name));

    fs::write(&project_dir.join("Cargo.toml"), cargo_template)?;
    Ok(())
}

pub fn run() {
    let matches = args::get_matches();

    let project_name;
    let framework;

    if matches.is_present("name") && matches.is_present("framework") {
    project_name = matches.value_of("name").unwrap().to_string();
    framework = matches.value_of("framework").unwrap().to_string();
    } else {
    tui::print_ascii_art();

    project_name = tui::get_project_name();
    framework = tui::get_framework();
    }

    println!("Creating a project named '{}' using the '{}' framework", project_name, framework);

    if framework.to_lowercase() == "rocket" {
        if let Err(e) = generate_rocket_file(&project_name) {
            eprintln!("Failed to generate Rocket file: {}", e);
        }
        if let Err(e) = generate_cargo_toml(&project_name) {
            eprintln!("Failed to generate Cargo.toml file: {}", e);
        }
    } else {
        eprintln!("Framework '{}' is not supported", framework);
    }
}