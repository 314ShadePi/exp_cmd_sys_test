use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::cmd_1::Cmd1;
use crate::cmd_2::Cmd2;
use crate::command::Command;

mod cmd_1;
mod command;
mod cmd_2;

// commands!()
#[derive(Clone, Deserialize, Serialize)]
enum Commands {
    Cmd1(Cmd1),
    Cmd2(Cmd2),
}

impl Commands {
    pub fn run(&self) -> Result<(), ()> {
        match self {
            Commands::Cmd1(cmd) => {cmd.run()}
            Commands::Cmd2(cmd) => {cmd.run()}
        }
    }
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = match s.contains(" ") {
            true => match s.split_once(' ') {
                None => return Err(()),
                Some(s) => s,
            },
            false => (s, ""),
        };

        match s {
            (crate::cmd_1::CMD1, params) => {
                Ok(Self::Cmd1(Cmd1::new_cmd(params.split(' ').collect::<Vec<&str>>().iter().map(|e| e.to_string()).collect()).unwrap()))
            }
            (crate::cmd_2::CMD2, params) => {
                Ok(Self::Cmd2(Cmd2::new_cmd(params.split(' ').collect::<Vec<&str>>().iter().map(|e| e.to_string()).collect()).unwrap()))
            }
            _ => Err(())
        }
    }
}

fn main() {
    Commands::from_str("cmd test_arg").unwrap().run().unwrap();
    Commands::from_str("cmd2 test_arg1 test_arg2").unwrap().run().unwrap();
}
