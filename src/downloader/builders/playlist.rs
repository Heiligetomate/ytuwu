use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Result,
    downloader::{
        Downloader,
        builders::empty::EmptyBuilder,
        playlist::{DwnBundleList, Dwnlist, PlaylistBrowse},
        streams::AnyStream,
    },
    id_resolver::GetId,
    itags::{AnyItag, AudioItag, Itag, ThumbRes, VideoItag},
    types::BrowseId,
};

/// This struct gets created when calling as_list on an EmptyBuilder
/// This is used for downloading entire playlists and albums
/// This can not be used for downloading yet, it has to be configured by calling some methods first
pub struct EmptyListBuilder {
    downloader: Arc<Downloader>,
    id: BrowseId,
    thumbnail: Option<ThumbRes>,
}

/// This stuct is ready for downloading an entire list / album by calling .download()
/// Holds the browse id and the itag that will be used for downloading the list
/// Only one stream will be downloaded.
pub struct ListBuilder<I: Itag> {
    downloader: Arc<Downloader>,
    id: BrowseId,
    itag: I,
    #[allow(unused)]
    thumbnail: Option<ThumbRes>,
}

/// This struct is ready for downloading an entire list / album by calling .download()
/// Holds the browse id and the itags that will be used for downloading the album / list
/// There will be a video stream and an audio stream downloaded for each media
pub struct MultipleListBuilder {
    downloader: Arc<Downloader>,
    id: BrowseId,
    thumbnail: Option<ThumbRes>,
    itags: &'static [AnyItag],
}

impl EmptyListBuilder {
    /// Creates a new EmptyListBuilder from an EmptyBuilder.
    /// Tries to get the browse id from the id collection
    /// Fails if there is no browse id contained in the id collection
    pub fn new(builder: EmptyBuilder) -> Result<Self> {
        Ok(Self {
            downloader: builder.downloader,
            id: builder.ids.get_id()?,
            thumbnail: None,
        })
    }

    /// Consumes itself and converts it to a ListBuilder with the given itag
    /// Uses the old values but adds the itag
    fn with_itag<I: Itag>(self, itag: I) -> ListBuilder<I> {
        let EmptyListBuilder { downloader, id, thumbnail } = self;

        ListBuilder { itag, downloader, id, thumbnail }
    }

    /// Consumes itself and returns a new instance of itself with the thumbnail resolution set to
    /// very high
    pub fn thumbnail(self) -> Self {
        Self { thumbnail: Some(ThumbRes::VeryHigh), ..self }
    }

    /// Call this method to convert the EmptyListBuilder to a ListBuilder that can download
    /// audio streams
    pub fn audio(self) -> ListBuilder<AudioItag> {
        self.with_itag(AudioItag::Highest)
    }

    /// Call this method to convert the EmptyListBuilder to a ListBuilder that can download
    /// video streams
    pub fn video(self) -> ListBuilder<VideoItag> {
        self.with_itag(VideoItag::Highest)
    }

    /// Consumes itself and builds a new MultipleListBuilder which can be used for downloading
    /// multiple stream for each media in that list
    /// Uses the old values but adds audio and video itag
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
    /// Consumes itself and returns a new instance of itself with the thumbnail resolution set to
    /// very high
    pub fn thumbnail(self) -> Self {
        Self { thumbnail: Some(ThumbRes::VeryHigh), ..self }
    }

    /// Downloaded the list with the configuration and returns the downloaded list
    /// This is achieved by first pushing all tasks to the downloader task handler and extracting it
    /// from the downloader storage with the previously created uuid.
    /// Fails if any of the songs counld not be downloaded
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
            .storage
            .lock()
            .await
            .extract_list(id)?;

        Ok(downloaded)
    }
}

impl MultipleListBuilder {
    /// Consumes itself and returns a new instance of itself with the thumbnail resolution set to
    /// very high
    pub fn thumbnail(self) -> Self {
        Self { thumbnail: Some(ThumbRes::VeryHigh), ..self }
    }

    /// Downloaded the List for both audio and video and returns the downloaded bundle list
    /// This is achieved by first pushing all tasks to the downloader task handler and extracting it
    /// from the downloader storage with the previously created uuid.
    /// Fails if any of the songs could not be downloaded
    pub async fn download(self) -> Result<DwnBundleList> {
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
