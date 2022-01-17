mod cmd;
mod events;
use std::env;
use cmd::Cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = Cli::new(&args);
    cli.execute();
}
