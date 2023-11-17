mod cli {
    pub mod args;
    pub mod tui;
    pub mod cmd;
    }
    
fn main() {
    cli::cmd::run();
}
    