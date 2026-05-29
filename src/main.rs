use std::{path::Path, sync::Arc, time::SystemTime};

use ytuwu::{
    Downloader, GetId, HandleProgress, Id, Result,
    id_types::{ChannelNameId, VideoId},
    itags::AudioItag,
    set_progress_handler,
};

struct Progress {}

impl HandleProgress for Progress {
    fn on_download_start(&self, title: &str, id: uuid::Uuid, total_chunks: u32) {
        println!("new started: {}\nchunks to do: {}", title, total_chunks);
    }

    fn on_chunk_downloaded(&self, id: uuid::Uuid, done: u32) {
        println!("print new chunk downloaded. {} total", done);
    }

    fn on_download_complete(&self, id: uuid::Uuid) {
        println!("downloaded");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = SystemTime::now();

    //let url = "https://music.youtube.com/@ntomusic";
    set_progress_handler(Arc::new(Progress {}));

    let id = VideoId::new("CDko2ux1bkE")?;

    let downloader = Downloader::new();

    let downloaded = downloader
        .download_media(id, AudioItag::OpusMedium, None)
        .await?;

    let path = Path::new("teehee");
    downloaded.save_media_stream(path)?;

    println!("took: {:?}", start_time.elapsed().unwrap());

    Ok(())
}
