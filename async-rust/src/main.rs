use std::{path::Path, sync::Arc};

use futures::future::join_all;
use reqwest::Client;
use tokio::{fs, sync::Semaphore};

async fn download_image(
    client: Client,
    semaphore: Arc<Semaphore>,
    index: usize,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _permit = semaphore.acquire().await?;
    let url = format!(
        "https://yavuzceliker.github.io/sample-images/image-{}.jpg",
        index
    );

    println!("Downloading {}", url);

    let bytes = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    let path = format!("images/image-{}.jpg", index);
    fs::write(&path, &bytes).await?;
    println!("Saved {}", path);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let semaphore = Arc::new(Semaphore::new(4));

    let output_dir = Path::new("images");
    if !output_dir.exists() {
        fs::create_dir_all("images").await?;
    }

    let mut tasks = Vec::with_capacity(128);

    for i in 1..=128 {
        let client = client.clone();
        let semaphore = Arc::clone(&semaphore);

        tasks.push(tokio::spawn(async move {
            if let Err(e) = download_image(client, semaphore, i).await {
                eprintln!("Failed to download image {}: {}", i, e);
            }
        }));
    }
    join_all(tasks).await;

    println!("All downloads completed");

    Ok(())
}
