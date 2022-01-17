mod vars;
use std::io;
use std::collections::HashMap;
use vars::Command;

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
    cfg_file: Result<String, u8>, 
    cmd_action_map: HashMap<String, Command>,
    usage: String
}

fn parse_direct(args: &Vec<String>) -> String {
    let mut command = String::from("start");
    if args.len()  > 2 {
        command = String::from(&args[1]);
    }
    command
}

fn parse_cfg_file(args: &Vec<String>) -> Result<String, u8> {
    if args.len()  > 3 {
        let cfg_file = String::from(&args[2]);
        return Ok(cfg_file);
    }
    Err(0)
}

fn init_cmd_action_map(cmd_actions :&mut HashMap<String, Command>) {
    cmd_actions.insert("start".to_string(), Command::START);
    cmd_actions.insert("version".to_string(), Command::VERSION("1.0".to_string()));
    cmd_actions.insert("help".to_string(), Command::HELP("help".to_string()));
}

impl Cli {

    pub fn new(args: &Vec<String>) -> Cli {
        let command = self::parse_direct(args);
        let cfg_file = self::parse_cfg_file(args);
        let cmd_action_map = HashMap::<String, Command>::new();
        Cli {
            command: String::from(command),
            cfg_file: cfg_file,
            usage: String::from(USAGE), 
            cmd_action_map: cmd_action_map,
            version: String::from("1.0.0"),
        }
    }

    pub fn execute(&self) {
        match self.command.to_string() {
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

impl CommandLine for Cli {

    fn parse(&self) -> io::Result<()>{
        Ok(())
    }

}
