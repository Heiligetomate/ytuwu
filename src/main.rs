mod shared_traits;
mod downloader;
mod browse_model;
mod player_model;
mod request;
mod id_resolver;
mod name_trimmer;

use crate::{
    downloader::music_releases::Downloader, 
    id_resolver::IdCollection, 
    player_model::video_details::ThumbnailResolution
};
use anyhow::{Result, anyhow};

#[tokio::main]
async fn main() -> Result<()> {

    let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";
    let playlist_url = "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
    let mixed_url = "https://music.youtube.com/watch?v=lndG8BiZCmM&list=OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU";
    let id_collection = IdCollection::from_url(mixed_url);
    if let Some(ids) = id_collection {
        println!("{}", ids);
        let mut downloader = Downloader::new();
        downloader.add_playlist_browse(ids.browse_id.ok_or(anyhow!("no browse id found"))?);
        downloader.browse_all().await?;
        downloader
            .playlist_download(
                &player_model::itag::Itag::OpusMedium,
                &Some(ThumbnailResolution::VeryHigh),
            )
            .await?
            .get(0)
            .ok_or(anyhow!("nope"))?
            .save(None)?;



        //let _ = downloader
        //    .download_media(
        //        &player_model::itag::Itag::OpusMedium, 
        //        &Some(ThumbnailResolution::VeryHigh)
        //        )
        //    .await?
        //    .get(0)
        //    .ok_or(anyhow!("nope"))?
        //    .save_thumbnail();
    } else {
        println!("no ids found");
    }
    

    Ok(())
}


