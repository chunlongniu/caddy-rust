use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;


lazy_static ! {
    pub static ref GLOBAL_FLAGS: HashMap<&'static str, Flag> = HashMap::from([
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
        )
    ]);
}


#[derive(Debug)]
pub enum Flag {
    StrEntity(&'static str, &'static str, &'static str),
    BoolEntity(&'static str, bool, &'static str),
}

pub trait SubCommandHelp {
    fn help(&self);
    fn execute(&self, args: &Vec<String>);
}

pub struct StartCommand {
    name: String,
    flags: Vec<&'static str>,
}


impl StartCommand {

    pub fn new() -> StartCommand {
        StartCommand {
            name: "start".to_string(),
            flags: vec!["--config", "--envfile", "--adapter", "--pidfile", "--watch"],
        }
    }

    pub fn assign_value_to_flags(&self, options: String) {
        let cmd_line_args = &options;
        let re = Regex::new(r"(--\w+\s*\w*)*").unwrap();
        for cap in re.captures_iter(cmd_line_args) {
            let rs = cap[1].split(' ').collect::<Vec<&str>>();
            match rs.len() {
                2 => {
                    if let Some(Flag::StrEntity(name, value, desc)) = GLOBAL_FLAGS.get(rs[0]) {

                    } else {
                        self.help();
                    }
                },
                1 => println!("{:?}", rs),
                _ => println!("help"),
            }
        }
    } 
}

mod common {
    pub fn parse_flags(flags: &Vec<&'static str>) {
        for flag in flags {
            match super::GLOBAL_FLAGS.get(flag).unwrap() {
                super::Flag::StrEntity(_, _, desc) => {
                    println!("  {:16}{}", flag, desc);
                },
                super::Flag::BoolEntity(_, _, desc) => {
                    println!("  {:16}{}", flag, desc);
                },
            }
        };
    }
}

impl SubCommandHelp for StartCommand {

    fn help(&self) {
        println!("Usage of {} ", self.name);
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
