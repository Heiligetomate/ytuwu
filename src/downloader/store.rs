use std::{collections::HashMap, fmt::Debug};

use uuid::Uuid;

use crate::{
    Result,
    downloader::{
        media::{DwnBundleMedia, DwnMedia},
        playlist::Dwnlist,
        streams::AnyStream,
        task_handler::{FinishedBundleTask, FinishedTask},
    },
    error::YtuwuError,
};

#[derive(Debug)]
pub struct DownloadedStore {
    list_names: HashMap<Uuid, String>,
    media: Vec<FinishedTask>,
    bundle: Vec<FinishedBundleTask>,
}

impl Default for DownloadedStore {
    fn default() -> Self {
        Self {
            list_names: HashMap::new(),
            media: Vec::new(),
            bundle: Vec::new(),
        }
    }
}

impl DownloadedStore {
    pub fn push_list_title(&mut self, id: Uuid, name: &str) {
        self.list_names
            .insert(id, name.to_owned());
    }

    pub fn push(&mut self, finished: FinishedTask) {
        self.media.push(finished);
    }

    pub fn push_bundle(&mut self, finished: FinishedBundleTask) {
        self.bundle.push(finished);
    }

    pub fn push_any_vec(&mut self, finished: Vec<FinishedTask>) {
        self.media.extend(finished);
    }

    // pub fn push_vec<M>(&mut self, mut medias: Vec<DwnMedia<M>>)
    // where
    //     M: MediaStream + Into<AnyStream>,
    // {
    //     for media in medias.drain(..) {
    //         self.media.push(media.to_any());
    //     }
    // }

    pub fn push_bundle_vec(&mut self, finished: Vec<FinishedBundleTask>) {
        self.bundle.extend(finished);
    }

    pub fn extract_media(&mut self, id: Uuid) -> Result<DwnMedia<AnyStream>> {
        for (i, finished) in self.media.iter().enumerate() {
            if finished.id == id {
                let media = self.media.remove(i);
                return Ok(media.data);
            }
        }

        Err(YtuwuError::MediaNotInStorage)
    }

    pub fn extract_bundle_media(&mut self, id: Uuid) -> Result<DwnBundleMedia> {
        for (i, finished) in self.bundle.iter().enumerate() {
            if finished.id == id {
                let media = self.bundle.remove(i);
                return Ok(media.data);
            }
        }
        Err(YtuwuError::MediaNotInStorage)
    }

    pub fn extract_list(&mut self, list_id: Uuid) -> Result<Dwnlist<AnyStream>> {
        let mut extracted = Vec::new();

        let mut remaining = Vec::new();

        for finished in self.media.drain(..) {
            if finished.playlist_id == Some(list_id) {
                extracted.push(finished.data);
            } else {
                remaining.push(finished);
            }
        }

        self.media = remaining;

        let title = self
            .list_names
            .get(&list_id)
            .ok_or(YtuwuError::ListNameNotFound)?;

        let dwn_list = Dwnlist::new(extracted, title);

        Ok(dwn_list)
    }
}
