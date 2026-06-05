use std::env;

use crate::config::Config;

pub fn print_help() {
    println!(
        r#"
  Password Generator

  Usage:
    cargo run -- [options]

  Options:
    --length <n>          Password length (default: 16)
    --no-lower            Disable lowercase
    --no-upper            Disable uppercase
    --no-digits           Disable digits
    --no-symbols          Disable symbols
    --exclude-ambiguous   Remove confusing chars (O0Il1)
    --symbols <chars>     Custom symbol set
    --help  or --h        Show help
  "#
    );
}

pub fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    parse_args_from(args)
}

fn parse_args_from(args: Vec<String>) -> Result<Config, String> {
    let mut config = Config::default();

    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--length" => {
                i += 1;

                if i >= args.len() {
                    return Err("Missing value for --length".into());
                }

                config.length = args[i].parse::<usize>().map_err(|_| "Invalid length")?;
            }

            "--no-lower" => config.lowercase = false,
            "--no-upper" => config.uppercase = false,
            "--no-digits" => config.digits = false,
            "--no-symbols" => config.symbols = false,

            "--exclude-ambiguous" => config.exclude_ambiguous = true,

            "--symbols" => {
                i += 1;

                if i >= args.len() {
                    return Err("Missing value for --symbols".into());
                }

                config.custom_symbols = Some(args[i].clone());
            }

            "--help" | "--h" => {
                print_help();
                std::process::exit(0);
            }

            other => {
                return Err(format!("Unknown argument: {}", other));
            }
        };
        i += 1;
    }

    Ok(config)
}

// :::: TESTS ::::
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_length_correctly() {
        let args = vec!["app".to_string(), "--length".to_string(), "24".to_string()];

        let config = parse_args_from(args).unwrap();

        assert_eq!(config.length, 24);
    }

    #[test]
    fn returns_error_when_length_missing() {
        let args = vec!["app".to_string(), "--length".to_string()];

        let result = parse_args_from(args);

        assert!(result.is_err());
    }

    #[test]
    fn disables_lowercase() {
        let args = vec!["app".to_string(), "--no-lower".to_string()];

        let config = parse_args_from(args).unwrap();

        assert!(!config.lowercase);
    }

    #[test]
    fn returns_error_for_unknown_argument() {
        let args = vec!["app".to_string(), "--unknown".to_string()];

        let result = parse_args_from(args);

        assert!(result.is_err());
    }
}
