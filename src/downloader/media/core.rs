use std::{fmt::Debug, sync::Arc};

use crate::{
    downloader::{
        Downloader,
        media::{
            DwnBundleMedia, DwnMedia,
            extracted_streams::{ExtractedStreams, ExtractedThumbnails},
            util::extract_size,
        },
        metadata::MediaMetadata,
        streams::{AnyStream, MediaStream, Thumbnail},
    },
    error::Result,
    itags::{AnyItag, Itag, ThumbRes},
    request,
};
use bytes::Bytes;
use std::sync::atomic::{AtomicU32, Ordering};
use uuid::Uuid;

/// This is the chunk size used for downloading chunks.
const CHUNK_SIZE: u32 = 1024 * 1024;
/// This determines the number of chunks downloaded at the same time
const MAX_TASKS: usize = 8;

/// This struct contains all downloadable media streams, thumbnail streams, an arc of the
/// downloader and an uuid for identification  
#[derive(Debug)]
pub struct Media {
    downloader: Arc<Downloader>,
    id: Uuid,
    pub(super) media_streams: ExtractedStreams,
    pub(super) thumbnail_streams: ExtractedThumbnails,
    pub metadata: MediaMetadata,
}

/// This struct holds the stream url and the range of what it has to download.
/// It is used for downloading chunks of media which is needed for better and faster downloads
struct DownloadTask {
    from: u32,
    to: u32,
    url: String,
}

impl DownloadTask {
    /// Creates a new Task
    fn new(from: u32, to: u32, url: &str) -> Self {
        Self { from, to, url: url.to_owned() }
    }

    // TODO: put this in the request module
    /// Uses a reference to a client to download.
    /// Downloads the task and returns the downloaded bytes
    /// Fails if anything went wrong while downloading
    async fn download(&self, client: &reqwest::Client) -> Result<Bytes> {
        let chunk_url = format!("{}&range={}-{}", self.url, self.from, self.to);
        let chunk = request::download_bytes(&chunk_url, client).await?;
        Ok(chunk)
    }
}

impl Media {
    /// Takes ExtractedStreams, ExtractedThumbnails, MediaMetadata, an Arc to a downloader and an
    /// unique Uuid for identification
    pub fn new(media_streams: ExtractedStreams, thumbnail_streams: ExtractedThumbnails, metadata: MediaMetadata, downloader: Arc<Downloader>, id: Uuid) -> Self {
        Self {
            downloader,
            id,
            media_streams,
            thumbnail_streams,
            metadata,
        }
    }

    /// Downloads a single stream with the given Itag
    /// Gets the best available Itag if the Itag is Itag::Highest
    /// Returns the Stream of the Itag I::Stream
    /// Returns Err if the itag does not exist, something went wrong while downloading or the
    /// acquire of the permit went wrong.
    pub async fn download_stream<I: Itag>(&self, itag: I) -> Result<I::Stream> {
        let url = self
            .media_streams
            .get_best_stream(&itag)?;

        let size = extract_size(url)?;
        let total_chunks = size.div_ceil(CHUNK_SIZE);
        let mut current_position: u32 = 0;

        let mut downloaded_stream = itag.new_stream();

        let mut tasks = Vec::new();

        self.downloader
            .progress_handler
            .on_download_start(&self.metadata.title, self.id, total_chunks);

        let completed = Arc::new(AtomicU32::new(0));

        let semaphore = Arc::new(tokio::sync::Semaphore::new(MAX_TASKS));
        for _ in 0..total_chunks {
            let op = DownloadTask::new(current_position, current_position + CHUNK_SIZE, url);
            let completed = Arc::clone(&completed);
            let id = self.id;
            let cloned = Arc::clone(&self.downloader);
            let permit = Arc::clone(&semaphore);
            tasks.push(tokio::spawn(async move {
                let _permit = permit.acquire().await?;
                let result = op.download(&cloned.client).await;
                let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
                cloned
                    .progress_handler
                    .on_chunk_downloaded(id, done);
                result
            }));
            current_position += CHUNK_SIZE + 1;
        }

        for task in tasks {
            downloaded_stream.push_data(task.await??);
        }

        self.downloader
            .progress_handler
            .on_download_complete(self.id);

        Ok(downloaded_stream)
    }

    /// Extracts the stream with the correct thumbnail resolution
    /// Downloads the thumbnail with the extracted url
    /// Returns the downloaded thumbnail
    /// Returns Err if something went wrong while sending the requests
    pub async fn download_thumbnail(&self, resolution: ThumbRes) -> Result<Thumbnail> {
        let url = self
            .thumbnail_streams
            .get_thumbnail_url_by_res(&resolution)?;

        let client = &self.downloader.client;

        let bytes = request::download_bytes(url, &client).await?;
        Ok(Thumbnail::from_bytes(bytes))
    }

    /// Downloads all streams for the given itags by matching against every of the itags
    /// Downloads the thumbnail if there was a thumbnail passed into the function
    /// Returns DwnBundleMedia if everything worked well.
    /// Uses the already existing metadata on self for the DwnBundleMedia
    /// Returns Err if something went wrong while downloading
    pub async fn download_bundle(self, itags: &[AnyItag], thumb_res: Option<ThumbRes>) -> Result<DwnBundleMedia> {
        let mut thumbnail = None;
        let mut streams = vec![];

        for itag in itags {
            let stream = match itag {
                AnyItag::Audio(i) => AnyStream::Audio(self.download_stream(*i).await?),
                AnyItag::Video(i) => AnyStream::Video(self.download_stream(*i).await?),
                AnyItag::Short(i) => AnyStream::Short(self.download_stream(*i).await?),
                AnyItag::Muxed(i) => AnyStream::Muxed(self.download_stream(*i).await?),
                AnyItag::Thumbnail(i) => AnyStream::Thumbnail(self.download_stream(*i).await?),
            };
            streams.push(stream);
        }

        if let Some(res) = thumb_res {
            let dwn_thumb = self.download_thumbnail(res).await?;
            thumbnail = Some(dwn_thumb)
        }

        Ok(DwnBundleMedia {
            metadata: self.metadata,
            streams,
            thumbnail,
            id: self.id,
        })
    }

    /// This funciton downloads the media stream with the given itag
    /// Downloads the thumbnail if there was a thumbnail passed into the function
    /// Uses the already existing metadata for DwnMedia
    /// Returns DwnMedia with the itags stream containing the thumbnail if existent, the mediastream
    /// and the metadata
    /// Returns Err if something went wrong while downloadign the streams or the thumbnail
    pub async fn download<I: Itag>(self, itag: I, thumb_res: Option<ThumbRes>) -> Result<DwnMedia<I::Stream>>
where {
        let mut thumbnail = None;

        if let Some(res) = thumb_res {
            let dwn_thumb = self.download_thumbnail(res).await?;
            thumbnail = Some(dwn_thumb)
        }

        let media = self.download_stream(itag).await?;

        let downloaded_media = DwnMedia::new(media, self.metadata.clone(), thumbnail, self.id);

        Ok(downloaded_media)
    }
}
