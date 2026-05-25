use std::{path::Path, time::SystemTime};

use ytuwu::{
    Downloader, GetId, IdCollection, Result, ThumbRes,
    itag::{AnyItag, AudioItag},
};

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = SystemTime::now();

    let url = "https://music.youtube.com/playlist?list=OLAK5uy_lYnxawfGdkGePjdFhIYaS6LjP-Md6UYf0";

    let ids = IdCollection::from_url(url)?;

    let downloader = Downloader::new();

    let downloaded = downloader
        .download_playlist(ids.get_id()?, AudioItag::OpusMedium, None)
        .await?;

    let path = Path::new("teehee");
    downloaded.save_with_dir(path)?;

    println!("took: {:?}", start_time.elapsed().unwrap());

    Ok(())
}
