use std::{path::Path, time::SystemTime};

use ytuwu::{
    Downloader, GetId, Id, IdCollection, Result,
    itags::{AnyItag, AudioItag},
    types::ChannelId,
};

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = SystemTime::now();

    let url = "https://music.youtube.com/channel/UCSM02YABF__s7_3et2UommA";

    let ids = IdCollection::from_url(url)?;
    let downloader = Downloader::default();

    let downloaded = downloader
        .download_channel_bundle(ids.get_id()?, &[AnyItag::Audio(AudioItag::OpusLow), AnyItag::LongVideo(ytuwu::itags::VideoItag::MP4144p)])
        .await?;

    downloaded.save(Path::new("teehee"))?;

    println!("took: {:?}", start_time.elapsed().unwrap());
    Ok(())
}
