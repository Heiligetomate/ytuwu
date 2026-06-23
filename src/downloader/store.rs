use std::{collections::HashMap, fmt::Debug};

use uuid::Uuid;

use crate::{
    Result,
    downloader::{
        channel::{DwnBundelChannel, DwnChannel},
        media::{DwnBundleMedia, DwnMedia},
        metadata::ChannelMetadata,
        playlist::{DwnBundleList, Dwnlist},
        streams::AnyStream,
        tasks::{FinishedBundleTask, FinishedTask},
    },
    error::{StorageError, YtuwuError},
};

/// This is a big part of the downloader stuct which holds all downloaded tasks created by the task
/// handler.  
/// It also holds channel templates which are important for extracting the correct downloaded media
/// and playlists for downloaded channels.
/// And it holds list names which are important to give the playlist back its metadata
/// The basic function is to push new downloaded data or to extract data with ids.
#[derive(Debug)]
pub struct DownloadedStore {
    channel_templates: HashMap<Uuid, ChannelTemplate>,
    list_names: HashMap<Uuid, String>,
    media: Vec<FinishedTask>,
    bundle: Vec<FinishedBundleTask>,
}

impl Default for DownloadedStore {
    /// Creates a new DownloadedStore with all values set to none
    fn default() -> Self {
        Self {
            channel_templates: HashMap::new(),
            list_names: HashMap::new(),
            media: Vec::new(),
            bundle: Vec::new(),
        }
    }
}

/// This struct is a template for downloaded channels
/// Its important because it gives the possibility to extract the correct channel structure from the
/// downloader store
/// it holds a vec for albums ids, a vec for single ids, a vec for ep ids and the channel metadata
/// It always gets inserted into the channel_templates field of the DownloadedStore with the correct
/// channel Uuid by calling task_handler.insert_channel_template(id, template)
#[derive(Debug)]
pub struct ChannelTemplate {
    metadata: ChannelMetadata,
    eps: Vec<Uuid>,
    albums: Vec<Uuid>,
    singles: Vec<Uuid>,
}

impl ChannelTemplate {
    /// Creates a new channel template with the given values
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
    /// inserts a new ChannelTemplate with the given channel uuid as key
    /// important for extracting the channel later
    pub fn insert_channel_template(&mut self, id: Uuid, template: ChannelTemplate) {
        self.channel_templates
            .insert(id, template);
    }

    /// Inserts a new title of a playlist with the playlist uuid as key
    /// Inportant for not loosing the playlist metadata
    pub fn insert_list_title(&mut self, id: Uuid, name: &str) {
        self.list_names
            .insert(id, name.to_owned());
    }

    /// Extract a playlist title by its uuid
    /// Fails if the id does not exist
    pub fn extract_list_title(&self, list_id: Uuid) -> Result<&str> {
        let title = self
            .list_names
            .get(&list_id)
            .ok_or(YtuwuError::Storage(StorageError::ListNameExtraction(list_id)))?;

        Ok(title.as_str())
    }

    /// Pushes a finished task on the media storage
    pub fn push(&mut self, finished: FinishedTask) {
        self.media.push(finished);
    }

    /// Pushes a finished bundle task on the bundle storage
    pub fn push_bundle(&mut self, finished: FinishedBundleTask) {
        self.bundle.push(finished);
    }

    /// Pushes a vec of finished tasks on the media storage
    pub fn push_vec(&mut self, finished: Vec<FinishedTask>) {
        self.media.extend(finished);
    }

    /// Pushes a vec of finished bundle tasks on the bundle storage
    pub fn push_bundle_vec(&mut self, finished: Vec<FinishedBundleTask>) {
        self.bundle.extend(finished);
    }

    /// Extract a media from the media storage.
    /// Removes the media from the storage and returns it as a DwnMedia with type AnyStream
    /// Fails if the media was not found
    pub fn extract_media(&mut self, id: Uuid) -> Result<DwnMedia<AnyStream>> {
        for (i, finished) in self.media.iter().enumerate() {
            if finished.id == id {
                let media = self.media.remove(i);
                return Ok(media.data);
            }
        }

        Err(YtuwuError::Storage(StorageError::MediaExtraction(id)))
    }

    /// Extract a bundle media from the bundle storage.
    /// Removes the bundle media from the storage and returns it as a DwnBundleMedia
    /// Fails if the bundle media was not found
    pub fn extract_bundle_media(&mut self, id: Uuid) -> Result<DwnBundleMedia> {
        for (i, finished) in self.bundle.iter().enumerate() {
            if finished.id == id {
                let media = self.bundle.remove(i);
                return Ok(media.data);
            }
        }

        Err(YtuwuError::Storage(StorageError::MediaExtraction(id)))
    }

    // TODO: Fail if empty
    /// Extract all finished tasks that have the given list id from the media storage
    /// Removes all the medias from the storage and return them as a Dwnlist with type AnyStream
    /// Fails if no media was found (not true rn)
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

    // TODO: Fail if empty
    /// Extract all finished bundle tasks that have the given list id from the bundle storage
    /// Removes all the bundle medias from the storage and return them as a DwnBundleList
    /// Fails if no bundle media was found (not true rn)
    pub fn extract_bundle_list(&mut self, list_id: Uuid) -> Result<DwnBundleList> {
        let mut extracted = Vec::new();

        let mut remaining = Vec::new();

        for finished in self.bundle.drain(..) {
            if finished.playlist_id == Some(list_id) {
                extracted.push(finished.data);
            } else {
                remaining.push(finished);
            }
        }

        self.bundle = remaining;

        let title = self.extract_list_title(list_id)?;
        let dwn_list = DwnBundleList::new(extracted, title);

        Ok(dwn_list)
    }

    /// Extracts all singles, eps and albums from the media storage by using the channel template
    /// which gets removed if found.
    /// It goes through every single id, album id and ep id and extract the list/media for each of
    /// those ids
    /// After extracting the media, a new DwnChannel with type AnyStream gets built from the
    /// extrated media and the already existing metadata
    /// Fails if any of the expected ids was not found in storage
    pub fn extract_channel(&mut self, id: Uuid) -> Result<DwnChannel<AnyStream>> {
        let ChannelTemplate { metadata, eps, albums, singles } = self
            .channel_templates
            .remove(&id)
            .ok_or(YtuwuError::Storage(StorageError::ChannelTemplateExtraction(id)))?;

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

    /// Extracts all singles, eps and albums from the bundle storage by using the channel template
    /// which gets removed if found.
    /// It goes through every single id, album id and ep id and extract the bundle list/media for each of
    /// those ids
    /// After extracting the bundles, a new DwnBundelChannel gets built from the
    /// extrated media and the already existing metadata
    /// Fails if any of the expected ids was not found in storage
    pub fn extract_bundle_channel(&mut self, id: Uuid) -> Result<DwnBundelChannel> {
        let ChannelTemplate { metadata, eps, albums, singles } = self
            .channel_templates
            .remove(&id)
            .ok_or(YtuwuError::Storage(StorageError::ChannelTemplateExtraction(id)))?;

        let mut dwn_singles = Vec::with_capacity(singles.len());
        let mut dwn_eps = Vec::with_capacity(eps.len());
        let mut dwn_albums = Vec::with_capacity(albums.len());

        for id in singles.iter() {
            let single = self.extract_bundle_media(*id)?;
            dwn_singles.push(single);
        }

        for id in eps.iter() {
            let ep = self.extract_bundle_list(*id)?;
            dwn_eps.push(ep);
        }

        for id in albums.iter() {
            let album = self.extract_bundle_list(*id)?;
            dwn_albums.push(album);
        }

        let channel = DwnBundelChannel::new(dwn_singles, dwn_eps, dwn_albums, metadata);

        Ok(channel)
    }
}
