use std::{fmt::Debug, sync::Arc};

use uuid::Uuid;

pub use crate::{Downloader, GetId, Result, downloader::builders::empty::EmptyBuilder, types::VideoId};
use crate::{
    DwnBundleMedia, DwnMedia, ThumbRes,
    downloader::media::browse::MediaBrowse,
    itags::{AnyItag, AudioItag, Itag, VideoItag},
    streams::AnyStream,
};

pub struct EmptyMediaBuilder {
    downloader: Arc<Downloader>,
    id: VideoId,
    thumbnail: Option<ThumbRes>,
}

pub struct MediaBuilder<I: Itag> {
    downloader: Arc<Downloader>,
    id: VideoId,
    thumbnail: Option<ThumbRes>,
    itag: I,
}

pub struct MultipleMediaBuilder {
    downloader: Arc<Downloader>,
    id: VideoId,
    thumbnail: Option<ThumbRes>,
    itags: &'static [AnyItag],
}

impl EmptyMediaBuilder {
    pub fn new(builder: EmptyBuilder) -> Result<Self> {
        Ok(Self {
            downloader: builder.downloader,
            id: builder.ids.get_id()?,
            thumbnail: None,
        })
    }

    fn with_itag<I: Itag>(self, itag: I) -> MediaBuilder<I> {
        let EmptyMediaBuilder { downloader, id, thumbnail } = self;

        MediaBuilder { downloader, id, thumbnail, itag }
    }

    pub fn audio(self) -> MediaBuilder<AudioItag> {
        self.with_itag(AudioItag::Highest)
    }

    pub fn video(self) -> MediaBuilder<VideoItag> {
        self.with_itag(VideoItag::Highest)
    }

    pub fn thumbnail(self) -> Self {
        Self {
            thumbnail: Some(ThumbRes::VeryHigh),
            ..self
        }
    }

    pub fn dual(self) -> MultipleMediaBuilder {
        let EmptyMediaBuilder { downloader, id, thumbnail } = self;
        MultipleMediaBuilder {
            downloader,
            id,
            thumbnail,
            itags: &[AnyItag::Audio(AudioItag::Highest), AnyItag::Video(VideoItag::Highest)],
        }
    }
}

impl<I> MediaBuilder<I>
where
    I: Itag + 'static,
    AnyStream: From<I::Stream>,
{
    pub fn thumbnail(self) -> Self {
        Self {
            thumbnail: Some(ThumbRes::VeryHigh),
            ..self
        }
    }

    pub async fn download(self) -> Result<DwnMedia<AnyStream>> {
        let id = Uuid::new_v4();

        self.downloader
            .task_handler
            .lock()
            .await
            .push(self.id, None, None, id);

        self.downloader.work(self.itag).await?;

        let downloaded = self
            .downloader
            .downloaded
            .lock()
            .await
            .extract_media(id)?;
        Ok(downloaded)
    }
}

impl MultipleMediaBuilder {
    pub fn thumbnail(self) -> Self {
        Self {
            thumbnail: Some(ThumbRes::VeryHigh),
            ..self
        }
    }

    pub async fn download(self) -> Result<DwnBundleMedia> {
        let id = Uuid::new_v4();
        self.downloader
            .task_handler
            .lock()
            .await
            .push_bundle(self.id, None, None, id, self.itags);

        self.downloader.work(self.itag).await?;

        let downloaded = self
            .downloader
            .downloaded
            .lock()
            .await
            .extract_media(id)?;

        Ok(downloaded)
    }
}
