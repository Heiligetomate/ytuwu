use anyhow::{Result, anyhow};

pub fn extract_size(url: &str) -> Result<u32> {
    let size: u32 = url
        .split("clen=")
        .nth(1)
        .ok_or(anyhow!("failed to get size from url"))?
        .split('&')
        .next()
        .ok_or(anyhow!("failed to get size from url"))?
        .parse()?;
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

