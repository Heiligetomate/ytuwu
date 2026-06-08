use std::{path::Path, time::SystemTime};

use ytuwu::{Downloader, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // let start_time = SystemTime::now();

    let url = "https://music.youtube.com/watch?v=d1mkqz422lg";

    let downloader = Downloader::testing();

    let start_time = SystemTime::now();

    let dwn = downloader
        .from_url(url)?
        .as_media()?
        .dual()
        .download()
        .await?;

    dwn.save_media_streams(Path::new("teehee"))?;

    println!("download took {:?}", start_time.elapsed().unwrap());

    Ok(())
}
