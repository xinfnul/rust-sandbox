pub mod config;
pub mod rgrep;

use colored::Colorize;
use regex::RegexBuilder;

pub fn highlight_query(line: &str, query: &str, ignore_case: bool) -> Result<String, regex::Error> {
    let regex = RegexBuilder::new(&regex::escape(query))
        .case_insensitive(ignore_case)
        .build()?;

    Ok(regex
        .replace_all(line, |caps: &regex::Captures| {
            caps[0].red().bold().to_string()
        })
        .into_owned())
}

pub fn search<'a>(
    query: &str,
    content: &'a str,
    ignore_case: bool,
) -> Result<Vec<&'a str>, regex::Error> {
    let regex = RegexBuilder::new(&regex::escape(query))
        .case_insensitive(ignore_case)
        .build()?;

    Ok(content
        .lines()
        .filter(|line| regex.is_match(line))
        .collect())
}
