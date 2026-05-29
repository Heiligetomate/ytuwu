use std::sync::{Arc, OnceLock};

use uuid::Uuid;

use crate::{Result, error::YtuwuError};

pub trait HandleProgress: Send + Sync {
    fn on_download_start(&self, title: &str, id: Uuid, total_chunks: u32);
    fn on_chunk_downloaded(&self, id: Uuid, done: u32);
    fn on_download_complete(&self, id: Uuid);
}

static PROGRESS: OnceLock<Arc<dyn HandleProgress + Send + Sync>> = OnceLock::new();

pub struct ProgressChanger {}

impl ProgressChanger {
    fn get_handler() -> Result<&'static Arc<dyn HandleProgress + Send + Sync>> {
        if let Some(handler) = PROGRESS.get() {
            return Ok(handler);
        }
        Err(YtuwuError::ProgressHandler)
    }

    pub fn start_media_download(title: &str, id: Uuid, total: u32) -> Result<()> {
        let handler = Self::get_handler()?;
        handler.on_download_start(title, id, total);
        Ok(())
    }

    pub fn update_chunks(id: Uuid, done: u32) -> Result<()> {
        Self::get_handler()?.on_chunk_downloaded(id, done);
        Ok(())
    }

    pub fn media_download_complete(id: Uuid) -> Result<()> {
        Self::get_handler()?.on_download_complete(id);
        Ok(())
    }
}

pub fn set_progress_handler(handler: Arc<dyn HandleProgress + Send + Sync>) {
    PROGRESS.set(handler).ok();
}
