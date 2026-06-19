use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Result,
    downloader::{
        Downloader,
        builders::empty::EmptyBuilder,
        media::ThumbRes,
        playlist::{DwnBundleList, Dwnlist, PlaylistBrowse},
        streams::AnyStream,
    },
    id_resolver::GetId,
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
    #[allow(unused)]
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
    I: Itag + 'static,
    I::Stream: Into<AnyStream>,
{
    pub fn thumbnail(self) -> Self {
        Self {
            thumbnail: Some(ThumbRes::VeryHigh),
            ..self
        }
    }

    pub async fn download(self) -> Result<Dwnlist<AnyStream>> {
        let id = Uuid::new_v4();

        let downloader = self.downloader;
        PlaylistBrowse::new(self.id, Arc::clone(&downloader), id)
            .browse()
            .await?
            .add_tasks(self.itag.to_any())
            .await?;
        downloader.work().await;

        let downloaded = downloader
            .downloaded
            .lock()
            .await
            .extract_list(id)?;

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
        // TODO: RAWR (add to task pool)
        let id = Uuid::new_v4();

        let downloaded = PlaylistBrowse::new(self.id, self.downloader, id)
            .browse()
            .await?
            .browse()
            .await?
            .download_bundle(self.itags, self.thumbnail)
            .await?;
        Ok(downloaded)
    }
}
