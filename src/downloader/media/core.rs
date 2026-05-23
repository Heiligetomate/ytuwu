use std::fmt::Debug;

use crate::{
    DwnBundleMedia, DwnMedia,
    downloader::{
        media::extracted_streams::{ExtractedStreams, ExtractedThumbnails, ThumbRes},
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

    pub async fn download_thumbnail(&self, resolution: &ThumbRes) -> Result<Thumbnail> {
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

    pub async fn download_streams(self, itags: &Vec<AnyItag>, thumb_res: &Option<ThumbRes>) -> Result<DwnBundleMedia> {
        let mut thumbnail = None;
        let mut streams = vec![];

        for itag in itags {
            let stream = match itag {
                AnyItag::Audio(i) => AnyStream::Audio(self.download_stream(*i).await?),
                AnyItag::LongVideo(i) => AnyStream::LongVideo(self.download_stream(*i).await?),
                AnyItag::ShortVideo(i) => AnyStream::ShortVideo(self.download_stream(*i).await?),
                AnyItag::Muxed(i) => AnyStream::Muxed(self.download_stream(*i).await?),
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

    pub async fn download<I>(&self, itag: I, thumb_res: &Option<ThumbRes>) -> Result<DwnMedia<I::Stream>>
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
