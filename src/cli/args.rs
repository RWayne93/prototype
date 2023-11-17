use clap::{App, Arg};

pub fn get_matches() -> clap::ArgMatches {
    App::new("Rust Project Generator")
        .version("0.1.0")
        .author("Your Name")
        .about("Generates project templates for Rust web frameworks")
        .arg(Arg::with_name("name")
             .help("Sets the name of the project to create")
             .index(1))
        .arg(Arg::with_name("framework")
             .help("Specifies the web framework to use (e.g., rocket, actix, axum)")
             .takes_value(true)
             .short('f')
             .long("framework"))
        .get_matches()
}