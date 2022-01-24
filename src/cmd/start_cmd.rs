use cmd::{
    Command,
    SubCommandHelp,
    Flag,
};

pub struct StartCommand {
    cmd: String,
    flags: Vec<String>,
}

impl StartCommand {

    fn new() -> Self {
        cmd: "start".to_string(),
        flags: vec!["--config", "--envfile", "--adapter", "--pidfile", "--watch"],
    }
}

impl StartCommand for SubCommandHelp {

    fn help(&self) {

    }

}
