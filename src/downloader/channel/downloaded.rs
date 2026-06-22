use crate::{
    Result,
    downloader::{
        media::{DwnBundleMedia, DwnMedia},
        metadata::ChannelMetadata,
        playlist::{DwnBundleList, Dwnlist},
        streams::{AnyStream, MediaStream},
    },
    error::YtuwuError,
};
use std::{
    fmt::Debug,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

/// This struct gets created when calling download() on a channel object
/// it contains metadata, a vec of DwnMedia for the singles and a vec of Dwnlist for albums and eps
/// All of them have the same Mediastream
#[derive(Debug)]
pub struct DwnChannel<M: MediaStream + Debug> {
    pub singles: Vec<DwnMedia<M>>,
    pub eps: Vec<Dwnlist<M>>,
    pub albums: Vec<Dwnlist<M>>,
    pub metadata: ChannelMetadata,
}

/// This struct gets created when callign downlaod_bundle on a channel object
/// It contains metadata, a vec of DwnBundleMedia for sinfles and a vec of DwnBundleList for albums
/// and eps.
/// All of them contain the same Mediastreams
#[derive(Debug)]
pub struct DwnBundelChannel {
    pub singles: Vec<DwnBundleMedia>,
    pub eps: Vec<DwnBundleList>,
    pub albums: Vec<DwnBundleList>,
    pub metadata: ChannelMetadata,
}

impl DwnBundelChannel {
    /// Creates a new DwnBundelChannel instance from the given parameters
    pub fn new(singles: Vec<DwnBundleMedia>, eps: Vec<DwnBundleList>, albums: Vec<DwnBundleList>, metadata: ChannelMetadata) -> Self {
        Self { singles, eps, albums, metadata }
    }
}

impl DwnChannel<AnyStream> {
    /// Creates a new DwnChannel instance from the given parameters for AnyStreams
    pub fn new(singles: Vec<DwnMedia<AnyStream>>, eps: Vec<Dwnlist<AnyStream>>, albums: Vec<Dwnlist<AnyStream>>, metadata: ChannelMetadata) -> Self {
        Self { singles, eps, albums, metadata }
    }
}

impl DwnBundelChannel {
    /// calls create paths with the path which creates a singles folder, an albums folder and an ep
    /// folder at the given path
    /// After that, every single gets saved by callign save_media_streams with the path for the
    /// singles, every ep gets saved by calling save_with_dir with the path for the eps and every
    /// album gets saved by calling save_with_dir with the paht for the albums.
    /// Fails if either the folders failed to create or any of the files could not save
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

    // TODO: Path is not created, not good
    /// Calls self.save with the given path but adds channel name to the path
    /// Faild if the creation of the directory failed or any of the saving failed
    pub fn save_with_dir(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(&self.metadata.name);

        self.save(&full_path)
    }
}

impl<M: MediaStream + Debug> DwnChannel<M> {
    /// calls create paths with the path which creates a singles folder, an albums folder and an ep
    /// folder at the given path
    /// After that, every single gets saved by callign save_media_stream with the path for the
    /// singles, every ep gets saved by calling save_with_dir with the path for the eps and every
    /// album gets saved by calling save_with_dir with the paht for the albums.
    /// Fails if either the folders failed to create or any of the files could not save
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

    // TODO: Path is not created, not good
    /// Calls self.save with the given path but adds channel name to the path
    /// Faild if the creation of the directory failed or any of the saving failed
    pub fn save_with_dir(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(&self.metadata.name);

        self.save(&full_path)
    }
}

/// Creates a singles fodlder at the given path, an eps folder and an album folder.
/// Returns the Paths
/// Fails if the directorys were not created
pub(super) fn create_paths(path: &Path) -> Result<(PathBuf, PathBuf, PathBuf)> {
    let mut singles_path = PathBuf::from(&path);
    let mut eps_path = PathBuf::from(&path);
    let mut albums_path = PathBuf::from(&path);

    singles_path.push("singles");
    eps_path.push("eps");
    albums_path.push("albums");

    create_dir_all(&singles_path).map_err(|e| channel_dir_error_message(e, "singles"))?;
    create_dir_all(&eps_path).map_err(|e| channel_dir_error_message(e, "eps"))?;
    create_dir_all(&albums_path).map_err(|e| channel_dir_error_message(e, "albums"))?;
    Ok((singles_path, eps_path, albums_path))
}

/// Creates a clean error message for the creation of directories for downloaded channels
/// Takes the std::io::Error that gets created when creating the dir and the type of the directory
/// that gets created which is just the name (single, album or ep)
fn channel_dir_error_message(e: std::io::Error, dir_type: &str) -> YtuwuError {
    YtuwuError::CreateDir(Some(format!("Failed to create the directory '{}' for storing the downloaded {} of the channel: {}", dir_type, dir_type, e.to_string())))
}
