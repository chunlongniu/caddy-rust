use regex::Regex;
use std::collections::HashMap;
use cmd::{StartCommand};

#[derive(Debug)]
pub enum Flag {
    StrEntity(&str, &str, &str),
    BoolEntity(&str, bool, &str),
}

pub Command {
    cmd: String,
    args: &Vec<String>,
    options: HashMap<str, Flag>,
}

pub struct SubCommand {

}

pub trait SubCommandHelp{
    fn execute(&self, args: &Vec<String>);
    fn help(&self, args:&Vec<Flag>);
}

impl<T> Command <T> where T: SubCommandHelp {

    fn new(args: &Vec<String>) => {
        command: StartCommand::new("start"),
        args: args,
        options: HashMap::from([
            ("--config",
             Flag::StrEntity(
                "config", "",
                "Configuration file"),),
            ("--envfile",
             Flag::StrEntity(
                "envfile", "",
                "Environment file to load"),
            ),
            ("--adapter",
             Flag::StrEntity(
                "adapter", "",
                "Name of config adapter to apply"),
            ),
            ("--pidfile",
            Flag::StrEntity(
                "pidfile", "",
                "Path of file to which to write process ID"),
            ),
            ("--watch",
            Flag::BoolEntity(
                "watch", false,
                "Reload changed config file automatically"),
            )]
        )
    }

    fn execute(&self) {

    }
}
