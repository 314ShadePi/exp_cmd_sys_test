use inquire::validator::Validation;
use inquire::CustomUserError;
use serde::{Deserialize, Serialize};

pub trait Commands: Clone + for<'a> Deserialize<'a> + Serialize {
    type Err;

    fn run(&self) -> Result<(), Self::Err>;
    fn validate(input: &str) -> Result<Validation, CustomUserError>;
    fn command_line();
    fn from_cl(s: &str) -> Result<Self, Self::Err>;
}
