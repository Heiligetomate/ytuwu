use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use crate::{Result, downloader::streams::core::MediaStream, error::YtuwuError};

/// This function takes a media stream, a path asnd a file name
/// The full path consists of the trimmed file name with the mime type as extension and the path
/// Returns an error if the given path is not a directory or if teh file saving did not work
pub fn save_media_stream<M>(path: &Path, file_name: &str, media_stream: &M) -> Result<()>
where
    M: MediaStream,
{
    let file_name = format!("{}.{}", trim_filename(file_name), media_stream.get_mime_type().as_str());
    if !path.is_dir() {
        return Err(YtuwuError::NotADir(Some(
            "Tried to save a media stream with a path that is not a directory. The file name gets automatically crated and should not be passed into the saving function.".into(),
        )));
    }
    let mut file_path = PathBuf::from(path);
    file_path.push(file_name);

    let mut file = File::create(file_path).map_err(|e| YtuwuError::CreateFile(Some(format!("Tried to create a new file to save a mediasteam but failed: {}", e.to_string()))))?;
    file.write_all(&media_stream.get_data())
        .map_err(|e| YtuwuError::WriteToFile(Some(format!("Tried to write the mediastream bytes to the file and failed: {}", e.to_string()))))?;
    Ok(())
}

/// This function replaces the most common special characters in file names with a "-"
pub fn trim_filename(filename: &str) -> String {
    // TODO: This is really bad
    filename
        .replace('/', "-")
        .replace('\\', "-")
        .replace(':', "-")
        .replace('*', "-")
        .replace('?', "-")
        .replace('"', "-")
        .replace('<', "-")
        .replace('>', "-")
        .replace('|', "-")
        .to_owned()
}
