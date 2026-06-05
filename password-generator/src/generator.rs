use rand::{prelude::SliceRandom, seq::IndexedRandom};

use crate::charset::build_charset;
use crate::config::Config;

pub fn generate_password(config: &Config) -> Result<String, String> {
    let sets = build_charset(config);

    if sets.is_empty() {
        return Err("No character sets enabled".into());
    }

    if config.length < 8 || config.length > 32 {
        return Err("Length must be between 8 and 32".into());
    }

    let all_chars: Vec<char> = sets.iter().flat_map(|s| s.chars()).collect();

    let mut rng = rand::rng();
    let mut password: Vec<char> = Vec::new();

    for set in &sets {
        let chars: Vec<char> = set.chars().collect();

        let ch = chars
            .choose(&mut rng)
            .copied()
            .ok_or("Character set empty")?;

        password.push(ch);
    }

    while password.len() < config.length {
        let ch = all_chars
            .choose(&mut rng)
            .copied()
            .ok_or("No available characters")?;

        password.push(ch);
    }

    password.shuffle(&mut rng);

    Ok(password.into_iter().collect())
}

// :::: TESTS ::::
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_password_with_correct_length() {
        let config = Config::default();

        let password = generate_password(&config).unwrap();

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn returns_error_when_no_charset_enabled() {
        let config = Config {
            length: 16,
            uppercase: false,
            lowercase: false,
            digits: false,
            exclude_ambiguous: true,
            symbols: false,
            custom_symbols: None,
        };

        let result = generate_password(&config);

        assert!(result.is_err());
    }

    #[test]
    fn returns_error_for_invalid_length() {
        let config = Config {
            length: 4,
            uppercase: true,
            lowercase: true,
            digits: true,
            exclude_ambiguous: false,
            symbols: true,
            custom_symbols: None,
        };

        let result = generate_password(&config);

        assert!(result.is_err());
    }
}
