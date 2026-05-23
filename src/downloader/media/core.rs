use std::fmt::Debug;

use crate::{
    MultipleStreamMedia,
    downloader::{
        media::{
            downloaded::DownloadedMediaWithThumbnail,
            extracted_streams::{ExtractedStreams, ExtractedThumbnails, ThumbnailResolution},
        },
        media_stream::{AnyStream, MediaStream},
        thumbnail::Thumbnail,
        util::*,
    },
    error::Result,
    itag::AnyItag,
    metadata::MediaMetadata,
    models::itag::Itag,
};
use bytes::Bytes;

const CHUNK_SIZE: u32 = 1024 * 1024;

#[derive(Debug)]
pub struct Media {
    media_streams: ExtractedStreams,
    thumbnail_streams: ExtractedThumbnails,
    metadata: MediaMetadata,
}

impl Media {
    pub fn new(media_streams: ExtractedStreams, thumbnail_streams: ExtractedThumbnails, metadata: MediaMetadata) -> Self {
        Self {
            media_streams,
            thumbnail_streams,
            metadata,
        }
    }

    async fn download_chunk(&self, from: u32, to: u32, url: &str) -> Result<Bytes> {
        let client = reqwest::Client::new();
        let chunk_url = format!("{}&range={}-{}", url, from, to);
        let chunk = client
            .get(&chunk_url)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(chunk)
    }

    async fn download_stream<I: Itag + Copy>(&self, itag: I) -> Result<I::Stream> {
        let url = self
            .media_streams
            .get_best_stream(&itag)?;
        let size = extract_size(url)?;
        let mut downloaded_stream = itag.new_stream();
        let mut current_position: u32 = 0;

        while size > current_position {
            println!("downloading chunk {} to {}", current_position, current_position + CHUNK_SIZE);
            let chunk = self
                .download_chunk(current_position, current_position + CHUNK_SIZE, url)
                .await?;
            downloaded_stream.push_data(chunk);
            current_position += CHUNK_SIZE + 1
        }
        Ok(downloaded_stream)
    }

    pub async fn download_thumbnail(&self, resolution: &ThumbnailResolution) -> Result<Thumbnail> {
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
        Ok(Thumbnail::new(thumbnail, &self.metadata.title))
    }

    pub async fn download_streams(self, itags: Vec<AnyItag>) -> Result<MultipleStreamMedia> {
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
        Ok(MultipleStreamMedia { metadata: self.metadata, streams })
    }

    pub async fn download_full<I>(self, itag: I, thumbnail_resolution: &ThumbnailResolution) -> Result<DownloadedMediaWithThumbnail<I::Stream>>
    where
        I: Itag + Copy + Debug,
        I::Stream: Debug,
    {
        let thumbnail = self
            .download_thumbnail(&thumbnail_resolution)
            .await?;
        let media = self.download_stream(itag).await?;

        let downloaded_media = DownloadedMediaWithThumbnail::new(media, thumbnail, self.metadata);

        Ok(downloaded_media)
    }
}
