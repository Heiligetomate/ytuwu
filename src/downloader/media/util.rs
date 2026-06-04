use crate::error::{Result, YtuwuError};

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
