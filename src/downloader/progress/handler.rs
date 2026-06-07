use std::fmt::Debug;

use uuid::Uuid;

pub trait HandleProgress: Send + Sync + Debug {
    fn on_download_start(&self, title: &str, id: Uuid, total_chunks: u32);
    fn on_chunk_downloaded(&self, id: Uuid, done: u32);
    fn on_download_complete(&self, id: Uuid);
    fn on_playlist_started(&self, id: Uuid, songs: Vec<&str>);
    fn on_playlist_downloaded(&self, id: Uuid);
    fn on_channel_started(&self, id: Uuid, single_count: u16, playlist_count: u16, ep_count: u16);
    fn on_channel_downloaded(&self, id: Uuid);
}
