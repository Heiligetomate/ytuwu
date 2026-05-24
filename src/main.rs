use std::{path::Path, time::SystemTime};

use ytuwu::{
    Downloader, GetId, IdCollection, Result, ThumbRes,
    itag::{AnyItag, AudioItag},
};

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = SystemTime::now();

    let url = "https://www.youtube.com/watch?v=AFNmwFpyB3E";

    let ids = IdCollection::from_url(url)?;

    let downloader = Downloader::new();

    let downloaded = downloader
        .download_media_bundle(
            ids.get_id()?,
            vec![AnyItag::Audio(AudioItag::AacMedium), AnyItag::LongVideo(ytuwu::itag::VideoItag::Highest)],
            Some(ThumbRes::High),
        )
        .await?;

    let path = Path::new("teehee");
    downloaded.save_full(path)?;

    println!("took: {:?}", start_time.elapsed().unwrap());

    Ok(())
}
