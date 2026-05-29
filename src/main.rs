use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use uuid::Uuid;
use ytuwu::{
    Downloader, HandleProgress, Id, Result,
    id_types::{ChannelNameId, FastBrowseId},
    itags::AudioItag,
    set_progress_handler,
};

struct Progress {
    ids: Mutex<HashMap<Uuid, (String, u32, u32)>>,
}

impl HandleProgress for Progress {
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
}

impl Progress {
    fn print(&self) {
        print!("\x1B[2J\x1B[H"); // clear screen
        let ids = self.ids.lock().unwrap();
        println!("Downloading {} track(s)\n", ids.len());
        for (title, done, total) in ids.values() {
            let percentage = (*done as f32 / *total as f32 * 100.0) as u32;
            let filled = percentage / 5;
            let bar = format!("[{}{}]", "█".repeat(filled as usize), "░".repeat((20 - filled) as usize));
            println!("  {} {}% {}", bar, percentage, title);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = SystemTime::now();

    //let url = "https://music.youtube.com/@ntomusic";
    let progress_handler = Progress { ids: Mutex::new(HashMap::new()) };
    set_progress_handler(Arc::new(progress_handler));

    let id = ChannelNameId::new("ntomusic")?;

    let downloader = Downloader::new();

    let downloaded = downloader
        .download_channel(id, AudioItag::AacLow)
        .await?;

    // downloaded.save_with_dir(&Path::new("teehee"))?;

    println!("took: {:?}", start_time.elapsed().unwrap());
    Ok(())
}
