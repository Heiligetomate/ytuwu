use std::fmt::Debug;

use crate::{
    DwnBundleList, DwnBundleMedia, DwnMedia, Dwnlist,
    downloader::{
        channel::{browse::ChannelBrowse, downloaded::DwnChannel},
        media::{browse::MediaBrowse, extracted_streams::ThumbRes},
        playlist::browse::PlaylistBrowse,
        thumbnail::Thumbnail,
    },
    error::Result,
    id_resolver::{
        id::MakeChannelId,
        id_types::{FastBrowseId, VideoId},
    },
    id_types::ShortId,
    itag::AnyItag,
    media_stream::MediaStream,
    models::itag::Itag,
};

#[derive(Debug)]
pub struct Downloader {}

impl Downloader {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    #[rustfmt::skip]
    pub async fn download_media_thumb(&self, video_id: VideoId, resolution: ThumbRes) -> Result<Thumbnail> {
        Ok(MediaBrowse::new(video_id)
            .browse()
            .await?
            .download_thumbnail(resolution)
            .await?)
    }

    #[rustfmt::skip]
    pub async fn download_media_stream<I: Itag + Copy>(&self, video_id: VideoId, itag: I) -> Result<I::Stream> {
        Ok(MediaBrowse::new(video_id)
            .browse()
            .await?
            .download_stream(itag)
            .await?)
    }

    #[rustfmt::skip]
    pub async fn download_media<I>(&self, video_id: VideoId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnMedia<I::Stream>>
    where
        I: Itag + Copy + Debug,
        I::Stream: Debug,
    {
        Ok(MediaBrowse::new(video_id)
            .browse()
            .await?
            .download(itag, thumbnail_resolution)
            .await?)
    }

    pub async fn download_media_bundle(&self, video_id: VideoId, itags: Vec<AnyItag>, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnBundleMedia> {
        Ok(MediaBrowse::new(video_id)
            .browse()
            .await?
            .download_streams(itags, thumbnail_resolution)
            .await?)
    }

    #[rustfmt::skip]
    pub async fn download_playlist<I>(&self, browse_id: FastBrowseId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<Dwnlist<I::Stream>>
    where
    I: Itag + Copy + Debug + Send + 'static,
    I::Stream: MediaStream + Debug + Send,
    {
        Ok(PlaylistBrowse::new(browse_id)
            .browse()
            .await?
            .browse()
            .await?
            .download(itag, thumbnail_resolution)
            .await?)
    }

    #[rustfmt::skip]
    pub async fn download_bundle_list(&self, browse_id: FastBrowseId, itags: Vec<AnyItag>, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnBundleList> {
        Ok(PlaylistBrowse::new(browse_id)
            .browse()
            .await?
            .browse()
            .await?
            .download_streams(itags, thumbnail_resolution)
            .await?)
    }

    #[rustfmt::skip]
    pub async fn download_short<I>(&self, short_id: ShortId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnMedia<I::Stream>>
    where
        I: Itag + Copy + Debug,
        I::Stream: Debug,
    {
        let video_id = short_id.transform()?;

        Ok(self
            .download_media(video_id, itag, thumbnail_resolution)
            .await?)
    }

    pub async fn download_channel<I, C>(&self, channel_id: C, itag: I) -> Result<DwnChannel<I::Stream>>
    where
        I: Itag + Copy + Debug + Send + 'static,
        I::Stream: MediaStream + Debug + Send,
        C: MakeChannelId,
    {
        let id = channel_id.transform().await?;
        Ok(ChannelBrowse::new(id)
            .browse()
            .await?
            .download(itag)
            .await?)
    }
}
