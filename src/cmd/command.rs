use regex::Regex;
pub enum Flag {
    StrEntity(String, String, String),
    BoolEntity(String, bool, String),
}

pub trait SubCommandHelp {
    fn help(&self);
    fn execute(&self, args: &Vec<String>);
}

pub struct StartCommand {
    name: String,
    usage: String,
    flags: Vec<Flag>,
}

impl StartCommand {

    pub fn new() -> StartCommand {
        StartCommand {
            name: "start".to_string(),
            usage: "[--config <path> [--adapter <name>]] \
                    [--envfile <path>] [--watch] \
                    [--pidfile <file>]".to_string(),
            flags: vec![
                Flag::StrEntity(
                    "config".to_string(),
                    "".to_string(),
                    "Configuration file".to_string()),
                Flag::StrEntity(
                    "envfile".to_string(),
                    "".to_string(),
                    "Environment file to load".to_string()),
                Flag::StrEntity(
                    "adapter".to_string(),
                    "".to_string(),
                    "Name of config adapter to apply".to_string()),
                Flag::StrEntity(
                    "pidfile".to_string(),
                    "".to_string(),
                    "Path of file to which to write process ID".to_string()),
                Flag::BoolEntity(
                    "watch".to_string(),
                    false,
                    "Reload changed config file automatically".to_string()),
            ],}
    }

    pub fn assign_value_to_flags(&self, options: String) {
        let cmd_line_args = &options;
        let re = Regex::new(r"[--(\w+\s+\w+)]*").unwrap();
        println!("---");
        for cap in re.captures_iter(cmd_line_args) {
            println!("{:?}", cap); 
        }
    } 
}

mod common {
    pub fn parse_flags(flags: &Vec<super::Flag>) {
        for flag in flags {
            match flag {
                super::Flag::StrEntity(opt, _, desc) => {
                    println!("  --{} string \n \t {}", opt, desc);
                },
                super::Flag::BoolEntity(opt, _, desc) => {
                    println!("  --{} \n \t {}", opt, desc);
                },
            }
        }            
    }

}

impl SubCommandHelp for StartCommand {

    fn help(&self) {
        println!("Usage of {} ", self.name);
        println!("caddy start {}", self.usage);
        common::parse_flags(&self.flags);
    }

    fn execute(&self, args: &Vec<String>) {
        let sub_cmd_flags = &args[2..]; 

        for flag in sub_cmd_flags {
            let m_flag: &str = &flag;
            match m_flag {
                "--help" => self.help(),
                _ => {
                    self.assign_value_to_flags(sub_cmd_flags.join(" "));
                    break;
                }
            }
        }
        
    }
}
