use std::{collections::HashMap, fmt::Debug};

use uuid::Uuid;

use crate::{
    Result,
    downloader::{
        channel::DwnChannel,
        media::{DwnBundleMedia, DwnMedia},
        metadata::ChannelMetadata,
        playlist::Dwnlist,
        streams::AnyStream,
        task_handler::{FinishedBundleTask, FinishedTask},
    },
    error::YtuwuError,
};

#[derive(Debug)]
pub struct DownloadedStore {
    channel_templates: HashMap<Uuid, ChannelTemplate>,
    list_names: HashMap<Uuid, String>,
    media: Vec<FinishedTask>,
    bundle: Vec<FinishedBundleTask>,
}

impl Default for DownloadedStore {
    fn default() -> Self {
        Self {
            channel_templates: HashMap::new(),
            list_names: HashMap::new(),
            media: Vec::new(),
            bundle: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct ChannelTemplate {
    metadata: ChannelMetadata,
    eps: Vec<Uuid>,
    albums: Vec<Uuid>,
    singles: Vec<Uuid>,
}

impl ChannelTemplate {
    pub fn new(metadata: ChannelMetadata, ep_ids: Vec<Uuid>, album_ids: Vec<Uuid>, single_ids: Vec<Uuid>) -> Self {
        Self {
            metadata,
            eps: ep_ids,
            albums: album_ids,
            singles: single_ids,
        }
    }
}

impl DownloadedStore {
    pub fn insert_channel_template(&mut self, id: Uuid, template: ChannelTemplate) {
        self.channel_templates
            .insert(id, template);
    }

    pub fn insert_list_title(&mut self, id: Uuid, name: &str) {
        self.list_names
            .insert(id, name.to_owned());
    }

    pub fn extract_list_title(&self, list_id: Uuid) -> Result<&str> {
        let title = self
            .list_names
            .get(&list_id)
            .ok_or(YtuwuError::ListNameNotFound)?;

        Ok(title.as_str())
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

        let title = self.extract_list_title(list_id)?;
        let dwn_list = Dwnlist::new(extracted, title);

        Ok(dwn_list)
    }

    pub fn extract_channel(&mut self, id: Uuid) -> Result<DwnChannel<AnyStream>> {
        let ChannelTemplate { metadata, eps, albums, singles } = self
            .channel_templates
            .remove(&id)
            .ok_or(YtuwuError::InvalidChannelId)?;

        let mut dwn_singles = Vec::with_capacity(singles.len());
        let mut dwn_eps = Vec::with_capacity(eps.len());
        let mut dwn_albums = Vec::with_capacity(albums.len());

        for id in singles.iter() {
            let single = self.extract_media(*id)?;
            dwn_singles.push(single);
        }

        for id in eps.iter() {
            let ep = self.extract_list(*id)?;
            dwn_eps.push(ep);
        }

        for id in albums.iter() {
            let album = self.extract_list(*id)?;
            dwn_albums.push(album);
        }

        let channel = DwnChannel::new(dwn_singles, dwn_eps, dwn_albums, metadata);

        Ok(channel)
    }
}
