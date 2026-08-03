use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use futures::{StreamExt, stream};
use reqwest::{Client, StatusCode};
use thiserror::Error;
use tokio::{fs, io::AsyncWriteExt};
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

const BASE_URL: &str = "https://yavuzceliker.github.io/sample-images";
const TOTAL_IMAGES: usize = 128;
const MAX_CONCURRENCY: usize = 4;
const MAX_RETRIES: u32 = 3;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const OUTPUT_DIR: &str = "images";

#[derive(Error, Debug)]
enum DownloadError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("server returned status {status} for {url}")]
    BadStatus { url: String, status: StatusCode },
}

async fn download_one(client: &Client, index: usize) -> Result<PathBuf, DownloadError> {
    let url = format!("{BASE_URL}/image-{index}.jpg");
    let dest = Path::new(OUTPUT_DIR).join(format!("image-{index}.jpg"));

    if fs::try_exists(&dest).await.unwrap_or(false) {
        info!(url = %url, "already present, skipping");
        return Ok(dest);
    }

    let mut last_err = None;

    for attempt in 1..=MAX_RETRIES {
        match try_download(client, &url, &dest).await {
            Ok(()) => {
                info!(url = %url, "saved {}", dest.display());
                return Ok(dest);
            }
            Err(e) => {
                let retryable = match &e {
                    DownloadError::BadStatus { status, .. } => {
                        status.is_server_error() || *status == StatusCode::TOO_MANY_REQUESTS
                    }
                    DownloadError::Request(_) | DownloadError::Io(_) => true,
                };

                if !retryable || attempt == MAX_RETRIES {
                    last_err = Some(e);
                    break;
                }

                let backoff = Duration::from_millis(300 * 2u64.pow(attempt - 1));

                warn!(url = %url, attempt, error = %e, ?backoff, "retrying");
                tokio::time::sleep(backoff).await;
                last_err = Some(e);
            }
        }
    }

    Err(last_err.expect("loop always sets an error before exiting without success"))
}

async fn try_download(client: &Client, url: &str, dest: &Path) -> Result<(), DownloadError> {
    let resp = client.get(url).send().await?;
    let status = resp.status();

    if !status.is_success() {
        return Err(DownloadError::BadStatus {
            url: url.to_string(),
            status: status,
        });
    }
    let bytes = resp.bytes().await?;

    let tmp_path = dest.with_extension("jpg.part");
    {
        let mut file = fs::File::create(&tmp_path).await?;
        file.write_all(&bytes).await?;
        file.flush().await?;
    }
    fs::rename(&tmp_path, dest).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let client = Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .connect_timeout(CONNECT_TIMEOUT)
        .build()?;
    let client = Arc::new(client);

    fs::create_dir_all(OUTPUT_DIR).await?;

    let results: Vec<(usize, Result<PathBuf, DownloadError>)> = stream::iter(1..=TOTAL_IMAGES)
        .map(|index| {
            let client = Arc::clone(&client);
            async move {
                let result = download_one(&client, index).await;
                (index, result)
            }
        })
        .buffer_unordered(MAX_CONCURRENCY)
        .collect()
        .await;

    let (succeeded, failed): (Vec<_>, Vec<_>) = results.into_iter().partition(|(_, r)| r.is_ok());

    for (index, result) in &failed {
        if let Err(e) = result {
            error!(index, error = %e, "download failed permanently");
        }
    }

    info!(
        succeeded = succeeded.len(),
        failed = failed.len(),
        total = TOTAL_IMAGES,
        "run complete"
    );

    if !failed.is_empty() {
        anyhow::bail!("{} of {} downloads failed", failed.len(), TOTAL_IMAGES);
    }

    Ok(())
}
