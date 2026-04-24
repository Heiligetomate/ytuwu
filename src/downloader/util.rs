use crate::error::{YtuwuError, Result};

pub fn extract_size(url: &str) -> Result<u32> {
    let size: u32 = url
        .split("clen=")
        .nth(1)
        .ok_or(YtuwuError::UrlSizeExtract)?
        .split('&')
        .next()
        .ok_or(YtuwuError::UrlSizeExtract)?
        .parse()
        .map_err(|_| YtuwuError::UrlSizeExtract)?;
    Ok(size)
}

pub fn file_name(mime_type: &str, title: &str) -> String {
    let file_ending = file_ending_from_mime_type(mime_type);
    format!("{title}.{file_ending}")
}

pub fn file_ending_from_mime_type(mime_type: &str) -> &str {
    let parts: Vec<&str> = mime_type.split('/').collect();
    let res: Vec<&str> = parts[1].split(';').collect(); 
    
    res[0]
}

