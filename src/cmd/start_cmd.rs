use super::commands::{Flag, SubCommandHelp};

pub struct StartCommand {
    cmd: String,
    flags: Vec<&'static str>,
    flagset: Vec<Flag>,
}

impl StartCommand {

    pub fn new() -> Self {
        StartCommand {
            cmd: "start".to_string(),
            flags: vec!["--config", "--envfile", "--adapter", "--pidfile", "--watch"],
            flagset: vec![],
        }
    }
}

impl  SubCommandHelp for StartCommand {

    fn execute(&self) {

    }
}
