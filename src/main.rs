use std::{path::Path, time::SystemTime};

use ytuwu::{Downloader, GetId, IdCollection, Result, itags::AudioItag};

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = SystemTime::now();

    let url = "https://music.youtube.com/playlist?list=OLAK5uy_nmq4-rfcWad4OIuBpBnZxpXjeg8Fx9MvA";

    let ids = IdCollection::from_url(url)?;

    let downloader = Downloader::default();

    let downloaded = downloader
        .download_album(ids.get_id()?, AudioItag::AacLow, None)
        .await?;
    downloaded.save(Path::new("teehee"))?;

    println!("took: {:?}", start_time.elapsed().unwrap());
    Ok(())
}
