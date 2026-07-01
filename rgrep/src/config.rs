use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub query: String,
    pub ignore_case: bool,
    pub line_numbers: bool,
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidArguments,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArguments => write!(
                f,
                "usage: rgrep <query> <path> [-i|--ignore-case] [-n|--line-number]"
            ),
        }
    }
}

impl Error for ConfigError {}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Self, ConfigError> {
        let mut ignore_case = false;
        let mut line_numbers = false;

        let mut positional = Vec::new();

        for arg in args.skip(1) {
            match arg.as_str() {
                "-i" | "--ignore-case" => ignore_case = true,
                "-n" | "--line-number" => line_numbers = true,
                _ => positional.push(arg),
            }
        }

        if positional.len() != 2 {
            return Err(ConfigError::InvalidArguments);
        }

        Ok(Self {
            query: positional.remove(0),
            path: positional.remove(0),
            ignore_case,
            line_numbers,
        })
    }
}
