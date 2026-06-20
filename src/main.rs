use std::{path::Path, time::SystemTime};

use ytuwu::{Downloader, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // let start_time = SystemTime::now();

    let url = "https://music.youtube.com/channel/UCSM02YABF__s7_3et2UommA";

    let downloader = Downloader::default();

    let start_time = SystemTime::now();

    let dwn = downloader
        .from_url(url)?
        .as_channel()?
        .audio()
        .download()
        .await?;

    dwn.save_with_dir(Path::new("teehee"))?;

    println!("download took {:?}", start_time.elapsed().unwrap());

    Ok(())
}
