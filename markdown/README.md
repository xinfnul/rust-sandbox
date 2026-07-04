# markdown

A lightweight command-line Markdown to HTML converter written in Rust. It parses a subset of Markdown syntax and generates a complete HTML document while safely escaping HTML characters.

---

## Project Structure

```text
markdown-to-html/
├── src/
│   └── main.rs         # Parser, renderer, CLI, configuration, and tests
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## Features

- Parse Markdown headings (`#`, `##`, `###`)
- Parse paragraphs
- Support **bold** text
- Support _italic_ text
- Support `inline code`
- Escape HTML special characters
- Generate complete HTML documents
- Command-line interface
- Unit tests

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

Convert a Markdown file into HTML:

```bash
cargo run -- -i input.md -o output.html
```

Example:

```bash
cargo run -- -i README.md -o README.html
```

---

## Usage

```text
cargo run -- -i <input.md> -o <output.html>
```

### Available Options

| Option                  | Description         |
| ----------------------- | ------------------- |
| `-i`, `--input <FILE>`  | Input Markdown file |
| `-o`, `--output <FILE>` | Output HTML file    |
| `-h`, `--help`          | Show help message   |

---

## Examples

Convert a Markdown file:

```bash
cargo run -- -i notes.md -o notes.html
```

Display help:

```bash
cargo run -- --help
```

---

## Supported Markdown Syntax

| Markdown      | HTML Output |
| ------------- | ----------- |
| `# Heading`   | `<h1>`      |
| `## Heading`  | `<h2>`      |
| `### Heading` | `<h3>`      |
| `**bold**`    | `<strong>`  |
| `*italic*`    | `<em>`      |
| `` `code` ``  | `<code>`    |
| Plain text    | `<p>`       |

---

## Example

Input:

```markdown
# Rust Markdown

This is **bold**, _italic_, and `code`.
```

Output:

```html
<!DOCTYPE html>
<html>
	<head>
		<meta charset="UTF-8" />
		<title>Rust Markdown Output</title>
	</head>
	<body>
		<h1>Rust Markdown</h1>
		<p>
			This is <strong>bold</strong>, <em>italic</em>, and <code>code</code>.
		</p>
	</body>
</html>
```

---

## How It Works

1. Parse command-line arguments.
2. Read the input Markdown file.
3. Parse Markdown into an internal Abstract Syntax Tree (AST).
4. Convert each block into HTML.
5. Escape HTML special characters.
6. Write the generated HTML to the output file.

---

## Error Handling

The application validates command-line arguments before processing.

Possible errors include:

- Missing input file
- Missing output file
- Unknown command-line option
- File I/O errors

Example:

```bash
cargo run -- -i README.md

Error: missing required argument: -o/--output value
```

---

## Testing

Run the test suite:

```bash
cargo test
```

Tests cover:

- Heading parsing
- Paragraph parsing
- Bold parsing
- Italic parsing
- Inline code parsing
- HTML rendering
- HTML escaping
- Reading Markdown files
- Writing HTML files

---

## Limitations

- Supports only headings (`#`, `##`, `###`)
- No unordered or ordered lists
- No blockquotes
- No fenced code blocks
- No links or images
- No tables
- No nested formatting
- Single-file implementation

---

*Built as part of my Rust learning journey to practice parsing, abstract syntax trees (ASTs), HTML rendering, command-line interfaces, file handling, error management, and unit testing.*
