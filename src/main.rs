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

    //let playlist_url = "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
    
    let channel_url = "https://music.youtube.com/browse/MPADUC6Tg7GWjZw48EiZ8m5bRtWg";

    let id_collection = IdCollection::from_url(channel_url)?;

    let downloader = Downloader::new();
    let media = downloader.channel_test(id_collection.get_id()?).await?;
    let path = Path::new("teehee");

    Ok(())
}
