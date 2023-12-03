use dialoguer::{theme::ColorfulTheme, Select};


pub fn print_ascii_art() {
    let ascii_art = r#"
 _____           _        _                    
|  __ \         | |      | |                   
| |__) | __ ___ | |_ ___ | |_ _   _ _ __   ___ 
|  ___/ '__/ _ \| __/ _ \| __| | | | '_ \ / _ \
| |   | | | (_) | || (_) | |_| |_| | |_) |  __/
|_|   |_|  \___/ \__\___/ \__|\__, | .__/ \___|
                               __| | |         
                              |___/|_|                
"#;

    println!("{}", ascii_art);
}

pub fn get_example() -> String {
    let examples = ["chat", "hello"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an example")
        .default(0)
        .items(&examples)
        .interact()
        .unwrap();

    examples[selection].to_string()
}

pub fn get_project_name() -> String {
    dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .interact_text()
        .unwrap()
}

pub fn get_framework() -> String {
    let frameworks = ["Rocket", "Actix", "Axum", "Rocket from GitHub"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a framework")
        .default(0)
        .items(&frameworks)
        .interact()
        .unwrap();

    frameworks[selection].to_string()
}




