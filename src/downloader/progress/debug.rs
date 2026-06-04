use uuid::Uuid;

use crate::downloader::progress::handler::HandleProgress;

#[derive(Debug)]
pub struct EmptyHandler {}

impl HandleProgress for EmptyHandler {
    fn on_download_start(&self, _: &str, _: Uuid, _: u32) {}
    fn on_chunk_downloaded(&self, _: Uuid, _: u32) {}
    fn on_download_complete(&self, _: Uuid) {}
    fn on_playlist_started(&self, _: Uuid, _: Vec<&str>) {}
    fn on_playlist_downloaded(&self, _: Uuid) {}
}
