use std::{fmt::Debug, fs::create_dir_all, path::{Path, PathBuf}};

use crate::{downloader::media_stream::PlaylistMediaStream, error::*};

use crate::{
    downloader::{
        media_stream::{
            AudioStream, 
            MediaStream, 
            VideoStream
        }, 
        metadata::{
            MediaMetadata, 
            PlaylistMetadata
        }, 
        thumbnail::Thumbnail
    }
};

#[derive(Debug)]
pub struct DownloadedMedia<M: MediaStream + Debug> {
    pub metadata: MediaMetadata,
    pub thumbnail: Thumbnail,
    pub stream: M,
}

pub struct DownloadedDualStreamMedia {
    pub metadata: MediaMetadata, 
    pub thumbnail: Thumbnail,
    pub audio_stream: AudioStream,
    pub video_stream: VideoStream,
}

impl DownloadedDualStreamMedia {

    pub fn new(audio_stream: AudioStream, video_stream: VideoStream, thumbnail: Thumbnail, title: &str, author: &str) -> Self {
        let metadata = MediaMetadata::new(title, author, None);
        Self {
            metadata,
            thumbnail,
            audio_stream,
            video_stream,
        }
    }

    fn save_thumbnail(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push("thumbnail.png");
        self.thumbnail.save_file(&full_path)?;
        Ok(())
    } 
        
    fn save_audio_stream(&self, path: &Path) -> Result<()> {
        self.audio_stream.save(path, "audio_stream")
    }

    fn save_video_stream(&self, path: &Path) -> Result<()> {
        self.video_stream.save(path, "video_stream")
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(&self.metadata.title);
        create_dir_all(&full_path).map_err(|_| YtuwuError::CreateDir)?;
        self.save_thumbnail(&full_path)?;
        self.save_video_stream(&full_path)?;
        self.save_audio_stream(&full_path)?;
        Ok(())
    }
}

impl<M: MediaStream + Debug> DownloadedMedia<M> {
    
    pub fn new(stream: M, title: &str, thumbnail: Thumbnail, author: &str) -> Self {
        let metadata = MediaMetadata::new(title, author, None);
        Self { thumbnail, stream, metadata }
    }
    
    pub fn save_thumbnail(&self, path: &Path) -> Result<()> {
        self.thumbnail.save(path)?;
        Ok(())
    }

    pub fn save_media_stream(&self, path: &Path) -> Result<()> {
        self.stream.save(path, &self.metadata.title)?;
        Ok(())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        self.save_thumbnail(&path)?;
        self.save_media_stream(&path)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DownloadedPlaylist<M: MediaStream + Debug> {
    pub media: PlaylistMediaStream<M>,
    pub metadata: PlaylistMetadata,
}

impl<M: MediaStream + Debug> DownloadedPlaylist<M> {
    pub fn new(title: &str, media: Vec<M>) -> Self {
        let metadata = PlaylistMetadata::new(title, media.len() as u16);
        let playalist_media_stream = PlaylistMediaStream::new(media);
        Self { media: playalist_media_stream, metadata }
    }
    
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(&self.metadata.title);
        create_dir_all(&full_path).map_err(|_| YtuwuError::CreateDir)?;
        for media in self.media.data.iter() {
            media.save(&full_path, media.metadata.title)?
        }
        Ok(())
    }
}

