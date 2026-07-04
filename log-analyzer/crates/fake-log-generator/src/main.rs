use std::{env, fs};

use chrono::{Duration, Local};
use log_core::{Generator, LogEntry, LogLevel, LogResult};
use rand::RngExt;

const INFO_MESSAGES: [&str; 5] = [
    "Service started successfully",
    "User logged in",
    "Cache refreshed",
    "Scheduled job completed",
    "Configuration reloaded",
];

const WARN_MESSAGES: [&str; 5] = [
    "High memory usage detected",
    "Retrying failed connection",
    "Deprecated API called",
    "Slow query detected",
    "Disk space running low",
];

const ERROR_MESSAGES: [&str; 5] = [
    "Database connection failed",
    "Unhandled exception occurred",
    "Request timed out",
    "Failed to write to disk",
    "Authentication failed",
];

const DEBUG_MESSAGES: [&str; 5] = [
    "Entering function process_request",
    "Variable state dumped",
    "Cache lookup miss",
    "Received payload of size 512 bytes",
    "Exiting function process_request",
];

fn message_for(level: LogLevel) -> &'static [&'static str; 5] {
    match level {
        LogLevel::Debug => &DEBUG_MESSAGES,
        LogLevel::Info => &INFO_MESSAGES,
        LogLevel::Warn => &WARN_MESSAGES,
        LogLevel::Error => &ERROR_MESSAGES,
    }
}

struct FakeLogGenerator {
    clock: chrono::DateTime<Local>,
}

impl FakeLogGenerator {
    fn new() -> Self {
        Self {
            clock: Local::now(),
        }
    }

    fn generate_one(&mut self) -> LogEntry {
        let mut rng = rand::rng();

        let delta = Duration::seconds(rng.random_range(10_000..=11_400));
        self.clock -= delta;

        let level = LogLevel::ALL[rng.random_range(0..LogLevel::ALL.len())];
        let pool = message_for(level);
        let message = pool[rng.random_range(0..pool.len())];

        let timestamp = self.clock.format("%Y-%m-%dT%H:%M:%S").to_string();

        LogEntry::new(timestamp, level, message)
    }
}

impl Generator for FakeLogGenerator {
    fn generate(&mut self, count: usize) -> Vec<LogEntry> {
        (0..count).map(|_| self.generate_one()).collect()
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("fake-log-generator error: {err}");
        std::process::exit(1);
    }
}

fn run() -> LogResult<()> {
    let args: Vec<String> = env::args().collect();

    let count: usize = args
        .get(1)
        .and_then(|value| value.parse().ok())
        .unwrap_or(200);

    let output_path = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "logs.txt".to_string());

    let mut generator = FakeLogGenerator::new();
    let entries = generator.generate(count);

    let content = entries
        .iter()
        .map(LogEntry::to_line)
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(&output_path, content)?;

    println!("Generated {count} fake log entries into '{output_path}'.");

    Ok(())
}
