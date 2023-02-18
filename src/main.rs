use crate::cmd_1::Cmd1;
use crate::cmd_2::Cmd2;
use crate::command::Command;
use crate::commands::Commands;
use inquire::validator::Validation;
use inquire::{CustomUserError, Text};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

mod cmd_1;
mod cmd_2;
mod command;
mod commands;

#[derive(Clone, Deserialize, Serialize /*, Commands*/)]
enum CommandLine {
    Cmd1(Cmd1),
    Cmd2(Cmd2),
}

impl Commands for CommandLine {
    type Err = ();

    fn run(&self) -> Result<(), Self::Err> {
        match self {
            Self::Cmd1(cmd) => cmd.run(),
            Self::Cmd2(cmd) => cmd.run(),
        }
    }

    fn validate(input: &str) -> Result<Validation, CustomUserError> {
        let s: (&str, &str) = match input.contains(" ") {
            true => match input.split_once(' ') {
                None => return Ok(Validation::Invalid("Couldn't parse command.".into())),
                Some(s) => s,
            },
            false => (input, ""),
        };

        if !["exit", Cmd1::CMD, Cmd2::CMD].contains(&s.0) {
            return Ok(Validation::Invalid("Not a command.".into()));
        }

        Ok(Validation::Valid)
    }

    fn command_line() {
        loop {
            let cmd = Text::new("exp_cmd_sys_test> ")
                .with_validator(Self::validate)
                .prompt();

            match cmd {
                Ok(s) => {
                    if s == "exit".to_string() {
                        return;
                    } else {
                        Self::from_cl(&s).unwrap().run().unwrap()
                    }
                }
                Err(_) => {}
            }
        }
    }

    fn from_cl(s: &str) -> Result<Self, Self::Err> {
        let s = match s.contains(" ") {
            true => match s.split_once(' ') {
                None => return Err(()),
                Some(s) => s,
            },
            false => (s, ""),
        };

        match s {
            (Cmd1::CMD, params) => Ok(Self::Cmd1(
                Cmd1::new_cmd(
                    params
                        .split(' ')
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|e| e.to_string())
                        .collect(),
                )
                .unwrap(),
            )),
            (Cmd2::CMD, params) => Ok(Self::Cmd2(
                Cmd2::new_cmd(
                    params
                        .split(' ')
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|e| e.to_string())
                        .collect(),
                )
                .unwrap(),
            )),
            _ => Err(()),
        }
    }
}

fn main() {
    CommandLine::from_cl("cmd test_arg").unwrap().run().unwrap();
    CommandLine::from_cl("cmd2 test_arg1 test_arg2")
        .unwrap()
        .run()
        .unwrap();

    CommandLine::command_line()
}
