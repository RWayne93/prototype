use std::path::PathBuf;
use std::env;
use std::fs;

const ROCKET_TEMPLATE: &str = include_str!("../templates/frameworks/rocket/rocket.rs");
const AXUM_TEMPLATE: &str = include_str!("../templates/frameworks/axum/axum.rs");
const ROCKET_CARGO_TEMPLATE: &str = include_str!("../templates/frameworks/rocket/Cargo.toml");
const AXUM_CARGO_TEMPLATE: &str = include_str!("../templates/frameworks/axum/Cargo.toml");

fn get_absolute_path(relative_path: &str) -> PathBuf {
    let mut dir = env::current_dir().expect("Failed to get current directory");
    dir.push(relative_path);
    dir
}

pub enum Framework {
    Rocket,
    Actix,
    Axum,
}

impl Framework {
    fn get_template(&self) -> String {
        match self {
            Framework::Rocket => ROCKET_TEMPLATE.to_string(),
            Framework::Actix => include_str!("../templates/frameworks/actix/actix.rs").to_string(),
            Framework::Axum => AXUM_TEMPLATE.to_string(),
        }
    }

    fn get_cargo_template(&self) -> String {
        match self {
            Framework::Rocket => ROCKET_CARGO_TEMPLATE.to_string(),
            Framework::Actix => include_str!("../templates/frameworks/actix/Cargo.toml").to_string(),
            Framework::Axum => AXUM_CARGO_TEMPLATE.to_string(),
        }
    }

    pub fn generate_main_rs(&self, project_name: &str) -> std::io::Result<()> {
        let project_dir = get_absolute_path(project_name);
        fs::create_dir_all(&project_dir.join("src"))?;

        let template = self.get_template();
        fs::write(&project_dir.join("src/main.rs"), template)?;

        Ok(())
    }

    // Generate Cargo.toml file based on selected framework
    pub fn generate_cargo_toml(&self, project_name: &str) -> std::io::Result<()> {
        let project_dir = get_absolute_path(project_name);
        let mut cargo_template = self.get_cargo_template();

        cargo_template = cargo_template.replace("name = \"\"", &format!("name = \"{}\"", project_name));
        fs::write(&project_dir.join("Cargo.toml"), cargo_template)?;

        Ok(())
    }
}