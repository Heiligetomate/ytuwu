use std::{fmt::Debug, sync::Arc, time::Duration};

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
    error::{Result, YtuwuError},
    itags::{AnyItag, Itag, ThumbRes},
    request,
};
use bytes::Bytes;
use std::sync::atomic::{AtomicU32, Ordering};
use uuid::Uuid;

/// This is the chunk size used for downloading chunks.
const CHUNK_SIZE: u32 = 1024 * 1024;
/// This determines the number of chunks downloaded at the same time.
const MAX_TASKS: usize = 8;
/// How often a single chunk is tried before the whole download fails.
const MAX_ATTEMPTS: u32 = 4;

/// This struct contains all downloadable media streams, thumbnail streams, an arc of the
/// downloader and an uuid for identification.
#[derive(Debug)]
pub struct Media {
    downloader: Arc<Downloader>,
    id: Uuid,
    pub(super) media_streams: ExtractedStreams,
    pub(super) thumbnail_streams: ExtractedThumbnails,
    pub metadata: MediaMetadata,
}

/// This struct holds the stream url and the range of what it has to download.
/// It is used for downloading chunks of media which is needed for better and faster downloads.
struct DownloadTask {
    from: u64,
    to: u64,
    url: String,
}

impl DownloadTask {
    /// Creates a new Task
    fn new(from: u64, to: u64, url: &str) -> Self {
        Self { from, to, url: url.to_owned() }
    }

    /// Uses a reference to a client to download.
    /// Downloads the task and returns the downloaded bytes.
    /// Fails if anything went wrong while downloading or if the server returned a different
    /// amount of bytes than requested.
    async fn download(&self, client: &reqwest::Client) -> Result<Bytes> {
        let chunk_url = format!("{}&range={}-{}", self.url, self.from, self.to);
        let expected = self.to - self.from + 1;
        let mut last_err = YtuwuError::YoutubeAPIReturn;

        for attempt in 1..=MAX_ATTEMPTS {
            match request::download_bytes(&chunk_url, client).await {
                Ok(chunk) if chunk.len() as u64 == expected => return Ok(chunk),
                Ok(_) => last_err = YtuwuError::YoutubeAPIReturn,
                Err(e) => last_err = e,
            }
            if attempt < MAX_ATTEMPTS {
                tokio::time::sleep(Duration::from_millis(300 * attempt as u64)).await;
            }
        }
        Err(last_err)
    }
}

impl Media {
    /// Takes ExtractedStreams, ExtractedThumbnails, MediaMetadata, an Arc to a downloader and an
    /// unique Uuid for identification.
    pub fn new(media_streams: ExtractedStreams, thumbnail_streams: ExtractedThumbnails, metadata: MediaMetadata, downloader: Arc<Downloader>, id: Uuid) -> Self {
        Self {
            downloader,
            id,
            media_streams,
            thumbnail_streams,
            metadata,
        }
    }

    /// Downloads a single stream with the given Itag.
    /// Gets the best available Itag if the Itag is Itag::Highest.
    /// Returns the Stream of the Itag I::Stream.
    /// Returns Err if the itag does not exist, something went wrong while downloading or the
    /// acquire of the permit went wrong.
    pub async fn download_stream<I: Itag>(&self, itag: I) -> Result<I::Stream> {
        let url = self
            .media_streams
            .get_best_stream(&itag)?;

        let size = extract_size(url)? as u64;
        let chunk_size = CHUNK_SIZE as u64;
        let total_chunks = size.div_ceil(chunk_size) as u32;

        let mut downloaded_stream = itag.new_stream();

        self.downloader
            .progress_handler
            .on_download_start(&self.metadata.title, self.id, total_chunks);

        let completed = Arc::new(AtomicU32::new(0));
        let semaphore = Arc::new(tokio::sync::Semaphore::new(MAX_TASKS));
        let mut tasks = Vec::with_capacity(total_chunks as usize);

        for i in 0..total_chunks as u64 {
            let from = i * chunk_size;
            let to = (from + chunk_size - 1).min(size - 1);
            let op = DownloadTask::new(from, to, url);
            let completed = Arc::clone(&completed);
            let id = self.id;
            let cloned = Arc::clone(&self.downloader);
            let permit = Arc::clone(&semaphore);
            tasks.push(tokio::spawn(async move {
                let _permit = permit.acquire().await?;
                let result = op.download(&cloned.client).await?;
                let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
                cloned
                    .progress_handler
                    .on_chunk_downloaded(id, done);
                Ok::<Bytes, YtuwuError>(result)
            }));
        }

        // Awaited in spawn order so the chunks are appended in the right order.
        for task in tasks {
            downloaded_stream.push_data(task.await??);
        }

        self.downloader
            .progress_handler
            .on_download_complete(self.id);

        Ok(downloaded_stream)
    }

    /// Extracts the stream with the correct thumbnail resolution.
    /// Downloads the thumbnail with the extracted url.
    /// Returns the downloaded thumbnail.
    /// Returns Err if something went wrong while sending the requests.
    pub async fn download_thumbnail(&self, resolution: ThumbRes) -> Result<Thumbnail> {
        let url = self
            .thumbnail_streams
            .get_thumbnail_url_by_res(&resolution)?;

        let client = &self.downloader.client;

        let bytes = request::download_bytes(url, &client).await?;
        Ok(Thumbnail::from_bytes(bytes))
    }

    /// Downloads all streams for the given itags by matching against every of the itags.
    /// Downloads the thumbnail if there was a thumbnail passed into the function.
    /// Returns DwnBundleMedia if everything worked well.
    /// Uses the already existing metadata on self for the DwnBundleMedia.
    /// Returns Err if something went wrong while downloading.
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

    /// This funciton downloads the media stream with the given itag.
    /// Downloads the thumbnail if there was a thumbnail passed into the function.
    /// Uses the already existing metadata for DwnMedia-
    /// Returns DwnMedia with the itags stream containing the thumbnail if existent, the mediastream
    /// and the metadata.
    /// Returns Err if something went wrong while downloadign the streams or the thumbnail.
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
