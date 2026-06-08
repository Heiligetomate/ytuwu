use std::collections::HashMap;
use std::sync::Mutex;

use uuid::Uuid;

use crate::downloader::progress::handler::HandleProgress;

#[derive(Debug)]
struct ActiveChannel {
    album_done: u16,
    album_total: u16,
    ep_done: u16,
    ep_total: u16,
    single_done: u16,
    single_total: u16,
}

impl ActiveChannel {
    fn new(single_count: u16, playlist_count: u16, album_count: u16) -> Self {
        Self {
            album_done: 0,
            album_total: album_count,
            ep_done: 0,
            ep_total: playlist_count,
            single_done: 0,
            single_total: single_count,
        }
    }
}

#[derive(Debug)]
pub struct CleanChannelHandler {
    channels: Mutex<HashMap<Uuid, ActiveChannel>>,
}

impl CleanChannelHandler {
    pub fn new() -> Self {
        Self { channels: Mutex::new(HashMap::new()) }
    }

    fn print(&self) {
        print!("\x1B[2J\x1B[H");
        let channels = self.channels.lock().unwrap();

        for ch in channels.values() {
            println!("Channel");
            if ch.album_total > 0 {
                println!("  Albums   {}/{}", ch.album_done, ch.album_total);
            }
            if ch.ep_total > 0 {
                println!("  EPs      {}/{}", ch.ep_done, ch.ep_total);
            }
            if ch.single_total > 0 {
                println!("  Singles  {}/{}", ch.single_done, ch.single_total);
            }
        }
    }
}
impl HandleProgress for CleanChannelHandler {
    fn on_download_start(&self, _title: &str, _id: Uuid, _total_chunks: u32) {}
    fn on_chunk_downloaded(&self, _id: Uuid, _done: u32) {}
    fn on_playlist_started(&self, _id: Uuid, _songs: Vec<&str>) {}

    fn on_download_complete(&self, _id: Uuid) {
        let mut channels = self.channels.lock().unwrap();
        if let Some(ch) = channels.values_mut().next() {
            if ch.single_done < ch.single_total {
                ch.single_done += 1;
            }
        }
        self.print();
    }

    fn on_playlist_downloaded(&self, _id: Uuid) {
        let mut channels = self.channels.lock().unwrap();
        if let Some(ch) = channels.values_mut().next() {
            if ch.ep_done < ch.ep_total {
                ch.ep_done += 1;
            } else {
                ch.album_done += 1;
            }
        }
        self.print();
    }

    fn on_channel_started(&self, id: Uuid, single_count: u16, playlist_count: u16, ep_count: u16, _title: &str) {
        self.channels
            .lock()
            .unwrap()
            .insert(id, ActiveChannel::new(single_count, playlist_count, ep_count));
        self.print();
    }

    fn on_channel_downloaded(&self, id: Uuid) {
        self.channels
            .lock()
            .unwrap()
            .remove(&id);
        self.print();
    }
}
