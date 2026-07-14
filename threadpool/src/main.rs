#![forbid(unsafe_code)]

mod download;
mod pool;

const NUMBER_OF_WORKERS: usize = 12;
const THREADPOOL_QUEUE_SIZE: usize = 100;

use std::{fs, path::PathBuf, sync::Arc, time::Duration};

use crate::{download::download_image, pool::ThreadPool};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("images")?;

    let pool = ThreadPool::new(NUMBER_OF_WORKERS, THREADPOOL_QUEUE_SIZE)?;

    let client = Arc::new(
        reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(15))
            .pool_max_idle_per_host(12)
            .build()?,
    );

    for i in 1..200 {
        let client = Arc::clone(&client);

        let url = format!(
            "https://yavuzceliker.github.io/sample-images/image-{}.jpg",
            i
        );

        let path = PathBuf::from(format!("images/image-{}.jpg", i));

        pool.execute(move || match download_image(&client, &url, &path) {
            Ok(_) => println!("Downloaded image {}", i),
            Err(e) => eprintln!("Failed to download image image {}: {}", i, e),
        })?;
    }

    Ok(())
}
