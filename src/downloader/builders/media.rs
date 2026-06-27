use std::sync::Arc;

use uuid::Uuid;

pub use crate::{Result, downloader::builders::empty::EmptyBuilder, types::VideoId};
use crate::{
    downloader::{
        Downloader,
        media::{DwnBundleMedia, DwnMedia},
        streams::AnyStream,
    },
    id_resolver::GetId,
    itags::{AnyItag, AudioItag, Itag, ThumbRes, VideoItag},
};

/// This struct gets created when calling as_media on an EmptyBuilder
/// This is used for downloading single songs or videos  
/// This can not be used for downloading yet, it has to be configured by calling some methods first
pub struct EmptyMediaBuilder {
    downloader: Arc<Downloader>,
    id: VideoId,
    thumbnail: Option<ThumbRes>,
}

/// This struct is ready for downloading media with one stream. This means that this can download
/// either an audio stream or a video stream
/// The video id and the itag will be used for downloading
pub struct MediaBuilder<I: Itag> {
    downloader: Arc<Downloader>,
    id: VideoId,
    #[allow(unused)]
    thumbnail: Option<ThumbRes>,
    itag: I,
}

/// This struct is ready for downloading media with both audio and video stream
/// The video id and the itags will be usef for downloading
pub struct MultipleMediaBuilder {
    downloader: Arc<Downloader>,
    id: VideoId,
    #[allow(unused)]
    thumbnail: Option<ThumbRes>,
    itags: &'static [AnyItag],
}

impl EmptyMediaBuilder {
    /// Creates a new EmptyMediaBuilder from an EmptyBuilder.
    /// Tries to get the video id from the id collection
    /// Fails if there is no video id contained in the id collection
    pub fn new(builder: EmptyBuilder) -> Result<Self> {
        Ok(Self {
            downloader: builder.downloader,
            id: builder.ids.get_id()?,
            thumbnail: None,
        })
    }

    /// Consumes itself and converts it to a MediaBuilder with the given itag
    /// Uses the old values but adds the itag
    fn with_itag<I: Itag>(self, itag: I) -> MediaBuilder<I> {
        let EmptyMediaBuilder { downloader, id, thumbnail } = self;

        MediaBuilder { downloader, id, thumbnail, itag }
    }

    /// Call this method to convert the EmptyChannelBuilder to a ChannelBuilder that can download
    /// audio streams
    pub fn audio(self) -> MediaBuilder<AudioItag> {
        self.with_itag(AudioItag::Highest)
    }

    /// Call this method to convert the EmptyChannelBuilder to a ChannelBuilder that can download
    /// video streams
    pub fn video(self) -> MediaBuilder<VideoItag> {
        self.with_itag(VideoItag::Highest)
    }

    /// Consumes itself and returns a new instance of itself with the thumbnail resolution set to
    /// very high
    pub fn thumbnail(self) -> Self {
        Self { thumbnail: Some(ThumbRes::VeryHigh), ..self }
    }

    /// Consumes itself and builds a new MultipleChannelBuilder which can be used for downloading
    /// multiple stream for each media in that channel
    /// Uses the old values but adds audio and video itag
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
    /// Consumes itself and returns a new instance of itself with the thumbnail resolution set to
    /// very high
    pub fn thumbnail(self) -> Self {
        Self { thumbnail: Some(ThumbRes::VeryHigh), ..self }
    }

    /// Downloaded the media with the configuration (audio/video) and returns the downloaded media
    /// This is achieved by first pushing a tasks to the downloader task handler and extracting it
    /// from the downloader storage with the previously created uuid.
    /// Fails if the song could not be downloaded
    pub async fn download(self) -> Result<DwnMedia<AnyStream>> {
        let id = Uuid::new_v4();

        self.downloader
            .task_handler
            .lock()
            .await
            .push(self.id, None, None, id, self.itag.to_any());

        self.downloader.work().await;

        let downloaded = self
            .downloader
            .storage
            .lock()
            .await
            .extract_media(id)?;
        Ok(downloaded)
    }
}

impl MultipleMediaBuilder {
    /// Consumes itself and returns a new instance of itself with the thumbnail resolution set to
    /// very high
    pub fn thumbnail(self) -> Self {
        Self { thumbnail: Some(ThumbRes::VeryHigh), ..self }
    }

    /// Downloaded the media for both audio and video and returns the downloaded bundle media
    /// This is achieved by first pushing a tasks to the downloader task handler and extracting it
    /// from the downloader storage with the previously created uuid.
    /// Fails if the song could not be downloaded
    pub async fn download(self) -> Result<DwnBundleMedia> {
        let id = Uuid::new_v4();
        self.downloader
            .task_handler
            .lock()
            .await
            .push_bundle(self.id, None, None, id, self.itags);

        self.downloader.work().await;

        let downloaded = self
            .downloader
            .storage
            .lock()
            .await
            .extract_bundle_media(id)?;

        Ok(downloaded)
    }
}
