use crate::config::{AMBIGUOUS, Config};

fn remove_ambiguous(input: &str) -> String {
    input.chars().filter(|c| !AMBIGUOUS.contains(*c)).collect()
}

pub fn build_charset(config: &Config) -> Vec<String> {
    let mut sets = Vec::new();

    if config.lowercase {
        let s = if config.exclude_ambiguous {
            remove_ambiguous(crate::config::LOWER)
        } else {
            crate::config::LOWER.to_string()
        };

        sets.push(s);
    }

    if config.uppercase {
        let s = if config.exclude_ambiguous {
            remove_ambiguous(crate::config::UPPER)
        } else {
            crate::config::UPPER.to_string()
        };

        sets.push(s);
    }

    if config.digits {
        let s = if config.exclude_ambiguous {
            remove_ambiguous(crate::config::DIGITS)
        } else {
            crate::config::DIGITS.to_string()
        };

        sets.push(s);
    }

    if config.symbols {
        let symbols = config
            .custom_symbols
            .clone()
            .unwrap_or_else(|| crate::config::SYMBOLS.to_string());

        let s = if config.exclude_ambiguous {
            remove_ambiguous(&symbols)
        } else {
            symbols
        };

        sets.push(s);
    }

    sets
}
