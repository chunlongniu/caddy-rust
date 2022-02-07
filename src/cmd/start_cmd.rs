use super::commands::{Flag, ValidFlagFn, SubCommandHelp};
use std::collections::HashMap;
use std::net::{TcpListener};

pub mod valid {
    pub fn validate_config(flag: &super::Flag) -> Result<i32, i32> {
        Ok(0)
    }
}

pub struct StartCommand {
    name: String,
    flags: Vec<String>,
    valid_fns: Vec<ValidFlagFn>,
}

impl StartCommand {

    pub fn new() -> Self {
        StartCommand {
            name: "start".to_string(),
            flags: vec!["--config".to_string(),
                        "--envfile".to_string(),
                        "--adapter".to_string(),
                        "--pidfile".to_string(),
                        "--watch".to_string()],
            valid_fns: vec![valid::validate_config],
        }
    }

}

impl  SubCommandHelp for StartCommand {

    fn execute(&mut self, _cmd_flags: &mut HashMap<String, Flag>) {
        let listener = TcpListener::bind("127.0.0.1:0");
        match listener {
            Ok(ln) => {

            }
            Err(e) => {

            }
        }
    }

    fn get_flags(&self) -> Option<&Vec<String>> {
        return Some(&self.flags);
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}
