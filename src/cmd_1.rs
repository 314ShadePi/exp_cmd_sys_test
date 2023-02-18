use serde::{Deserialize, Serialize};
use crate::command::Command;

// #[command("cmd")]
#[derive(Clone, Deserialize, Serialize)]
pub struct Cmd1 {
    arg1: String,
}

pub const CMD1: &str = "cmd";

impl Command for Cmd1 {
    type Err = ();

    fn run(&self) -> Result<(), Self::Err> {
        println!("cmd1 with arg: {}", self.arg1);
        Ok(())
    }

    fn new_cmd(params: Vec<String>) -> Result<Self, Self::Err> {
        Ok(Self {
            arg1: params[0].clone(),
        })
    }
}