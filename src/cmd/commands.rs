use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Flag {
    StrEntry{_name: String, _value: String, _desc: String},
    BoolEntry{_name: String, _value: bool, _desc: String},
}

pub struct Command<'a> {
    args:  &'a Vec<String>,
    flags: HashMap<String, Flag>,
}

pub type ValidFlagFn = fn(flag: &Flag) -> Result<i32, i32>;

pub trait SubCommandHelp{
    fn execute(&mut self, cmd_flags: &mut HashMap<String, Flag>);
    fn get_name(&self)-> &String;
    fn get_flags(&self)-> Option<&Vec<String>>;
}


impl<'a> Command<'a> {

    pub fn new(args: &'a Vec<String>) -> Self {
        Command{
            args: args,
            flags: HashMap::from([
                ("--config".to_string(),
                 Flag::StrEntry{
                   _name: "config".to_string(),
                   _value: "".to_string(),
                   _desc: "Configuration file".to_string()
                }),
                ("--envfile".to_string(),
                 Flag::StrEntry{
                  _name:  "envfile".to_string(),
                  _value: "".to_string(),
                  _desc:  "Environment file to load".to_string()
                }),
                ("--adapter".to_string(),
                 Flag::StrEntry{
                  _name:  "adapter".to_string(),
                  _value: "".to_string(),
                  _desc:  "Name of config adapter to apply".to_string()
                 }),
                ("--pidfile".to_string(),
                Flag::StrEntry{
                   _name:  "pidfile".to_string(),
                   _value: "".to_string(),
                   _desc:  "Path of file to which to write process ID".to_string()
                }),
                ("--pingback".to_string(),
                Flag::StrEntry{
                   _name:  "pingback".to_string(),
                   _value: "".to_string(),
                   _desc:  "Echo confirmation bytes to this address on success".to_string()
                }),
                ("--watch".to_string(),
                Flag::BoolEntry{
                    _name: "watch".to_string(),
                    _value: false,
                    _desc: "Reload changed config file automatically".to_string()}),
                ("--resume".to_string(),
                Flag::BoolEntry{
                    _name: "resume".to_string(),
                    _value: false,
                    _desc: "Use saved config, if any(and prefer over --config file)".to_string()}),
                ("--environ".to_string(),
                Flag::BoolEntry{
                    _name: "environ".to_string(),
                    _value: false,
                    _desc: "Print environment".to_string()}),
                ]
            )
        }
    }

    fn parse_flags(&mut self) -> Result<Vec<String>, i32>{
        let opts_in_line = &self.args[2..].join(" ");
        let re = Regex::new(r"(--\w+\s*[a-zA-Z0-9.:]*)").unwrap();
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
        flags:&Vec<String>) -> Result<i32, i32> {
        let mut cache_flags: Vec<(&str, Option<&str>)> = vec![];
        for in_opt in in_opts {
            let dict = in_opt.split(' ').collect::<Vec<&str>>();
            match dict.len() {
                1 => {
                    let opt = dict.get(0).unwrap();
                    if !flags.contains(&opt.to_string()) || 
                        !self.flags.contains_key(&opt.to_string()) {
                        println!("Invalid option {} in command line", opt);
                        return Err(-1);
                    }
                    cache_flags.push((*opt, None));
                },
                2 => {
                    let (opt, val) = (dict.get(0).unwrap(),
                                    dict.get(1).unwrap());
                    if !flags.contains(&opt.to_string()) || 
                        !self.flags.contains_key(&opt.to_string()){
                        println!("Invalid option {} in command line", opt);
                        return Err(-1);
                    }
                    cache_flags.push((*opt, Some(*val)));
                },
                _ => {
                }
            }
        }
        self.filter_and_assigned_opts(&cache_flags);
        Ok(0)
    }

    fn filter_and_assigned_opts(&mut self, _flags: &Vec<(&str, Option<&str>)>) {
        for flag in _flags {
            let (opt, val) = flag ;
            if let Some(entity) = self.flags.get_mut(&opt.to_string()) {
                match entity{
                    Flag::StrEntry{_name, _value, _desc} => {
                        if let Some(_in_val) = val {
                            *_value = String::from(*_in_val);
                        }
                    },
                    Flag::BoolEntry{_name, _value, _desc} => {
                        *_value = true;
                        println!("{:?}", entity);
                    },
                }
            }
        }
    }

    pub fn execute<T>(&mut self, cmd: &mut T) where T: SubCommandHelp {
        let opts = self.parse_flags();
        if let Ok(in_opts) = opts {
            if let Some(flags) = cmd.get_flags() {
                match self.set_cmd_flags(&in_opts, flags) {
                    Err(-1) => {
                        self.help(cmd);
                    },
                    Ok(0) => {
                        cmd.execute(&mut self.flags);
                    },
                    _ => {
                    }
                }
            } else {
                self.help(cmd);
            }
        } else {
            self.help(cmd);
        }
    }

    pub fn help<T>(&self, cmd: &T) where T: SubCommandHelp {
        println!("Usage of subcommand {}: ", cmd.get_name());
        if let Some(flags) = cmd.get_flags() {
            for flag in flags {
                match self.flags.get(flag).unwrap() {
                    Flag::StrEntry{_name, _value, _desc} => {
                        println!("  {:16}{}", flag, _desc);    
                    }
                    Flag::BoolEntry{_name, _value, _desc} => {
                        println!("  {:16}{}", flag, _desc);    
                    }
                }
            }
        }
    }
}
