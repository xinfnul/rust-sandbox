use std::env;

use colored::Colorize;

use rgrep::{config::Config, rgrep::run};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("{} {}", format!("Argument error:").red(), error);
        std::process::exit(1);
    });

    #[cfg(debug_assertions)]
    println!("{}", format!("{config:#?}\n").bright_blue().bold());

    if let Err(e) = run(config) {
        eprintln!("{} {}", format!("Application error:").red(), e);
        std::process::exit(1);
    }
}
