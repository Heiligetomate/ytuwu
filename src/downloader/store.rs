use std::fmt::Debug;

use uuid::Uuid;

use crate::{
    DwnMedia, Result,
    error::YtuwuError,
    streams::{AnyStream, MediaStream},
};

type AnyMedia = DwnMedia<AnyStream>;

#[derive(Debug)]
pub struct DownloadedStore {
    contents: Vec<AnyMedia>,
}

impl Default for DownloadedStore {
    fn default() -> Self {
        Self { contents: Vec::new() }
    }
}

impl DownloadedStore {
    pub fn push(&mut self, media: AnyMedia) {
        self.contents.push(media);
    }

    pub fn push_any_vec(&mut self, medias: Vec<AnyMedia>) {
        self.contents.extend(medias);
    }

    pub fn push_vec<M>(&mut self, mut medias: Vec<DwnMedia<M>>)
    where
        M: MediaStream + Debug + Into<AnyStream>,
    {
        for media in medias.drain(..) {
            self.contents.push(media.to_any());
        }
    }

    pub fn extract_media(&mut self, id: Uuid) -> Result<DwnMedia<AnyStream>> {
        for (i, media) in self.contents.iter().enumerate() {
            if media.id == id {
                let media = self.contents.remove(i);
                return Ok(media);
            }
        }
        Err(YtuwuError::MediaNotInStorage)
    }
}
