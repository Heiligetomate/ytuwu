use std::{error::Error, fs, path::Path};

use crate::{album::AlbumResponse, channel::ChannelBrowse};

mod album;
mod channel;

fn main() -> Result<(), Box<dyn Error>> {
    let channel_path = Path::new("./samples/channel.json");
    let album_path = Path::new("./samples/album.json");

    let channel_file_content = fs::read_to_string(channel_path)?;
    let album_file_content = fs::read_to_string(album_path)?;

    let channel_res: ChannelBrowse = serde_json::from_str(&channel_file_content)?;
    let album_res: AlbumResponse = serde_json::from_str(&album_file_content)?;

    println!("{:#?}", channel_res);
    println!("{:#?}", album_res);

    Ok(())
}
