use std::path::Path;

use ytuwu::{
    Downloader, GetId, IdCollection, Result, ThumbnailResolution,
    itag::{AudioItag, Itag},
};

#[tokio::main]
async fn main() -> Result<()> {
    let playlist_url = "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
    let id_collection = IdCollection::from_url(playlist_url)?;

    let downloader = Downloader::new();
    let media = downloader
        .download_full_playlist(id_collection.get_id()?, AudioItag::highest(), ThumbnailResolution::Low)
        .await?;
    let path = Path::new("teehee");
    println!("title: {}", media.metadata.title);
    media.save(&path)?;
    Ok(())
}
