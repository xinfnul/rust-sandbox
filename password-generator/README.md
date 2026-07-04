# password-generator

A configurable command-line password generator written in Rust. It creates secure random passwords with customizable character sets, length, and ambiguity filtering while ensuring at least one character from each enabled category is included.

---

## Project Structure

```text
password-generator/
├── src/
│   ├── charset.rs      # Builds character sets from configuration
│   ├── cli.rs          # Command-line argument parsing
│   ├── config.rs       # Configuration and constants
│   ├── generator.rs    # Password generation logic
│   ├── lib.rs          # Library exports
│   └── main.rs         # Application entry point
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

Generate a password with the default settings:

```bash
cargo run
```

Example output:

```text
Password: hA8!mQ2@rX7$LpN9
```

---

## Usage

```text
cargo run -- [options]
```

### Available Options

| Option | Description | Default |
| ------ | ----------- | ------- |
| `--length <n>` | Password length | `16` |
| `--no-lower` | Disable lowercase letters | Enabled |
| `--no-upper` | Disable uppercase letters | Enabled |
| `--no-digits` | Disable digits | Enabled |
| `--no-symbols` | Disable symbols | Enabled |
| `--exclude-ambiguous` | Remove confusing characters (`O`, `0`, `I`, `l`, `1`) | Disabled |
| `--symbols <chars>` | Use a custom symbol set | Default symbols |
| `--help` | Show help message | |

---

## Examples

Generate a 24-character password:

```bash
cargo run -- --length 24
```

Generate without symbols:

```bash
cargo run -- --no-symbols
```

Generate using only letters:

```bash
cargo run -- --no-symbols --no-digits
```

Exclude visually similar characters:

```bash
cargo run -- --exclude-ambiguous
```

Use custom symbols:

```bash
cargo run -- --symbols "@#$%"
```

---

## How It Works

1. Command-line arguments are parsed into a `Config` structure.
2. Character sets are built according to the selected options.
3. One random character is chosen from every enabled character set.
4. Remaining characters are selected randomly from the combined pool.
5. The password is shuffled to randomize character positions.
6. The generated password is printed to the terminal.

This guarantees that every enabled character category appears at least once in the final password.

---

## Configuration

Default configuration:

| Setting | Value |
| ---------------------------- | -------: |
| Length | `16` |
| Lowercase | Enabled |
| Uppercase | Enabled |
| Digits | Enabled |
| Symbols | Enabled |
| Exclude ambiguous characters | Disabled |

Default character sets:

```text
Lowercase : abcdefghijklmnopqrstuvwxyz
Uppercase : ABCDEFGHIJKLMNOPQRSTUVWXYZ
Digits    : 0123456789
Symbols   : ~!@#$%^&*(){}[]-+|<>?
```

---

## Error Handling

The application validates user input before generating passwords.

Possible errors include:

- Invalid password length
- Missing command-line argument values
- Unknown command-line options
- No enabled character sets
- Empty custom character sets

Example:

```bash
cargo run -- --length 4

Error: Length must be between 8 and 32
```

---

## Testing

Run the test suite:

```bash
cargo test
```

Tests cover:

- CLI argument parsing
- Invalid argument handling
- Password length validation
- Character set validation
- Password generation

---

## Dependencies

| Crate | Purpose |
| ------ | ------- |
| `rand` | Cryptographically secure random password generation |

---

## Limitations

- Generates passwords only through the command line.
- Passwords are printed to standard output and are not copied to the clipboard.
- No password strength estimation.
- No password history or storage.
- No configuration file support.

---

*Built as part of my Rust learning journey to practice modules, CLI parsing, configuration management, random number generation, testing, and building reusable libraries.*