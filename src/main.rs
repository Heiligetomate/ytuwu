mod shared_traits;
mod downloader;
mod browse_model;
mod player_model;
mod request;
mod id_resolver;
mod name_trimmer;

use std::path::Path;

use crate::{
    downloader::downloader::Downloader, 
    id_resolver::IdCollection, 
    player_model::video_details::ThumbnailResolution,
    player_model::itag::Itag,
};
use anyhow::{Result, anyhow};

#[tokio::main]
async fn main() -> Result<()> {

    let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";
    let playlist_url = "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
    let mixed_url = "https://music.youtube.com/watch?v=lndG8BiZCmM&list=OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU";
    let id_collection = IdCollection::from_url(mixed_url);
    

    let downloader = Downloader::new();
    if let Some(ids) = id_collection {
        let media = downloader.download_full_media(
            ids.video_id.ok_or(anyhow!("no video id found"))?, 
            &Itag::AacMedium, 
            ThumbnailResolution::VeryHigh,
        ).await?;
        println!("title: {} \nauthor: {:?}", media.title, media.artist);
        let path = Path::new("hallo");
        media.save(&path)?;
    
    } else {
         println!("no ids found");
    }
    

    Ok(())
}


