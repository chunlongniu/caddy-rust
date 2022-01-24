use regex::Regex;
use std::collections::HashMap;

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
    fn get_name(&self)-> &String;
    fn get_flags(&self)-> Option<&Vec<String>>;
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

    fn parse_flags(&mut self) -> Result<Vec<String>, i32>{
        let opts_in_line = &self.args[2..].join(" ");
        let re = Regex::new(r"(--\w+\s*[a-zA-Z]*)").unwrap();
        let mut opts: Vec<String> = vec![];

        for cap in re.captures_iter(opts_in_line) {
            let opt = cap[1].trim_end();
            if opt.eq("--help") {
                return Err(-1);
            }
            opts.push(String::from(cap[1].trim_end()))
        }
        Ok(opts)
    }

    fn set_cmd_flags(&mut self, in_opts: &Vec<String>,
        flags:&Vec<String>) {

        let mut cache_flags: Vec<(&str, Option<&str>)> = vec![];
        for in_opt in in_opts {
            let dict = in_opt.split(' ').collect::<Vec<&str>>();
            match dict.len() {
                1 => {
                    let opt = dict.get(0).unwrap();
                    if !flags.contains(&opt.to_string()) {

                    }
                    cache_flags.push((*opt, None));
                },
                2 => {
                    let (opt, val) = (dict.get(0).unwrap(),
                                    dict.get(1).unwrap());
                    if !flags.contains(&opt.to_string()) {

                    }
                    cache_flags.push((*opt, Some(*val)));
                },
                _ => {
                }
            }
        }
        self.filter_and_assigned_opts(&cache_flags);
    }

    fn filter_and_assigned_opts(&mut self, _flags: &Vec<(&str, Option<&str>)>) {
        for flag in _flags {
            let (opt, val) = flag ;
            if let Some(entity) = self.flags.get_mut(&opt.to_string()) {
                match entity{
                    Flag::StrEntity(_opt, _val, _desc) => {
                        if let Some(_in_val) = val {
                            println!("{:?}", entity);
                        }
                    },
                    Flag::BoolEntity(_opt, _, _desc) => {
                        println!("{}", opt);
                    },
                }
            }
        }
    }

    pub fn execute<T>(&mut self, cmd: &mut T) where T: SubCommandHelp {
        let opts = self.parse_flags();
        if let Ok(in_opts) = opts {
            if let Some(flags) = cmd.get_flags() {
                self.set_cmd_flags(&in_opts, flags);
            } else {
                self.help(cmd);
            }
        } else {
            self.help(cmd);
        }
    }

    pub fn help<T>(&self, cmd: &T) where T: SubCommandHelp {
        println!("Usage of {}", cmd.get_name());
        if let Some(flags) = cmd.get_flags() {
            for flag in flags {
                match self.flags.get(flag).unwrap() {
                    Flag::StrEntity(_, _, desc) => {
                        println!("  {:16}{}", flag, desc);    
                    }
                    Flag::BoolEntity(_, _, desc) => {
                        println!("  {:16}{}", flag, desc);    
                    }
                }
            }
        }
    }
}
