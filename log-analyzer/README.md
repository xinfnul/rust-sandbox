# log-analyzer

A Rust workspace for generating, parsing, and analyzing log files. The project is split into reusable crates, featuring a fake log generator, a shared core library, and a command-line log analyzer that produces summary reports.

---

## Workspace Structure

```text
log-analyzer/
├── crates/
│   ├── fake-log-generator/
│   │   └── src/
│   │       └── main.rs          # Generates fake log files
│   ├── log-analyzer/
│   │   └── src/
│   │       └── main.rs          # Reads and analyzes log files
│   └── log-core/
│       └── src/
│           └── lib.rs           # Shared models, parser, analyzer, traits, and tests
├── Cargo.toml                   # Workspace configuration
├── Cargo.lock
└── README.md
```

---

## Features

- Cargo workspace with multiple crates
- Shared reusable core library
- Generate realistic fake log files
- Parse structured log entries
- Analyze logs by severity
- Analyze logs by date
- Find the most common log message
- Gracefully skip malformed log entries
- Comprehensive unit tests

---

## Requirements

- Rust (stable toolchain)
- Cargo

---

## Building

Build the entire workspace:

```bash
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

Build a specific crate:

```bash
cargo build -p log-analyzer
cargo build -p fake-log-generator
cargo build -p log-core
```

---

## Running

Generate a log file:

```bash
cargo run -p fake-log-generator
```

Generate 500 log entries:

```bash
cargo run -p fake-log-generator -- 500
```

Generate 500 entries into a custom file:

```bash
cargo run -p fake-log-generator -- 500 server.log
```

Analyze the default log file:

```bash
cargo run -p log-analyzer
```

Analyze a custom log file:

```bash
cargo run -p log-analyzer -- server.log
```

---

## Usage

### Fake Log Generator

```text
cargo run -p fake-log-generator -- [count] [output-file]
```

| Argument | Description | Default |
| -------- | ----------- | ------- |
| `count` | Number of log entries to generate | `200` |
| `output-file` | Output log file | `logs.txt` |

### Log Analyzer

```text
cargo run -p log-analyzer -- [input-file]
```

| Argument | Description | Default |
| -------- | ----------- | ------- |
| `input-file` | Log file to analyze | `logs.txt` |

---

## Log Format

Each log entry is stored as:

```text
TIMESTAMP|LEVEL|MESSAGE
```

Example:

```text
2026-07-04T11:22:13|INFO|Service started successfully
2026-07-04T11:25:44|WARN|Disk space running low
2026-07-04T11:27:02|ERROR|Database connection failed
```

---

## Example Workflow

Generate fake logs:

```bash
cargo run -p fake-log-generator -- 300 logs.txt
```

Analyze them:

```bash
cargo run -p log-analyzer -- logs.txt
```

Example output:

```text
===== Log Analysis Report =====

Total logs: 300

-- Counts by level --
DEBUG: 74
INFO : 80
WARN : 69
ERROR: 77

-- Counts by date --
2026-07-01: 96
2026-07-02: 101
2026-07-03: 103

-- Most common message --
"Service started successfully" (18 occurrences)
```

---

## How It Works

1. The fake log generator creates random log entries.
2. Each entry is written using the format:

   ```text
   TIMESTAMP|LEVEL|MESSAGE
   ```

3. The analyzer reads the log file.
4. Valid log entries are parsed into `LogEntry` structures.
5. Invalid or malformed lines are ignored.
6. Statistics are computed, including:
   - Total log entries
   - Counts by log level
   - Counts by date
   - Most common message
7. A formatted report is printed to the terminal.

---

## Workspace Crates

### `log-core`

Shared library containing:

- `LogEntry`
- `LogLevel`
- Parser
- Analyzer
- Report
- Error types
- Traits (`Analyzer`, `Generator`)
- Unit tests

### `fake-log-generator`

Responsible for:

- Creating fake timestamps
- Generating random log levels
- Selecting realistic messages
- Writing logs to a file

### `log-analyzer`

Responsible for:

- Reading log files
- Parsing log entries
- Running the analyzer
- Printing formatted reports

---

## Error Handling

The application handles several common errors gracefully, including:

- Missing log file
- File I/O errors
- Invalid log levels
- Malformed log lines
- Empty or invalid log files

Example:

```bash
cargo run -p log-analyzer -- missing.log

log-analyzer error: I/O error: No such file or directory
```

If no valid log entries are found:

```text
No valid log entries found in 'logs.txt'.
Did you run `cargo run -p fake-log-generator` first?
```

---

## Testing

Run all workspace tests:

```bash
cargo test
```

Run tests for a specific crate:

```bash
cargo test -p log-core
```

Tests cover:

- Log parsing
- Invalid log handling
- Log level parsing
- Generic counting utilities
- Analyzer statistics
- Error formatting

---

## Dependencies

| Crate | Purpose |
| ------ | ------- |
| `chrono` | Timestamp generation and date handling |
| `rand` | Random log generation |

---

## Limitations

- Supports only the predefined log format.
- Malformed log entries are skipped instead of reported.
- Reports are printed to the terminal only.
- No filtering by date or log level.
- No CSV, JSON, or HTML report export.
- No live log monitoring.

---

*Built as part of my Rust learning journey to practice Cargo workspaces, reusable libraries, traits, parsing, error handling, generic programming, testing, and command-line application development.*