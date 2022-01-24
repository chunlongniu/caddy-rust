use regex::Regex;
use std::collections::HashMap;
use super::start_cmd::{StartCommand};

#[derive(Debug)]
pub enum Flag {
    StrEntity(String, String, String),
    BoolEntity(String, bool, String),
}

pub struct Command<'a> {
    args:  &'a Vec<String>,
    flags: HashMap<String, Flag>,
}

pub trait SubCommandHelp{
    fn execute(&self);
}

impl<'a> Command<'a> {

    pub fn new(args: &'a Vec<String>) -> Self {
        Command{
            args: args,
            flags: HashMap::from([
                ("--config".to_string(),
                 Flag::StrEntity(
                    "config".to_string(),
                    "".to_string(),
                    "Configuration file".to_string()),
                 ),
                ("--envfile".to_string(),
                 Flag::StrEntity(
                    "envfile".to_string(),
                    "".to_string(),
                    "Environment file to load".to_string()),
                ),
                ("--adapter".to_string(),
                 Flag::StrEntity(
                    "adapter".to_string(),
                    "".to_string(),
                    "Name of config adapter to apply".to_string()),
                ),
                ("--pidfile".to_string(),
                Flag::StrEntity(
                    "pidfile".to_string(),
                    "".to_string(),
                    "Path of file to which to write process ID".to_string()),
                ),
                ("--watch".to_string(),
                Flag::BoolEntity(
                    "watch".to_string(),
                    false,
                    "Reload changed config file automatically".to_string()),
                )]
            )
        }
    }

    fn parse_flags(&self) -> i32{
        let opts_in_line = &self.args[2..].join(" ");
        let re = Regex::new(r"(--\w+\s*[a-zA-Z]*)").unwrap();
        let opts: Vec<String> = vec![];

        for cap in re.captures_iter(opts_in_line) {
            let opt = cap[1].trim_end();
            if opt.eq("--help") {
                return -1;
            }
            //opts.add(cap[1].trim_end())
        }
        return 0;
    }

    pub fn execute<T>(&self, cmd: &T) where T: SubCommandHelp {
        self.parse_flags();
    }

    fn help<T>(&self, cmd: &T) where T: SubCommandHelp {
            
    }
}
