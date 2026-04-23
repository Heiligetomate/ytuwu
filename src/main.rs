mod shared_traits;
mod downloader;
mod browse_model;
mod player_model;
mod request;
mod id_resolver;
mod name_trimmer;

use std::path::Path;

use ytuwu::id_resolver::IdCollection;
use ytuwu::Downloader;
use ytuwu::itag::{VideoItag, AudioItag, Itag};
use ytuwu::ThumbnailResolution;

use anyhow::{Result, anyhow};

#[tokio::main]
async fn main() -> Result<()> {

    let playlist_url = "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
    let mixed_url = "https://music.youtube.com/watch?v=lndG8BiZCmM&list=OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU";

    let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";

    let id_collection = IdCollection::from_url(media_url);

    let downloader = Downloader::new();
    if let Some(ids) = id_collection {
        let media = downloader.download_dual_media_stream(
            ids.video_id.ok_or(anyhow!("no video id found"))?, 
            VideoItag::highest(),
            AudioItag::highest(), 
            ThumbnailResolution::VeryHigh,
        ).await?;
        let path = Path::new("teehee");
        media.save(&path)?;
        println!("{}", media.metadata.author);
    } else {
         println!("no ids found");
    }
    Ok(())
}


