use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

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
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let source = fs::read_to_string(path)?;
        Ok(Self {
            blocks: Parser::parse(&source),
        })
    }

    pub fn to_html(&self) -> String {
        HtmlRenderer::render(&self.blocks)
    }

    pub fn write_html<P: AsRef<Path>>(&self, output: P) -> io::Result<()> {
        fs::write(output, self.to_html())
    }
}

struct Parser;

impl Parser {
    pub fn parse(source: &str) -> Vec<Block> {
        let mut blocks = Vec::new();

        for line in source.lines() {
            let line = line.trim_end();

            if line.is_empty() {
                continue;
            }

            if let Some(text) = line.strip_prefix("# ") {
                blocks.push(Block::Heading {
                    level: 1,
                    content: Self::parse_inline(text),
                });
            } else if let Some(text) = line.strip_prefix("## ") {
                blocks.push(Block::Heading {
                    level: 2,
                    content: Self::parse_inline(text),
                });
            } else if let Some(text) = line.strip_prefix("### ") {
                blocks.push(Block::Heading {
                    level: 3,
                    content: Self::parse_inline(text),
                });
            } else {
                blocks.push(Block::Paragraph(Self::parse_inline(line)));
            }
        }

        blocks
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

    fn render_inline(nodes: &[Inline], html: &mut String) {
        for node in nodes {
            match node {
                Inline::Text(text) => html.push_str(text),

                Inline::Bold(text) => {
                    html.push_str("<strong>");
                    html.push_str(text);
                    html.push_str("</strong>");
                }

                Inline::Italic(text) => {
                    html.push_str("<em>");
                    html.push_str(text);
                    html.push_str("</em>");
                }
                Inline::Code(text) => {
                    html.push_str("<code>");
                    html.push_str(text);
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
    fn parse() -> Result<Self, String> {
        let mut args = env::args().skip(1);

        let mut input = None;
        let mut output = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-i" | "--input" => {
                    let path = args.next().ok_or("expected a file after -i/--input")?;
                    input = Some(PathBuf::from(path));
                }

                "-o" | "--output" => {
                    let path = args.next().ok_or("expected a file after -o/--output")?;
                    output = Some(PathBuf::from(path));
                }

                "-h" | "--help" => {
                    print_usage();
                    std::process::exit(0);
                }

                _ => return Err(format!("unknown argument: {arg}")),
            }
        }

        Ok(Self {
            input: input.ok_or("missing required -i/--input")?,
            output: output.ok_or("missing required -o/--output")?,
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

fn main() -> io::Result<()> {
    let config = match Config::parse() {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error: {err}");
            print_usage();
            std::process::exit(1);
        }
    };

    let markdown = Markdown::from_file(config.input)?;
    markdown.write_html(config.output)?;

    Ok(())
}
