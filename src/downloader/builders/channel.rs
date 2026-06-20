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

pub struct EmptyChannelBuilder {
    downloader: Arc<Downloader>,
    id: ChannelId,
}

pub struct ChannelBuilder<I: Itag> {
    downloader: Arc<Downloader>,
    id: ChannelId,
    itag: I,
    // thumbnail: Option<ThumbRes>,
}

pub struct MultipleChannelBuilder {
    downloader: Arc<Downloader>,
    id: ChannelId,
    // thumbnail: Option<ThumbRes>,
    itags: &'static [AnyItag],
}

impl EmptyChannelBuilder {
    pub fn new(builder: EmptyBuilder) -> Result<Self> {
        Ok(Self {
            downloader: builder.downloader,
            id: builder.ids.get_id()?,
        })
    }

    fn with_itag<I: Itag>(self, itag: I) -> ChannelBuilder<I> {
        let EmptyChannelBuilder { downloader, id } = self;

        ChannelBuilder {
            itag,
            downloader,
            id,
            // thumbnail: None,
        }
    }

    pub fn dual(self) -> MultipleChannelBuilder {
        let EmptyChannelBuilder { downloader, id } = self;
        MultipleChannelBuilder {
            downloader,
            id,
            itags: &[AnyItag::Audio(AudioItag::Highest), AnyItag::Video(VideoItag::Highest)],
        }
    }

    pub fn audio(self) -> ChannelBuilder<AudioItag> {
        self.with_itag(AudioItag::Highest)
    }

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
