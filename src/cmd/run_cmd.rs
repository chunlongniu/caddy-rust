use super::commands:: {Flag, ValidFlagFn, SubCommandHelp};
use std::collections::HashMap;
use std::io::{Read};

pub struct RunCommand {
    name: String,
    flags: Vec<String>,
    valid_fns: ValidFlagFn,
}

pub mod valid {
    pub fn validate(_flag: &super::Flag) -> Result<i32, i32> {
        Ok(0)
    }
}

impl RunCommand {

    pub fn new() -> Self {
        RunCommand {
            name: "run".to_string(),
            flags: vec![
                "--config".to_string(),
                "--envfile".to_string(),
                "--adapter".to_string(),
                "--pidfile".to_string(),
                "--resume".to_string(),
                "--environ".to_string(),
                "--watch".to_string(),
                "--pingback".to_string(),
            ],
            valid_fns: valid::validate,
        }
    }

    fn read_stdin(&self, _cmd_flags: &HashMap<String, Flag>) {
        let mut buffer = String::new();
        let mut stdin = std::io::stdin();
        stdin.read_to_string(&mut buffer);
    }

    fn read_pinback_flag(&self, _cmd_flags: &HashMap<String, Flag>) {
        let entry = _cmd_flags.get("--pingback").unwrap(); 
        let mut ip_addr = "";
        if let Flag::StrEntry{_name, _value, _desc} = entry {
            ip_addr = &_value;
        }
        println!("=== {}", ip_addr);
    }

}

impl SubCommandHelp for RunCommand {

    fn execute(&mut self, _cmd_flags: &mut HashMap<String, Flag>) {
        self.read_pinback_flag(_cmd_flags);
    }

    fn get_flags(&self) -> Option<&Vec<String>> {
        return Some(&self.flags);
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}
