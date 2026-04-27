use std::path::Path;

use ytuwu::id_resolver::{GetId, IdCollection};
use ytuwu::itag::{AudioItag, Itag, ShortVideoItag};
use ytuwu::{Downloader, Result, ThumbnailResolution};

#[tokio::main]
async fn main() -> Result<()> {
    //let playlist_url = "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
    //let mixed_url = "https://music.youtube.com/watch?v=lndG8BiZCmM&list=OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU";
    let short_url = "https://youtube.com/shorts/mW5Cit8CNrA?si=2kzTtul8T4Cwqp70";
    //let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";

    let id_collection = IdCollection::from_url(short_url)?;

    let downloader = Downloader::new();
    let media = downloader
        .download_short(id_collection.get_id()?, ShortVideoItag::highest(), AudioItag::highest(), ThumbnailResolution::Low)
        .await?;
    let path = Path::new("teehee");
    media.save(&path)?;

    Ok(())
}
