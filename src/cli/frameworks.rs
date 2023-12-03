use std::path::PathBuf;
use std::env;
use std::fs;

const ROCKET_TEMPLATE: &str = include_str!("../templates/frameworks/rocket/rocket.rs");
const AXUM_TEMPLATE: &str = include_str!("../templates/frameworks/axum/axum.rs");
const ACTIX_TEMPLATE: &str = include_str!("../templates/frameworks/actix/actix.rs");
const ROCKET_CARGO_TEMPLATE: &str = include_str!("../templates/frameworks/rocket/Cargo.toml");
const AXUM_CARGO_TEMPLATE: &str = include_str!("../templates/frameworks/axum/Cargo.toml");
const ACTIX_CARGO_TEMPLATE: &str = include_str!("../templates/frameworks/actix/Cargo.toml");

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
    fn get_templates(&self) -> (String, String) {
        match self {
            Framework::Rocket => (ROCKET_TEMPLATE.to_string(), ROCKET_CARGO_TEMPLATE.to_string()),
            Framework::Actix => (ACTIX_TEMPLATE.to_string(), ACTIX_CARGO_TEMPLATE.to_string()),
            Framework::Axum => (AXUM_TEMPLATE.to_string(), AXUM_CARGO_TEMPLATE.to_string()),
        }
    }

    pub fn generate_files(&self, project_name: &str) -> std::io::Result<()> {
        let project_dir = get_absolute_path(project_name);
        fs::create_dir_all(&project_dir.join("src"))?;

        let (template, cargo_template) = self.get_templates();
        fs::write(&project_dir.join("src/main.rs"), template)?;

        let cargo_template = cargo_template.replace("name = \"\"", &format!("name = \"{}\"", project_name));
        fs::write(&project_dir.join("Cargo.toml"), cargo_template)?;

        Ok(())
    }
}
