use serde::{Deserialize, Serialize};
use crate::command::Command;

// #[command("cmd2")]
#[derive(Clone, Deserialize, Serialize)]
pub struct Cmd2 {
    arg1: String,
    arg2: String,
}

pub const CMD2: &str = "cmd2";

impl Command for Cmd2 {
    type Err = ();

    fn run(&self) -> Result<(), Self::Err> {
        println!("cmd2 with args: {}, {}", self.arg1, self.arg2);
        Ok(())
    }

    fn new_cmd(params: Vec<String>) -> Result<Self, Self::Err> {
        Ok(Self {
            arg1: params[0].clone(),
            arg2: params[1].clone(),
        })
    }
}