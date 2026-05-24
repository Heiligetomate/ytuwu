use std::path::Path;

use ytuwu::{
    Downloader, GetId, IdCollection, Result,
    itag::{Itag, LongVideoItag},
};

#[tokio::main]
async fn main() -> Result<()> {
    let playlist_url = "https://music.youtube.com/watch?v=hTWKbfoikeg&list=OLAK5uy_lYnxawfGdkGePjdFhIYaS6LjP-Md6UYf0";
    let id_collection = IdCollection::from_url(playlist_url)?;

    let downloader = Downloader::new();

    let media = downloader
        .download_media(id_collection.get_id()?, LongVideoItag::highest(), None)
        .await?;

    let path = Path::new("teehee");
    println!("title: {}", media.metadata.title);
    media.save_media_stream(&path)?;
    Ok(())
}
