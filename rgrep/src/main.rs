use std::env;

use colored::Colorize;

use rgrep::{config::Config, rgrep::run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("{} {}", format!("Argument error: ").red(), error);
        std::process::exit(1);
    });

    #[cfg(debug_assertions)]
    println!("{}", format!("{config:#?}").bright_blue().bold());

    if let Err(e) = run(config) {
        eprintln!("{} {}", format!("Application error: ").red(), e);
        std::process::exit(1);
    }
}
