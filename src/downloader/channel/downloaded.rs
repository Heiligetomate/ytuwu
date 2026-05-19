use std::{
    fmt::Debug,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use crate::{
    Result,
    downloader::{media::downloaded::RawDownloadedMedia, media_stream::MediaStream, playlist::downloaded::RawDownloadedPlaylist},
    error::YtuwuError,
};

pub struct DownloadedChannel<M: MediaStream + Debug> {
    pub singles: Vec<RawDownloadedMedia<M>>,
    pub eps: Vec<RawDownloadedPlaylist<M>>,
    pub albums: Vec<RawDownloadedPlaylist<M>>,
}

impl<M: MediaStream + Debug> DownloadedChannel<M> {
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut singles_path = PathBuf::from(&path);
        let mut eps_path = PathBuf::from(&path);
        let mut albums_path = PathBuf::from(&path);

        singles_path.push("singles");
        eps_path.push("eps");
        albums_path.push("albums");

        create_dir_all(&singles_path).map_err(|_| YtuwuError::CreateDir)?;
        create_dir_all(&eps_path).map_err(|_| YtuwuError::CreateDir)?;
        create_dir_all(&albums_path).map_err(|_| YtuwuError::CreateDir)?;

        for single in self.singles.iter() {
            single.save(&singles_path)?;
        }

        for ep in self.eps.iter() {
            ep.save(&eps_path)?;
        }

        for album in self.albums.iter() {
            album.save(&albums_path)?;
        }

        Ok(())
    }
}
