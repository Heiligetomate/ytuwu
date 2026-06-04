use std::{collections::HashMap, sync::Mutex};

use uuid::Uuid;

use crate::downloader::progress::handler::HandleProgress;

const BAR_LENGTH: u32 = 20;
const COMPLETED: &'static str = "█";
const NOT_COMPLETED: &'static str = "░";

#[derive(Debug)]
pub struct DefaultProgressHandler {
    ids: Mutex<HashMap<Uuid, (String, u32, u32)>>,
}

impl HandleProgress for DefaultProgressHandler {
    fn on_download_start(&self, title: &str, id: Uuid, total_chunks: u32) {
        self.ids
            .lock()
            .unwrap()
            .insert(id, (title.to_string(), 0, total_chunks));
        self.print();
    }

    fn on_chunk_downloaded(&self, id: Uuid, done: u32) {
        if let Some(entry) = self.ids.lock().unwrap().get_mut(&id) {
            entry.1 = done;
        }
        self.print();
    }

    fn on_download_complete(&self, id: Uuid) {
        self.ids.lock().unwrap().remove(&id);
        self.print();
    }

    fn on_playlist_started(&self, id: Uuid, songs: Vec<&str>) {
        println!("downloading playlist: {:?} \nwith id {}", songs, id);
    }

    fn on_playlist_downloaded(&self, id: Uuid) {
        println!("downloaded playlist with id {}", id);
    }
}

impl DefaultProgressHandler {
    pub fn new() -> Self {
        Self { ids: Mutex::new(HashMap::new()) }
    }

    fn print(&self) {
        print!("\x1B[2J\x1B[H");
        let ids = self.ids.lock().unwrap();
        println!("Downloading {} track(s)\n", ids.len());
        for (title, done, total) in ids.values() {
            let percentage = if *total == 0 { 0 } else { ((*done as f32 / *total as f32) * 100.0).round() as u32 };
            let filled = (percentage * BAR_LENGTH / 100) as usize;
            let empty = (BAR_LENGTH as usize).saturating_sub(filled);
            let bar = format!("[{}{}]", COMPLETED.repeat(filled), NOT_COMPLETED.repeat(empty));
            println!("{} {}% {}", bar, percentage, title);
        }
    }
}
