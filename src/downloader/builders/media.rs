use std::{fmt::Debug, sync::Arc};

pub use crate::{Downloader, GetId, Result, downloader::builders::empty::EmptyBuilder, types::VideoId};
use crate::{
    DwnMedia, ThumbRes,
    downloader::media::browse::MediaBrowse,
    itags::{AudioItag, Itag, VideoItag},
};

pub struct EmptyMediaBuilder {
    downloader: Arc<Downloader>,
    id: VideoId,
}

pub struct MediaBuilder<I: Itag> {
    downloader: Arc<Downloader>,
    id: VideoId,
    itag: I,
    thumbnail: Option<ThumbRes>,
}

impl EmptyMediaBuilder {
    pub fn new(builder: EmptyBuilder) -> Result<Self> {
        Ok(Self {
            downloader: builder.downloader,
            id: builder.ids.get_id()?,
        })
    }

    fn with_itag<I: Itag>(self, itag: I) -> MediaBuilder<I> {
        let EmptyMediaBuilder { downloader, id } = self;

        MediaBuilder {
            itag,
            downloader,
            id,
            thumbnail: None,
        }
    }

    pub fn audio(self) -> MediaBuilder<AudioItag> {
        self.with_itag(AudioItag::Highest)
    }

    pub fn video(self) -> MediaBuilder<VideoItag> {
        self.with_itag(VideoItag::Highest)
    }
}

impl<I> MediaBuilder<I>
where
    I: Itag + Copy + Debug + Send + 'static,
    I::Stream: Debug + Send,
{
    pub fn thumbnail(self) -> Self {
        Self {
            thumbnail: Some(ThumbRes::VeryHigh),
            ..self
        }
    }

    pub async fn download(self) -> Result<DwnMedia<I::Stream>> {
        let downloaded = MediaBrowse::new(self.id)
            .browse(self.downloader)
            .await?
            .download(self.itag, self.thumbnail)
            .await?;
        Ok(downloaded)
    }
}
