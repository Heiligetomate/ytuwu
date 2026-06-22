use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Result,
    downloader::{
        Downloader,
        builders::empty::EmptyBuilder,
        channel::{ChannelBrowse, DwnBundelChannel, DwnChannel},
        streams::AnyStream,
    },
    id_resolver::GetId,
    itags::{AnyItag, AudioItag, Itag, VideoItag},
    types::ChannelId,
};

/// This struct gets created when calling as_channel on an EmptyBuilder
/// This is used for downloading entire channels or artists
/// This can not be used for downloading yet, it has to be configured by calling some methods first
pub struct EmptyChannelBuilder {
    downloader: Arc<Downloader>,
    id: ChannelId,
}

/// This stuct is ready for downloading an entire channel / artist by calling .download()
/// Holds the channel id and the itag that will be used for downloading the channel
/// Only one stream will be downloaded for each media
pub struct ChannelBuilder<I: Itag> {
    downloader: Arc<Downloader>,
    id: ChannelId,
    itag: I,
    // thumbnail: Option<ThumbRes>,
}

/// This struct is ready for downloading an entire channel / artist by calling .download()
/// Holds the channel id and the itags that will be used for downloading the channel
/// There will be a video stream and an audio stream downloaded for each media
pub struct MultipleChannelBuilder {
    downloader: Arc<Downloader>,
    id: ChannelId,
    // thumbnail: Option<ThumbRes>,
    itags: &'static [AnyItag],
}

impl EmptyChannelBuilder {
    /// Creates a new EmptyChannelBuilder from an empty builder.
    /// Tries to get the channel id from the id collection
    /// Fails if there is no channel id contained in the id collection
    pub fn new(builder: EmptyBuilder) -> Result<Self> {
        Ok(Self {
            downloader: builder.downloader,
            id: builder.ids.get_id()?,
        })
    }

    /// Consumes itself and converts it to a channel builder with the given itag
    /// Uses the old values but adds the itag
    fn with_itag<I: Itag>(self, itag: I) -> ChannelBuilder<I> {
        let EmptyChannelBuilder { downloader, id } = self;

        ChannelBuilder {
            itag,
            downloader,
            id,
            // thumbnail: None,
        }
    }

    /// Consumes itself and builds a new MultipleChannelBuilder which can be used for downloading
    /// multiple stream for each media in that channel
    /// Uses the old values but adds audio and video itag
    pub fn dual(self) -> MultipleChannelBuilder {
        let EmptyChannelBuilder { downloader, id } = self;
        MultipleChannelBuilder {
            downloader,
            id,
            itags: &[AnyItag::Audio(AudioItag::Highest), AnyItag::Video(VideoItag::Highest)],
        }
    }

    /// Call this method to convert the EmptyChannelBuilder to a ChannelBuilder that can download
    /// audio streams
    pub fn audio(self) -> ChannelBuilder<AudioItag> {
        self.with_itag(AudioItag::Highest)
    }

    /// Call this method to convert the EmptyChannelBuilder to a ChannelBuilder that can download
    /// video streams
    pub fn video(self) -> ChannelBuilder<VideoItag> {
        self.with_itag(VideoItag::Highest)
    }
}

impl<I> ChannelBuilder<I>
where
    I: Itag + 'static,
{
    // pub fn thumbnail(self) -> Self {
    //     Self {
    //         thumbnail: Some(ThumbRes::VeryHigh),
    //         ..self
    //     }
    // }

    /// Downloaded the channel with the configuration and returns the downloaded channel
    /// This is achieved by first pushing all tasks to the downloader task handler and extracting it
    /// from the downloader storage with the previously created uuid.
    /// Fails if any of the songs counld not be downloaded
    pub async fn download(self) -> Result<DwnChannel<AnyStream>> {
        let id = Uuid::new_v4();

        ChannelBrowse::new(self.id, Arc::clone(&self.downloader), Some(id))
            .await?
            .browse()
            .await?
            .add_tasks(self.itag.to_any())
            .await?;

        let downloader = self.downloader;

        let task_handler = std::mem::take(&mut *downloader.task_handler.lock().await);
        task_handler
            .work(Arc::clone(&downloader))
            .await;

        let channel = downloader
            .storage
            .lock()
            .await
            .extract_channel(id)?;

        Ok(channel)
    }
}

impl MultipleChannelBuilder {
    /// Downloaded the channel for both audio and video and returns the downloaded channel
    /// This is achieved by first pushing all tasks to the downloader task handler and extracting it
    /// from the downloader storage with the previously created uuid.
    /// Fails if any of the songs could not be downloaded
    pub async fn download(self) -> Result<DwnBundelChannel> {
        let id = Uuid::new_v4();

        ChannelBrowse::new(self.id, Arc::clone(&self.downloader), Some(id))
            .await?
            .browse()
            .await?
            .add_bundle_tasks(self.itags)
            .await?;

        let downloader = self.downloader;

        let task_handler = std::mem::take(&mut *downloader.task_handler.lock().await);
        task_handler
            .work(Arc::clone(&downloader))
            .await;

        let channel = downloader
            .storage
            .lock()
            .await
            .extract_bundle_channel(id)?;

        Ok(channel)
    }
}
