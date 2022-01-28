//use std::io;
mod commands;
mod error;
mod start_cmd;
use start_cmd::{StartCommand};
use commands::{Command};
static USAGE: &'static str = 
"usage:
    caddy <command> [<args>...]

commands:
    adapt           Adapts a configuration to Caddy's native JSON
    add-package     Adds Caddy packages (EXPERIMENTAL)
    build-info      Prints information about this build
    environ         Prints the environment
    file-server     Spins up a production-ready file server
    fmt             Formats a Caddyfile
    hash-password   Hashes a password and writes base64
    help            Shows help for a Caddy subcommand
    list-modules    Lists the installed Caddy modules
    reload          Changes the config of the running Caddy instance
    remove-package  Removes Caddy packages (EXPERIMENTAL)
    reverse-proxy   A quick and production-ready reverse proxy
    run             Starts the Caddy process and blocks indefinitely
    start           Starts the Caddy process in the background and then returns
    stop            Gracefully stops a started Caddy process
    trust           Installs a CA certificate into local trust stores
    untrust         Untrusts a locally-trusted CA certificate
    upgrade         Upgrade Caddy (EXPERIMENTAL)
    validate        Tests whether a configuration file is valid
    version         Prints the version

Use 'caddy help <command>' for more information about a command.
Full documentation is available at:
https://caddyserver.com/docs/command-line
";

pub struct Cli<'a>{
    sub_cmd: Command<'a>,
    version: String,
    usage: String
}

pub fn build_cmd_cli(args: &Vec<String>) -> Result<Cli, &'static str> {
    if args.len()  > 1 {
        let cli = Cli::new(args);
        return Ok(cli); 
    }
    Err("No arguments provided by OS: args[0] must be command")
}

fn pickup_cmd_params(args :&Vec<String>, index: usize, default: &str) -> String  {
    let arg = args.get(index);
    match arg {
        Some(value) => String::from(value),
        None => String::from(default),
    }
}

impl <'a> Cli<'a> {

    pub fn new(args: &'a Vec<String>) -> Self {

        Cli {
            sub_cmd: Command::new(args),
            usage: String::from(USAGE), 
            version: String::from("v1.0.0"),
        }
    }

    pub fn execute(&mut self, args: &Vec<String>) {
        let sub_cmd: &str = &pickup_cmd_params(args, 1, "start");
        match sub_cmd {
            "version" =>
                println!("{}", self.version),
            "help" =>
                println!("{}", self.usage),
            "start" => {
                let mut start_cmd = StartCommand::new();
                self.sub_cmd.execute(&mut start_cmd);
            },
            _ => {
                println!("[ERROR] {:?} is not a recognized sub command; see 'caddy help'", args.get(1).unwrap());
            },
        }
    }
}
