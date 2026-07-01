use std::{error::Error, fs};

use colored::Colorize;

use crate::{config::Config, highlight_query, search};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.path)?;

    if content.is_empty() {
        println!("{}", "File is empty.".yellow());
        return Ok(());
    }

    let results = search(&config.query, &content, config.ignore_case)?
        .into_iter()
        .map(|line| highlight_query(line, &config.query, config.ignore_case))
        .collect::<Result<Vec<_>, _>>()?;

    for line in results {
        println!("{line}");
    }

    Ok(())
}
