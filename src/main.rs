mod cmd;
mod events;
use std::env;
use cmd::{buildup_cli};

fn main() {
    let args: Vec<String> = env::args().collect();
    match buildup_cli(&args) {
        Ok(cli) => cli.execute(&args),
        Err(e) => println!("error {:?}", e),
    }
}
