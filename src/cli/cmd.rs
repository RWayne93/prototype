use super::args;
use super::tui;
use super::frameworks::Framework;
//use crate::cli::frameworks::Framework;

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

    let framework = match framework.to_lowercase().as_str() {
        "rocket" => Framework::Rocket,
        "axum" => Framework::Axum,
        _ => {
            eprintln!("Framework '{}' is not supported", framework);
            return;
        }
    };

    if let Err(e) = framework.generate_main_rs(&project_name) {
        eprintln!("Failed to generate main.rs file: {}", e);
    }
    if let Err(e) = framework.generate_cargo_toml(&project_name) {
        eprintln!("Failed to generate Cargo.toml file: {}", e);
    }
}