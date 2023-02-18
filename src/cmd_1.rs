use crate::command::Command;
use serde::{Deserialize, Serialize};

// #[command(name = "cmd")]
#[derive(Clone, Deserialize, Serialize)]
pub struct Cmd1 {
    arg1: String,
}

impl Command for Cmd1 {
    type Err = ();
    const CMD: &'static str = "cmd";

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
