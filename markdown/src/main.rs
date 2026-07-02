use std::{
    env, fmt, fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
enum Error {
    Io(io::Error),
    MissingArgument(&'static str),
    UnknownArgument(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "I/O error: {}", err),

            Error::MissingArgument(arg) => {
                write!(f, "missing required argument: {}", arg)
            }

            Error::UnknownArgument(arg) => {
                write!(f, "unknown argument: {}", arg)
            }
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

#[derive(Debug)]
enum Block {
    Heading { level: u8, content: Vec<Inline> },
    Paragraph(Vec<Inline>),
}

#[derive(Debug)]
enum Inline {
    Text(String),
    Bold(String),
    Italic(String),
    Code(String),
}

struct Markdown {
    blocks: Vec<Block>,
}

impl Markdown {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let source = fs::read_to_string(path)?;
        Ok(Self {
            blocks: Parser::parse(&source)?,
        })
    }

    pub fn to_html(&self) -> String {
        HtmlRenderer::render(&self.blocks)
    }

    pub fn write_html<P: AsRef<Path>>(&self, output: P) -> Result<()> {
        fs::write(output, self.to_html())?;
        Ok(())
    }
}

struct Parser;

impl Parser {
    pub fn parse(source: &str) -> Result<Vec<Block>> {
        let mut blocks = Vec::new();

        for line in source.lines() {
            let line = line.trim_end();

            if line.is_empty() {
                continue;
            }

            if let Some(text) = line.strip_prefix("### ") {
                blocks.push(Block::Heading {
                    level: 3,
                    content: Self::parse_inline(text),
                });
            } else if let Some(text) = line.strip_prefix("## ") {
                blocks.push(Block::Heading {
                    level: 2,
                    content: Self::parse_inline(text),
                });
            } else if let Some(text) = line.strip_prefix("# ") {
                blocks.push(Block::Heading {
                    level: 1,
                    content: Self::parse_inline(text),
                });
            } else {
                blocks.push(Block::Paragraph(Self::parse_inline(line)));
            }
        }

        Ok(blocks)
    }

    fn parse_inline(text: &str) -> Vec<Inline> {
        let chars: Vec<char> = text.chars().collect();
        let mut nodes = Vec::new();
        let mut plain = String::new();

        let mut i = 0;

        while i < chars.len() {
            // **bold**
            if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
                if !plain.is_empty() {
                    nodes.push(Inline::Text(std::mem::take(&mut plain)));
                }

                i += 2;
                let start = i;

                while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '*') {
                    i += 1;
                }

                let content: String = chars[start..i].iter().collect();
                nodes.push(Inline::Bold(content));

                i += 2;
            }
            // *italic*
            else if chars[i] == '*' {
                if !plain.is_empty() {
                    nodes.push(Inline::Text(std::mem::take(&mut plain)));
                }

                i += 1;
                let start = i;

                while i < chars.len() && chars[i] != '*' {
                    i += 1;
                }

                let content: String = chars[start..i].iter().collect();
                nodes.push(Inline::Italic(content));

                i += 1;
            }
            // `code`
            else if chars[i] == '`' {
                if !plain.is_empty() {
                    nodes.push(Inline::Text(std::mem::take(&mut plain)));
                }

                i += 1;
                let start = i;

                while i < chars.len() && chars[i] != '`' {
                    i += 1;
                }

                let content: String = chars[start..i].iter().collect();
                nodes.push(Inline::Code(content));

                i += 1;
            } else {
                plain.push(chars[i]);
                i += 1;
            }
        }

        if !plain.is_empty() {
            nodes.push(Inline::Text(plain));
        }

        nodes
    }
}

struct HtmlRenderer;

impl HtmlRenderer {
    pub fn render(blocks: &[Block]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<title>Rust Markdown Output</title>\n");
        html.push_str("</head>\n<body>\n");

        for block in blocks {
            match block {
                Block::Heading { level, content } => {
                    html.push_str(&format!("<h{}>", level));
                    Self::render_inline(content, &mut html);
                    html.push_str(&format!("</h{}>", level));
                }

                Block::Paragraph(content) => {
                    html.push_str("<p>");
                    Self::render_inline(content, &mut html);
                    html.push_str("</p>");
                }
            }
        }

        html.push_str("</body>\n</html>\n");

        html
    }

    fn escape_html(text: &str) -> String {
        let mut escaped = String::new();

        for ch in text.chars() {
            match ch {
                '&' => escaped.push_str("&amp;"),
                '<' => escaped.push_str("&lt;"),
                '>' => escaped.push_str("&gt;"),
                '"' => escaped.push_str("&quot;"),
                '\'' => escaped.push_str("&#39;"),
                _ => escaped.push(ch),
            }
        }
        escaped
    }

    fn render_inline(nodes: &[Inline], html: &mut String) {
        for node in nodes {
            match node {
                Inline::Text(text) => html.push_str(&Self::escape_html(text)),

                Inline::Bold(text) => {
                    html.push_str("<strong>");
                    html.push_str(&Self::escape_html(text));
                    html.push_str("</strong>");
                }

                Inline::Italic(text) => {
                    html.push_str("<em>");
                    html.push_str(&Self::escape_html(text));
                    html.push_str("</em>");
                }
                Inline::Code(text) => {
                    html.push_str("<code>");
                    html.push_str(&Self::escape_html(text));
                    html.push_str("</code>");
                }
            }
        }
    }
}

struct Config {
    input: PathBuf,
    output: PathBuf,
}

impl Config {
    fn parse() -> Result<Self> {
        let mut args = env::args().skip(1);

        let mut input = None;
        let mut output = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-i" | "--input" => {
                    let path = args
                        .next()
                        .ok_or(Error::MissingArgument("-i/--input value"))?;
                    input = Some(PathBuf::from(path));
                }

                "-o" | "--output" => {
                    let path = args
                        .next()
                        .ok_or(Error::MissingArgument("-o/--output value"))?;
                    output = Some(PathBuf::from(path));
                }

                "-h" | "--help" => {
                    print_usage();
                    std::process::exit(0);
                }

                _ => return Err(Error::UnknownArgument(arg)),
            }
        }

        Ok(Self {
            input: input.ok_or(Error::MissingArgument("-i/--input value"))?,
            output: output.ok_or(Error::MissingArgument("-o/--output value"))?,
        })
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  md -i <input.md> -o <output.html>");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -i, --input <FILE>     Input markdown file");
    eprintln!("  -o, --output <FILE>    Output HTML file");
    eprintln!("  -h, --help             Show this help message");
}

fn main() -> Result<()> {
    let config = match Config::parse() {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    };

    let markdown = Markdown::from_file(config.input)?;
    markdown.write_html(config.output)?;

    Ok(())
}

// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn parse_heading() {
        let blocks = Parser::parse("# Rust").unwrap();

        assert_eq!(blocks.len(), 1);

        match &blocks[0] {
            Block::Heading { level, content } => {
                assert_eq!(*level, 1);
                assert!(matches!(&content[0], Inline::Text(t) if t == "Rust"))
            }

            _ => panic!("expected heading"),
        }
    }

    #[test]
    fn parse_paragraph() {
        let blocks = Parser::parse("Rust").unwrap();

        assert_eq!(blocks.len(), 1);

        match &blocks[0] {
            Block::Paragraph(content) => {
                assert!(matches!(&content[0], Inline::Text(t) if t == "Rust"))
            }

            _ => panic!("expected paragraph"),
        }
    }

    #[test]
    fn parse_bold() {
        let nodes = Parser::parse_inline("xyz **rust** c");

        assert_eq!(nodes.len(), 3);

        assert!(matches!(&nodes[0], Inline::Text(t) if t == "xyz "));
        assert!(matches!(&nodes[1], Inline::Bold(t) if t == "rust"));
        assert!(matches!(&nodes[2], Inline::Text(t) if t == " c"));
    }

    #[test]
    fn parse_italic() {
        let nodes = Parser::parse_inline("xyz *rust* c");

        assert_eq!(nodes.len(), 3);

        assert!(matches!(&nodes[0], Inline::Text(t) if t == "xyz "));
        assert!(matches!(&nodes[1], Inline::Italic(t) if t == "rust"));
        assert!(matches!(&nodes[2], Inline::Text(t) if t == " c"));
    }

    #[test]
    fn parse_code() {
        let nodes = Parser::parse_inline("xyz `rust` c");

        assert_eq!(nodes.len(), 3);

        assert!(matches!(&nodes[0], Inline::Text(t) if t == "xyz "));
        assert!(matches!(&nodes[1], Inline::Code(t) if t == "rust"));
        assert!(matches!(&nodes[2], Inline::Text(t) if t == " c"));
    }

    #[test]
    fn render_html() {
        let markdown = Markdown {
            blocks: Parser::parse("# Rust\nSystem **PC**").unwrap(),
        };

        let html = markdown.to_html();

        assert!(html.contains("<h1>Rust</h1>"));
        assert!(html.contains("<p>System <strong>PC</strong></p>"));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn render_escapes() {
        let markdown = Markdown {
            blocks: Parser::parse("**<Rust & HTML>**").unwrap(),
        };

        let html = markdown.to_html();

        assert!(html.contains("<strong>&lt;Rust &amp; HTML&gt;</strong>"))
    }

    #[test]
    fn markdown_from_file() {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "md_test_{}.md",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        fs::write(&path, "# RUst").unwrap();

        let markdown = Markdown::from_file(&path).unwrap();

        assert!(markdown.to_html().contains("<h1>RUst</h1>"));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn write_html_creates_file() {
        let markdown = Markdown {
            blocks: Parser::parse("# Rust").unwrap(),
        };

        let mut path = std::env::temp_dir();
        path.push(format!(
            "md_test_{}.html",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        markdown.write_html(&path).unwrap();

        let html = fs::read_to_string(&path).unwrap();

        assert!(html.contains("<h1>Rust</h1>"));

        fs::remove_file(path).unwrap();
    }
}
