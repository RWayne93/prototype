mod cli {
    pub mod args;
    pub mod tui;
    pub mod cmd;
    pub mod frameworks;
    }
    
fn main() {
    cli::cmd::run();
}
    