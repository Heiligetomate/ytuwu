use std::{path::Path, sync::Arc, time::SystemTime};

use ytuwu::{Downloader, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // let start_time = SystemTime::now();

    let url = "https://music.youtube.com/watch?v=d1mkqz422lg&list=OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU";

    let downloader = Downloader::default();

    let start_time = SystemTime::now();

    let dwn = downloader
        .from_url(url)?
        .as_list()?
        .audio()
        .download()
        .await?;

    dwn.save_with_dir(Path::new("teehee"))?;

    println!("download took {:?}", start_time.elapsed().unwrap());

    Ok(())
}
