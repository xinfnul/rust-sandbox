use std::{collections::HashMap, fmt, hash::Hash, io, str::FromStr};

// LOG_LEVELS
// ----------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub const ALL: [LogLevel; 4] = [
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
    ];
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        };
        write!(f, "{text}")
    }
}

impl FromStr for LogLevel {
    type Err = LogError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEBUG" => Ok(LogLevel::Debug),
            "INFO" => Ok(LogLevel::Info),
            "WARN" => Ok(LogLevel::Warn),
            "ERROR" => Ok(LogLevel::Error),
            other => Err(LogError::InvalidLevel(other.to_string())),
        }
    }
}

// LOG_ENTRY
// ---------
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

impl LogEntry {
    pub fn new(timestamp: impl Into<String>, level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            timestamp: timestamp.into(),
            level,
            message: message.into(),
        }
    }

    pub fn date<'a>(&'a self) -> &'a str {
        self.timestamp.split('T').next().unwrap_or(&self.timestamp)
    }

    pub fn to_line(&self) -> String {
        format!("{}|{}|{}", self.timestamp, self.level, self.message)
    }
}

// ERRORS
// ------
#[derive(Debug)]
pub enum LogError {
    MalformedLine(String),
    InvalidLevel(String),
    Io(io::Error),
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogError::MalformedLine(line) => write!(f, "malformed log line: '{line}'"),
            LogError::InvalidLevel(level) => write!(f, "invalid log level: '{level}'"),
            LogError::Io(err) => write!(f, "I/O error: {err}"),
        }
    }
}

impl std::error::Error for LogError {}

impl From<io::Error> for LogError {
    fn from(err: io::Error) -> Self {
        LogError::Io(err)
    }
}

// ALIAS_TYPE_
pub type LogResult<T> = Result<T, LogError>;

// PARSING
// -------
pub fn parse_line(line: &str) -> LogResult<LogEntry> {
    let mut parts = line.splitn(3, '|');

    let Some(timestamp) = parts.next() else {
        return Err(LogError::MalformedLine(line.to_string()));
    };

    let Some(level_str) = parts.next() else {
        return Err(LogError::MalformedLine(line.to_string()));
    };

    let Some(message) = parts.next() else {
        return Err(LogError::MalformedLine(line.to_string()));
    };

    let level = level_str.parse::<LogLevel>()?;

    Ok(LogEntry::new(timestamp, level, message))
}

pub fn parse_log_content(content: &str) -> Vec<LogEntry> {
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| parse_line(line).ok())
        .collect()
}

// TRAITS
// ------
pub trait Analyzer {
    fn analyze(&self, entries: &[LogEntry]) -> Report;
}

pub trait Generator {
    fn generate(&mut self, count: usize) -> Vec<LogEntry>;
}

// STATISTICS/ REPORT
// ------------------
#[derive(Debug, Clone)]
pub struct Report {
    pub total: usize,
    pub by_level: HashMap<LogLevel, usize>,
    pub by_date: HashMap<String, usize>,
    pub most_common_message: Option<(String, usize)>,
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "===== Log Analysis Report =====")?;
        writeln!(f, "Total logs: {}", self.total)?;
        writeln!(f)?;

        writeln!(f, "-- Counts by level --")?;
        for level in LogLevel::ALL {
            let count = self.by_level.get(&level).copied().unwrap_or(0);
            writeln!(f, "  {level:<5}: {count}")?;
        }

        writeln!(f, "-- Counts by date --")?;
        let mut dates: Vec<_> = self.by_date.iter().collect();
        dates.sort_by(|a, b| a.0.cmp(b.0));
        for (date, count) in dates {
            writeln!(f, "  {date}: {count}")?;
        }
        writeln!(f)?;

        writeln!(f, "-- Most common message --")?;
        match &self.most_common_message {
            Some((message, count)) => writeln!(f, "  \"{message}\" ({count} occurrences)")?,
            None => writeln!(f, "  (no messages)")?,
        }

        Ok(())
    }
}

pub struct DefaultAnalyzer;

impl Analyzer for DefaultAnalyzer {
    fn analyze(&self, entries: &[LogEntry]) -> Report {
        let by_level = count_by(entries.iter(), |entry| entry.level);
        let by_date = count_by(entries.iter(), |entry| entry.date().to_string());

        let message_counts = count_by(entries.iter(), |entry| entry.message.clone());
        let most_common_message = message_counts
            .into_iter()
            .reduce(|a, b| if b.1 > a.1 { b } else { a });

        Report {
            total: entries.len(),
            by_level,
            by_date,
            most_common_message,
        }
    }
}

pub fn count_by<T, K, F>(iter: impl Iterator<Item = T>, key_fn: F) -> HashMap<K, usize>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut counts = HashMap::new();
    for item in iter {
        let key = key_fn(&item);
        *counts.entry(key).or_insert(0) += 1;
    }

    counts
}

// TESTS
// -----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_line() {
        let line = "2026-07-04T11:11:11|INFO|Service started successfully";
        let entry = parse_line(line).unwrap();

        assert_eq!(entry.timestamp, "2026-07-04T11:11:11");
        assert_eq!(entry.level, LogLevel::Info);
    }

    #[test]
    fn rejects_a_line_with_too_few_fields() {
        let line = "2026-07-04T10:15:30|INFO";
        let result = parse_line(line);
        assert!(matches!(result, Err(LogError::MalformedLine(_))));
    }

    #[test]
    fn rejects_an_invalid_level() {
        let line = "2026-07-04T10:15:30|WEIRD|Something happened";
        let result = parse_line(line);
        assert!(matches!(result, Err(LogError::InvalidLevel(_))));
    }

    #[test]
    fn parse_log_content_skips_bad_line() {
        let content = "\
    2026-07-04T11:11:11|INFO|First message
    not a valid line at all
    2026-07-04T12:12:12|ERROR|Second message
    ";
        let entries = parse_log_content(content);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].level, LogLevel::Info);
        assert_eq!(entries[1].level, LogLevel::Error);
    }

    #[test]
    fn count_by_groups_correctly() {
        let entries = vec![
            LogEntry::new("t1", LogLevel::Info, "a"),
            LogEntry::new("t2", LogLevel::Info, "b"),
            LogEntry::new("t3", LogLevel::Error, "c"),
        ];

        let counts = count_by(entries.iter(), |entry| entry.level);
        assert_eq!(counts.get(&LogLevel::Info), Some(&2));
        assert_eq!(counts.get(&LogLevel::Error), Some(&1));
        assert_eq!(counts.get(&LogLevel::Warn), None);
    }

    #[test]
    fn default_analyzer_produces_expected_totals() {
        let entries = vec![
            LogEntry::new("2026-07-04T11:00:00", LogLevel::Info, "started"),
            LogEntry::new("2026-07-04T12:05:00", LogLevel::Error, "failed"),
            LogEntry::new("2026-07-05T13:00:00", LogLevel::Info, "started"),
        ];

        let report = DefaultAnalyzer.analyze(&entries);
        assert_eq!(report.total, 3);
        assert_eq!(report.by_level.get(&LogLevel::Info), Some(&2));
        assert_eq!(report.by_date.get("2026-07-04"), Some(&2));
        assert_eq!(report.most_common_message, Some(("started".to_string(), 2)));
    }

    #[test]
    fn log_error_displays_helpful_message() {
        let err = LogError::InvalidLevel("WEIRD".to_string());
        assert_eq!(err.to_string(), "invalid log level: 'WEIRD'");
    }
}
