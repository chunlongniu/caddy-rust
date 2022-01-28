use super::commands::{Flag, SubCommandHelp};
use std::collections::HashMap;

pub struct StartCommand {
    name: String,
    flags: Vec<String>,
    flagset: Vec<Flag>,
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
            flagset: vec![],
        }
    }
}

impl  SubCommandHelp for StartCommand {

    fn execute(&mut self, cmd_flags: &mut HashMap<String, Flag>) {

    }
    fn get_flags(&self) -> Option<&Vec<String>> {
        return Some(&self.flags);
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}
