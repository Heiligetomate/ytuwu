use std::path::Path;

use ytuwu::id_resolver::IdCollection;
use ytuwu::itag::{AudioItag, Itag};
use ytuwu::{Downloader, Result, ThumbnailResolution};

#[tokio::main]
async fn main() -> Result<()> {
    //let playlist_url = "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
    //let mixed_url = "https://music.youtube.com/watch?v=lndG8BiZCmM&list=OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU";

    let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";

    let id_collection = IdCollection::from_url(media_url);

    let downloader = Downloader::new();
    if let Some(ids) = id_collection {
        let media = downloader
            .download_full_playlist(
                ids.get_browse_id()?,
                AudioItag::highest(),
                ThumbnailResolution::Low,
            )
            .await?;
        let path = Path::new("teehee");
        media.save(&path)?;
    } else {
        println!("no ids found");
    }
    Ok(())
}
