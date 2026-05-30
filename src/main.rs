use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use uuid::Uuid;
use ytuwu::{Downloader, GetId, HandleProgress, IdCollection, Result, itags::AudioItag};

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = SystemTime::now();

    let url = "https://music.youtube.com/playlist?list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs";

    let ids = IdCollection::from_url(url)?;

    let downloader = Downloader::default();

    let downloaded = downloader
        .download_album(ids.get_id()?, AudioItag::AacLow, None)
        .await?;

    downloaded.save_with_dir(Path::new("teehee"))?;

    println!("took: {:?}", start_time.elapsed().unwrap());
    Ok(())
}
