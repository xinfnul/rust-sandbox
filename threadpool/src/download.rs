use std::{
    fs::File,
    io::{self, BufWriter},
    path::Path,
};

use reqwest::blocking::Client;

#[derive(Debug)]
pub enum DownloadError {
    Request(reqwest::Error),
    Io(io::Error),
    InvalidStatus {
        url: String,
        status: reqwest::StatusCode,
    },
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request(e) => write!(f, "Reqwest error: {}", e),
            Self::Io(e) => write!(f, "I/ O error: {}", e),
            Self::InvalidStatus { url, status } => {
                write!(f, "{} returned {}", url, status)
            }
        }
    }
}

impl std::error::Error for DownloadError {}

impl From<reqwest::Error> for DownloadError {
    #[inline]
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}

impl From<io::Error> for DownloadError {
    #[inline]
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

pub fn download_image(client: &Client, url: &str, output: &Path) -> Result<(), DownloadError> {
    let mut response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(DownloadError::InvalidStatus {
            url: url.to_string(),
            status: response.status(),
        });
    }

    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);

    std::io::copy(&mut response, &mut writer)?;

    Ok(())
}
