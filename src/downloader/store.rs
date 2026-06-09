use std::fmt::Debug;

use uuid::Uuid;

use crate::{
    DwnBundleMedia, DwnMedia, Result,
    error::YtuwuError,
    streams::{AnyStream, MediaStream},
};

type AnyMedia = DwnMedia<AnyStream>;

#[derive(Debug)]
pub struct DownloadedStore {
    media: Vec<AnyMedia>,
    bundle: Vec<DwnBundleMedia>,
}

impl Default for DownloadedStore {
    fn default() -> Self {
        Self {
            media: Vec::new(),
            bundle: Vec::new(),
        }
    }
}

impl DownloadedStore {
    pub fn push(&mut self, media: AnyMedia) {
        self.media.push(media);
    }

    pub fn push_bundle(&mut self, media: DwnBundleMedia) {
        self.bundle.push(media);
    }

    pub fn push_any_vec(&mut self, medias: Vec<AnyMedia>) {
        self.media.extend(medias);
    }

    pub fn push_vec<M>(&mut self, mut medias: Vec<DwnMedia<M>>)
    where
        M: MediaStream + Into<AnyStream>,
    {
        for media in medias.drain(..) {
            self.media.push(media.to_any());
        }
    }

    pub fn push_bundle_vec(&mut self, medias: Vec<DwnBundleMedia>) {
        self.bundle.extend(medias);
    }

    pub fn extract_media(&mut self, id: Uuid) -> Result<DwnMedia<AnyStream>> {
        for (i, media) in self.media.iter().enumerate() {
            if media.id == id {
                let media = self.media.remove(i);
                return Ok(media);
            }
        }

        for (i, media) in self.bundle.iter().enumerate() {
            if media.id == id {
                let media = self.media.remove(i);
                return Ok(media);
            }
        }
        Err(YtuwuError::MediaNotInStorage)
    }
}
