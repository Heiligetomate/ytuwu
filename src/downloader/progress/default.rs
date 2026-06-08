use std::collections::HashMap;
use std::sync::Mutex;

use uuid::Uuid;

use crate::downloader::progress::handler::HandleProgress;

const BAR_LENGTH: u32 = 20;
const COMPLETED: &'static str = "█";
const NOT_COMPLETED: &'static str = "░";

#[derive(Debug)]
struct ActiveMedia {
    title: String,
    done: u32,
    total: u32,
}

#[derive(Debug)]
pub struct DefaultProgressHandler {
    active: Mutex<HashMap<Uuid, ActiveMedia>>,
    pending: Mutex<HashMap<Uuid, Vec<String>>>,
}

impl HandleProgress for DefaultProgressHandler {
    fn on_download_start(&self, title: &str, id: Uuid, total_chunks: u32) {
        {
            // for removing old pending stuff
            let mut pending = self.pending.lock().unwrap();
            for titles in pending.values_mut() {
                if let Some(pos) = titles.iter().position(|t| t == title) {
                    titles.remove(pos);
                    break;
                }
            }
            pending.retain(|_, v| !v.is_empty());
        }

        self.active.lock().unwrap().insert(
            id,
            ActiveMedia {
                title: title.to_string(),
                done: 0,
                total: total_chunks,
            },
        );
        self.print();
    }

    fn on_chunk_downloaded(&self, id: Uuid, done: u32) {
        if let Some(active) = self.active.lock().unwrap().get_mut(&id) {
            active.done = done;
        }
        self.print();
    }

    fn on_download_complete(&self, id: Uuid) {
        self.active.lock().unwrap().remove(&id);
        self.print();
    }

    fn on_playlist_started(&self, id: Uuid, songs: Vec<&str>) {
        let titles: Vec<String> = songs
            .iter()
            .map(|s| s.to_string())
            .collect();
        self.pending
            .lock()
            .unwrap()
            .insert(id, titles);
        self.print();
    }

    fn on_playlist_downloaded(&self, id: Uuid) {
        self.pending.lock().unwrap().remove(&id);
        self.print();
    }

    fn on_channel_started(&self, _id: Uuid, single_count: u16, ep_count: u16, album_count: u16, title: &str) {
        println!("channel download started.\nname: {title} singles: {single_count}\neps: {ep_count}\nalbums: {album_count}");
    }

    fn on_channel_downloaded(&self, _id: Uuid) {}
}

impl DefaultProgressHandler {
    pub fn new() -> Self {
        Self {
            active: Mutex::new(HashMap::new()),
            pending: Mutex::new(HashMap::new()),
        }
    }

    fn print(&self) {
        print!("\x1B[2J\x1B[H"); // clear screen

        let active = self.active.lock().unwrap();
        let pending = self.pending.lock().unwrap();

        let pending_titles: Vec<&String> = pending.values().flatten().collect(); // merges playlists

        let total = active.len() + pending_titles.len();

        println!("Downloading {} track(s)\n", total);

        for active in active.values() {
            let percentage = if active.total == 0 {
                0
            } else {
                ((active.done as f32 / active.total as f32) * 100.0).round() as u32
            };
            let filled = (percentage * BAR_LENGTH / 100) as usize;
            let empty = (BAR_LENGTH as usize).saturating_sub(filled);
            let bar = format!("[{}{}]", COMPLETED.repeat(filled), NOT_COMPLETED.repeat(empty));
            println!("{} {}% {}", bar, percentage, active.title);
        }

        if !pending_titles.is_empty() {
            println!("\nQueued:");
            for title in &pending_titles {
                println!("{} {}", NOT_COMPLETED.repeat(BAR_LENGTH as usize), title);
            }
        }
    }
}
