use std::path::Path;

use ytuwu::{
    Downloader, Result, ThumbnailResolution,
    id_resolver::{id::GetId, id_collection::IdCollection},
    itag::{AudioItag, Itag, LongVideoItag},
};

#[tokio::main]
async fn main() -> Result<()> {
    let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";
    let id_collection = IdCollection::from_url(media_url)?;

    let downloader = Downloader::new();
    let media = downloader
        .download_dual_media_stream(id_collection.get_id()?, LongVideoItag::highest(), AudioItag::highest(), ThumbnailResolution::Low)
        .await?;
    let path = Path::new("teehee");
    println!("title: {}", media.metadata.title);
    media.save(&path)?;

    Ok(())
}
