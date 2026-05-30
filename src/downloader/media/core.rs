use std::{fmt::Debug, sync::Arc};

use crate::{
    Downloader, DwnBundleMedia, DwnMedia,
    downloader::{
        media::extracted_streams::{ExtractedStreams, ExtractedThumbnails, ThumbRes},
        progress::ProgressChanger,
        streams::{AnyStream, MediaStream, Thumbnail},
        util::*,
    },
    error::Result,
    itags::{AnyItag, Itag},
    metadata::MediaMetadata,
};
use bytes::Bytes;
use std::sync::atomic::{AtomicU32, Ordering};
use uuid::Uuid;

const CHUNK_SIZE: u32 = 1024 * 1024;

#[derive(Debug)]
pub struct Media {
    downloader: Arc<Downloader>,
    id: Uuid,
    media_streams: ExtractedStreams,
    thumbnail_streams: ExtractedThumbnails,
    pub metadata: MediaMetadata,
}

struct DownloadTask {
    from: u32,
    to: u32,
    url: String,
}

impl DownloadTask {
    fn new(from: u32, to: u32, url: &str) -> Self {
        Self { from, to, url: url.to_owned() }
    }

    async fn download(&self) -> Result<Bytes> {
        let client = reqwest::Client::new();

        // println!("downloading chunk {} to {}", self.from, self.to);

        let chunk_url = format!("{}&range={}-{}", self.url, self.from, self.to);
        let chunk = client
            .get(&chunk_url)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(chunk)
    }
}

impl Media {
    pub fn new(media_streams: ExtractedStreams, thumbnail_streams: ExtractedThumbnails, metadata: MediaMetadata, downloader: Arc<Downloader>) -> Self {
        let id = Uuid::new_v4();
        Self {
            downloader,
            id,
            media_streams,
            thumbnail_streams,
            metadata,
        }
    }

    pub async fn download_stream<I: Itag + Copy>(&self, itag: I) -> Result<I::Stream> {
        let url = self
            .media_streams
            .get_best_stream(&itag)?;

        let size = extract_size(url)?;
        let total_chunks = size.div_ceil(CHUNK_SIZE);
        let mut current_position: u32 = 0;

        let mut downloaded_stream = itag.new_stream();

        let mut ops = Vec::new();
        let mut tasks = Vec::new();

        self.downloader
            .progress_handler
            .on_download_start(&self.metadata.title, self.id, total_chunks);

        for _ in 0..total_chunks {
            let op = DownloadTask::new(current_position, current_position + CHUNK_SIZE, url);
            ops.push(op);
            current_position += CHUNK_SIZE + 1
        }

        let completed = Arc::new(AtomicU32::new(0));

        for op in ops {
            let completed = Arc::clone(&completed);
            let id = self.id;
            let cloned = Arc::clone(&self.downloader);
            tasks.push(tokio::spawn(async move {
                let result = op.download().await;
                let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
                cloned
                    .progress_handler
                    .on_chunk_downloaded(id, done);
                result
            }));
        }

        for task in tasks {
            downloaded_stream.push_data(task.await??);
        }

        self.downloader
            .progress_handler
            .on_download_complete(self.id);

        Ok(downloaded_stream)
    }

    pub async fn download_thumbnail(&self, resolution: ThumbRes) -> Result<Thumbnail> {
        let url = self
            .thumbnail_streams
            .get_thumbnail_url_by_res(&resolution)?;
        let client = reqwest::Client::new();
        let thumbnail = client
            .get(url)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(Thumbnail::new(thumbnail))
    }

    pub async fn download_streams(self, itags: Vec<AnyItag>, thumb_res: Option<ThumbRes>) -> Result<DwnBundleMedia> {
        let mut thumbnail = None;
        let mut streams = vec![];

        for itag in itags {
            let stream = match itag {
                AnyItag::Audio(i) => AnyStream::Audio(self.download_stream(i).await?),
                AnyItag::LongVideo(i) => AnyStream::LongVideo(self.download_stream(i).await?),
                AnyItag::ShortVideo(i) => AnyStream::ShortVideo(self.download_stream(i).await?),
                AnyItag::Muxed(i) => AnyStream::Muxed(self.download_stream(i).await?),
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
        })
    }

    pub async fn download<I>(self, itag: I, thumb_res: Option<ThumbRes>) -> Result<DwnMedia<I::Stream>>
    where
        I: Itag + Copy + Debug,
        I::Stream: Debug,
    {
        let mut thumbnail = None;

        if let Some(res) = thumb_res {
            let dwn_thumb = self.download_thumbnail(res).await?;
            thumbnail = Some(dwn_thumb)
        }

        let media = self.download_stream(itag).await?;

        let downloaded_media = DwnMedia::new(media, self.metadata.clone(), thumbnail);

        Ok(downloaded_media)
    }
}
