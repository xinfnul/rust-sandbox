use std::{env, fs};

use log_core::{Analyzer, DefaultAnalyzer, LogResult, parse_log_content};

fn main() {
    if let Err(err) = run() {
        eprintln!("log-analyzer error: {err}");
        std::process::exit(1);
    }
}

fn run() -> LogResult<()> {
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "logs.txt".to_string());

    let content = fs::read_to_string(&input_path)?;

    let entries = parse_log_content(&content);

    if entries.is_empty() {
        println!(
            "No valid log entries found in '{input_path}'. \
                Did you run `cargo run -p fake-log-generator` first?"
        );
        return Ok(());
    }

    let analyzer = DefaultAnalyzer;
    let report = analyzer.analyze(&entries);

    println!("Analyzed {} lines from '{input_path}.\n'", entries.len());
    print!("{report}");

    Ok(())
}
