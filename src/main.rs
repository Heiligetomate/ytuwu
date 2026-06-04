use std::{path::Path, time::SystemTime};

use ytuwu::{Downloader, GetId, IdCollection, Result, itags::AudioItag};

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = SystemTime::now();

    let url = "https://music.youtube.com/watch?v=HPG7gYoqpHM&list=OLAK5uy_mgi7GF3ptCZvPbGOBICaqmMQlHCH7p0Uk";

    let ids = IdCollection::from_url(url)?;

    let downloader = Downloader::default();

    let downloaded = downloader
        .download_album(ids.get_id()?, AudioItag::OpusMedium, None)
        .await?;

    downloaded.save(Path::new("teehee"))?;

    println!("took: {:?}", start_time.elapsed().unwrap());
    Ok(())
}
