use std::{error::Error, fs, path::Path};

use album::SlowAlbumResponse;
use channel::ChannelBrowse;

mod album;
mod channel;

fn main() -> Result<(), Box<dyn Error>> {
    let channel_path = Path::new("./samples/channel.json");
    let album_path = Path::new("./samples/album.json");

    let channel_file_content = fs::read_to_string(channel_path)?;
    let album_file_content = fs::read_to_string(album_path)?;

    let channel_res: ChannelBrowse = serde_json::from_str(&channel_file_content)?;
    let album_res: SlowAlbumResponse = serde_json::from_str(&album_file_content)?;

    for item in &channel_res
        .contents
        .single_column_browse_results_renderer
        .tabs[0]
        .tab_renderer
        .content
        .section_list_renderer
        .contents[0]
        .grid_renderer
        .items
    {
        let r = &item.music_two_row_item_renderer;
        let browse_id = &r
            .navigation_endpoint
            .browse_endpoint
            .browse_id;
        let subtitle = r
            .subtitle
            .as_ref()
            .map(|s| {
                s.runs
                    .iter()
                    .map(|r| r.text.as_str())
                    .collect::<String>()
            })
            .unwrap_or_default();
        let release_type = subtitle
            .split('•')
            .next()
            .unwrap_or("")
            .trim();
        println!("{} | {}", browse_id, release_type);
    }

    for item in &album_res
        .contents
        .two_column_browse_results_renderer
        .secondary_contents
        .section_list_renderer
        .contents[0]
        .music_playlist_shelf_renderer
        .contents
    {
        let r = &item.music_responsive_list_item_renderer;
        let video_id = &r.playlist_item_data.video_id;
        let title = &r.flex_columns[0]
            .music_responsive_list_item_flex_column_renderer
            .text
            .runs[0]
            .text;
        let artist = &r.flex_columns[1]
            .music_responsive_list_item_flex_column_renderer
            .text
            .runs[0]
            .text;
        println!("{} | {} | {}", video_id, title, artist);
    }

    Ok(())
}
