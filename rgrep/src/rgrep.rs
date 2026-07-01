use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use colored::Colorize;
use regex::RegexBuilder;

use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.path)?;
    let reader = BufReader::new(file);

    let regex = RegexBuilder::new(&regex::escape(&config.query))
        .case_insensitive(config.ignore_case)
        .build()?;

    let mut found = false;

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;

        if regex.is_match(&line) {
            found = true;

            let highlighted = regex
                .replace_all(&line, |caps: &regex::Captures| {
                    caps[0].red().bold().to_string()
                })
                .into_owned();

            if config.line_numbers {
                println!(
                    "{}:{}",
                    (line_number + 1).to_string().bright_blue().bold(),
                    highlighted
                );
            } else {
                println!("{highlighted}");
            }
        }
    }

    if !found {
        println!("{}", "No matches found".yellow());
    }

    Ok(())
}
