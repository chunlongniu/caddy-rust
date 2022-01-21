use std::io;
mod command;

use command::{SubCommandHelp, StartCommand};
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

pub trait CommandLine {

    fn parse(&self) -> io::Result<()>;    
} 

pub struct Cli {
    command: String,
    version: String,
    cfg_file: String, 
    usage: String
}

pub fn buildup_cli(args: &Vec<String>) -> Result<Cli, &'static str> {
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


impl Cli {

    pub fn new(args: &Vec<String>) -> Self {
        
        let command = pickup_cmd_params(args, 1, "start");
        let cfg_file = pickup_cmd_params(args, 2, "./Caddyfile");

        Cli {
            command: String::from(command),
            cfg_file: cfg_file,
            usage: String::from(USAGE), 
            version: String::from("v1.0.0"),
        }
    }

    pub fn execute(&self, args: &Vec<String>) {
        let command: &str = &self.command;
        match command {
            "version" =>
                self.echo_version(),
            "help" =>
                self.echo_usage(),
            "start" => {
                let sub_cmd = StartCommand::new();
                sub_cmd.execute(args);
            },
            _ => self.echo_usage(),
        }
    }

    fn echo_usage(&self) {
        println!("{}", self.usage)
    }

    fn echo_version(&self) {
        println!("{}", self.version)
    }
}
