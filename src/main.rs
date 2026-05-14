use std::path::Path;

use ytuwu::id_resolver::id::GetId;
use ytuwu::id_resolver::id_collection::IdCollection;
use ytuwu::itag::{AudioItag, Itag};
use ytuwu::{Downloader, Result, ThumbnailResolution};

#[rustfmt::skip]
#[tokio::main]
async fn main() -> Result<()> {
    //let mixed_url = "https://music.youtube.com/watch?v=lndG8BiZCmM&list=OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU";
    //let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";
    //let short_url = "https://youtube.com/shorts/any_short";

    let playlist_url = "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
    let id_collection = IdCollection::from_url(playlist_url)?;

    let downloader = Downloader::new();
    let media = downloader
        .download_full_playlist(
            id_collection.get_id()?, 
            AudioItag::highest(), 
            ThumbnailResolution::Low
            )
        .await?;
    let path = Path::new("teehee");
    println!("title: {}", media.metadata.title);
    media.save(&path)?;

    Ok(())
}
