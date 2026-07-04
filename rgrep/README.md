# rgrep

A lightweight `grep`-like command-line search tool written in Rust. It searches files for matching text, supports case-insensitive searches, optional line numbers, and highlights matched results with colored terminal output.

---

## Project Structure

```text
rgrep/
├── src/
│   ├── config.rs       # command-line argument parsing
│   ├── rgrep.rs        # search implementation
│   ├── lib.rs          # library exports
│   └── main.rs         # application entry point
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## Requirements

- Rust (stable toolchain)
- Cargo

---

## Building

```bash
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

---

## Running

```bash
cargo run -- <query> <path> [options]
```

---

## Usage

```text
cargo run -- <query> <path> [options]
```

### Available Options

| Option                | Description                       |
| --------------------- | --------------------------------- |
| `-i`, `--ignore-case` | Perform a case-insensitive search |
| `-n`, `--line-number` | Display matching line numbers     |

---

## Examples

Search for a word:

```bash
cargo run -- hello notes.txt
```

Case-insensitive search:

```bash
cargo run -- hello notes.txt --ignore-case
```

Show line numbers:

```bash
cargo run -- hello notes.txt --line-number
```

Combine both options:

```bash
cargo run -- hello notes.txt -i -n
```

---

## Features

- Fast line-by-line file processing
- Literal text search using regular expressions
- Optional case-insensitive matching
- Optional line number display
- Colored highlighting of matched text
- Helpful error messages for invalid usage

---

## How It Works

1. Command-line arguments are parsed into a `Config` structure.
2. The target file is opened using a buffered reader.
3. A regular expression is built from the search query.
4. Each line is checked for matches.
5. Matching text is highlighted using colored terminal output.
6. Matching lines are printed, optionally with line numbers.
7. If no matches are found, an informative message is displayed.

---

## Example Output

Input file:

```text
Rust is fast.
Learning Rust is fun.
I enjoy programming.
```

Command:

```bash
cargo run -- Rust sample.txt -n
```

Output:

```text
1: Rust is fast.
2: Learning Rust is fun.
```

(The matching text is highlighted in red in the terminal.)

---

## Error Handling

The application reports common errors without crashing.

Examples include:

- Missing command-line arguments
- Invalid command syntax
- File not found
- Permission denied
- Invalid regular expression construction

Example:

```bash
cargo run

Argument error:
usage: rgrep <query> <path> [-i|--ignore-case] [-n|--line-number]
```

---

## Dependencies

| Crate     | Purpose                           |
| --------- | --------------------------------- |
| `regex`   | Pattern matching and highlighting |
| `colored` | Colored terminal output           |

---

## Limitations

- Searches a single file at a time.
- Performs literal text matching by escaping regex metacharacters.
- Does not recursively search directories.
- No support for glob patterns.
- No context lines (`-A`, `-B`, `-C`) or recursive search options.

---

*Built as part of my Rust learning journey to practice file I/O, buffered reading, regular expressions, CLI argument parsing, modular project organization, and terminal applications.*
