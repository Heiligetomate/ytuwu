use std::{path::Path, time::SystemTime};

use ytuwu::{Downloader, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // let start_time = SystemTime::now();

    let url = "https://music.youtube.com/watch?v=vHdCCc1T8os&list=OLAK5uy_lD0SLa66bgs9XtbHQAlQjqlwdfHc72mDI";

    let downloader = Downloader::default();

    let start_time = SystemTime::now();

    let dwn = downloader
        .from_url(url)?
        .as_media()?
        .audio()
        .download()
        .await?;

    dwn.save_media_stream(Path::new("teehee"))?;

    println!("download took {:?}", start_time.elapsed().unwrap());

    Ok(())
}
