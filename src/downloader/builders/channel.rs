use std::{fmt::Debug, sync::Arc};

use crate::{
    Downloader, DwnChannel, GetId, Result,
    downloader::{builders::empty::EmptyBuilder, channel::browse::ChannelBrowse},
    itags::{AudioItag, Itag, VideoItag},
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

    pub fn audio(self) -> ChannelBuilder<AudioItag> {
        self.with_itag(AudioItag::Highest)
    }

    pub fn video(self) -> ChannelBuilder<VideoItag> {
        self.with_itag(VideoItag::Highest)
    }
}

impl<I> ChannelBuilder<I>
where
    I: Itag + Copy + Debug + Send + 'static,
    I::Stream: Debug + Send,
{
    // pub fn thumbnail(self) -> Self {
    //     Self {
    //         thumbnail: Some(ThumbRes::VeryHigh),
    //         ..self
    //     }
    // }

    pub async fn download(self) -> Result<DwnChannel<I::Stream>> {
        let downloaded = ChannelBrowse::new(self.id, self.downloader)
            .await?
            .browse()
            .await?
            .download(self.itag)
            .await?;
        Ok(downloaded)
    }
}
