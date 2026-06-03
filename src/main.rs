use std::{path::Path, time::SystemTime};

use ytuwu::{Downloader, GetId, IdCollection, Result, itags::AudioItag};

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = SystemTime::now();

    let url = "https://music.youtube.com/@ntomusic";

    let ids = IdCollection::from_url(url)?;

    // let id = ChannelNameId::new("@ntomusic")?;

    let downloader = Downloader::default();

    // let _ = downloader
    //     .download_channel(id, AudioItag::AacMedium)
    //     .await?;
    let downloaded = downloader
        .download_channel(ids.get_id()?, AudioItag::AacMedium)
        .await?;
    downloaded.save(Path::new("teehee"))?;

    println!("took: {:?}", start_time.elapsed().unwrap());
    Ok(())
}
