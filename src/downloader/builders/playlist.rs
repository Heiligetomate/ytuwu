use std::{fmt::Debug, sync::Arc};

use crate::{
    Downloader, Dwnlist, GetId, Result, ThumbRes,
    downloader::{builders::empty::EmptyBuilder, playlist::browse::PlaylistBrowse},
    itags::{AudioItag, Itag, VideoItag},
    types::BrowseId,
};

pub struct EmptyListBuilder {
    downloader: Arc<Downloader>,
    id: BrowseId,
}

pub struct ListBuilder<I: Itag> {
    downloader: Arc<Downloader>,
    id: BrowseId,
    itag: I,
    thumbnail: Option<ThumbRes>,
}

impl EmptyListBuilder {
    pub fn new(builder: EmptyBuilder) -> Result<Self> {
        Ok(Self {
            downloader: builder.downloader,
            id: builder.ids.get_id()?,
        })
    }

    fn with_itag<I: Itag>(self, itag: I) -> ListBuilder<I> {
        let EmptyListBuilder { downloader, id } = self;

        ListBuilder {
            itag,
            downloader,
            id,
            thumbnail: None,
        }
    }

    pub fn audio(self) -> ListBuilder<AudioItag> {
        self.with_itag(AudioItag::Highest)
    }

    pub fn video(self) -> ListBuilder<VideoItag> {
        self.with_itag(VideoItag::Highest)
    }
}

impl<I> ListBuilder<I>
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

    pub async fn download(self) -> Result<Dwnlist<I::Stream>> {
        let downloaded = PlaylistBrowse::new(self.id, self.downloader)
            .browse()
            .await?
            .browse()
            .await?
            .download(self.itag, self.thumbnail)
            .await?;
        Ok(downloaded)
    }
}
