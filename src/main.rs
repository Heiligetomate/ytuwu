use std::{path::Path, time::SystemTime};

use ytuwu::{
    Downloader, GetId, IdCollection, Result, ThumbRes,
    id_types::{ChannelId, ChannelNameId},
    itag::{AnyItag, AudioItag},
};

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = SystemTime::now();

    let url = "https://music.youtube.com/@ntomusic";

    let ids = IdCollection::from_url(url)?;

    let downloader = Downloader::new();

    let downloaded = downloader
        .download_channel(GetId::<ChannelNameId>::get_id(&ids)?, AudioItag::OpusMedium)
        .await?;

    let path = Path::new("teehee");
    downloaded.save(path)?;

    // TODO this doesnt save proberly

    println!("took: {:?}", start_time.elapsed().unwrap());

    Ok(())
}
