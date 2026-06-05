pub const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
pub const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const DIGITS: &str = "1234567890";
pub const SYMBOLS: &str = r#"~!@#$%^&*(){}[]-+|<>?"#;
pub const AMBIGUOUS: &str = "O0Il1";

#[derive(Debug)]
pub struct Config {
    pub length: usize,
    pub lowercase: bool,
    pub uppercase: bool,
    pub digits: bool,
    pub symbols: bool,
    pub exclude_ambiguous: bool,
    pub custom_symbols: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            length: 16,
            lowercase: true,
            uppercase: true,
            digits: true,
            symbols: true,
            exclude_ambiguous: false,
            custom_symbols: None,
        }
    }
}
