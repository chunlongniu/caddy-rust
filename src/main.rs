mod cmd;
mod events;
use std::env;
use cmd::{build_cmd_cli};

fn main() {
    let args: Vec<String> = env::args().collect();
    match build_cmd_cli(&args) {
        Ok(cli) => cli.execute(&args),
        Err(e) => println!("error {:?}", e),
    }
}
