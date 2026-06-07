use std::{
    fmt::Debug,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use uuid::Uuid;

use crate::{
    DwnBundleList, DwnBundleMedia, Dwnlist, Result,
    downloader::{media::downloaded::DwnMedia, streams::MediaStream},
    error::YtuwuError,
};

#[derive(Debug)]
pub struct DwnChannel<M: MediaStream + Debug> {
    pub singles: Vec<DwnMedia<M>>,
    pub eps: Vec<Dwnlist<M>>,
    pub albums: Vec<Dwnlist<M>>,
}

#[derive(Debug)]
pub struct DwnBundelChannel {
    pub singles: Vec<DwnBundleMedia>,
    pub eps: Vec<DwnBundleList>,
    pub albums: Vec<DwnBundleList>,
}

impl DwnBundelChannel {
    pub fn save(&self, path: &Path) -> Result<()> {
        let (singles_path, eps_path, albums_path) = create_paths(path)?;

        for single in self.singles.iter() {
            single.save_media_streams(&singles_path)?;
        }

        for ep in self.eps.iter() {
            ep.save_with_dir(&eps_path)?;
        }

        for album in self.albums.iter() {
            album.save_with_dir(&albums_path)?;
        }

        Ok(())
    }
}

impl<M: MediaStream + Debug> DwnChannel<M> {
    pub fn save(&self, path: &Path) -> Result<()> {
        let (singles_path, eps_path, albums_path) = create_paths(path)?;

        for single in self.singles.iter() {
            single.save_media_stream(&singles_path)?;
        }

        for ep in self.eps.iter() {
            ep.save_with_dir(&eps_path)?;
        }

        for album in self.albums.iter() {
            album.save_with_dir(&albums_path)?;
        }

        Ok(())
    }

    // TODO: get the channel name
    pub fn save_with_dir(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(Uuid::new_v4().to_string());

        self.save(&full_path)
    }
}

pub(super) fn create_paths(path: &Path) -> Result<(PathBuf, PathBuf, PathBuf)> {
    let mut singles_path = PathBuf::from(&path);
    let mut eps_path = PathBuf::from(&path);
    let mut albums_path = PathBuf::from(&path);

    singles_path.push("singles");
    eps_path.push("eps");
    albums_path.push("albums");

    create_dir_all(&singles_path).map_err(|_| YtuwuError::CreateDir)?;
    create_dir_all(&eps_path).map_err(|_| YtuwuError::CreateDir)?;
    create_dir_all(&albums_path).map_err(|_| YtuwuError::CreateDir)?;

    Ok((singles_path, eps_path, albums_path))
}
