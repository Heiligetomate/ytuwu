use std::{fmt::Debug, sync::Arc};

use crate::{
    Downloader, DwnBundleList, Dwnlist, GetId, Result, ThumbRes,
    downloader::{builders::empty::EmptyBuilder, playlist::browse::PlaylistBrowse},
    itags::{AnyItag, AudioItag, Itag, VideoItag},
    types::BrowseId,
};

pub struct EmptyListBuilder {
    downloader: Arc<Downloader>,
    id: BrowseId,
    thumbnail: Option<ThumbRes>,
}

pub struct ListBuilder<I: Itag> {
    downloader: Arc<Downloader>,
    id: BrowseId,
    itag: I,
    thumbnail: Option<ThumbRes>,
}

pub struct MultipleListBuilder {
    downloader: Arc<Downloader>,
    id: BrowseId,
    thumbnail: Option<ThumbRes>,
    itags: &'static [AnyItag],
}

impl EmptyListBuilder {
    pub fn new(builder: EmptyBuilder) -> Result<Self> {
        Ok(Self {
            downloader: builder.downloader,
            id: builder.ids.get_id()?,
            thumbnail: None,
        })
    }

    fn with_itag<I: Itag>(self, itag: I) -> ListBuilder<I> {
        let EmptyListBuilder { downloader, id, thumbnail } = self;

        ListBuilder { itag, downloader, id, thumbnail }
    }

    pub fn thumbnail(self) -> Self {
        Self {
            thumbnail: Some(ThumbRes::VeryHigh),
            ..self
        }
    }

    pub fn audio(self) -> ListBuilder<AudioItag> {
        self.with_itag(AudioItag::Highest)
    }

    pub fn video(self) -> ListBuilder<VideoItag> {
        self.with_itag(VideoItag::Highest)
    }

    pub fn dual(self) -> MultipleListBuilder {
        let EmptyListBuilder { downloader, id, thumbnail } = self;
        MultipleListBuilder {
            downloader,
            id,
            thumbnail,
            itags: &[AnyItag::Audio(AudioItag::Highest), AnyItag::Video(VideoItag::Highest)],
        }
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

impl MultipleListBuilder {
    pub fn thumbnail(self) -> Self {
        Self {
            thumbnail: Some(ThumbRes::VeryHigh),
            ..self
        }
    }

    pub async fn download(self) -> Result<DwnBundleList> {
        let downloaded = PlaylistBrowse::new(self.id, self.downloader)
            .browse()
            .await?
            .browse()
            .await?
            .download_bundle(self.itags, self.thumbnail)
            .await?;
        Ok(downloaded)
    }
}
