use std::path::Path;

use ytuwu::{Downloader, GetId, IdCollection, Result, id_types::ChannelId, itag::AudioItag};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://music.youtube.com/channel/UCwp0yHVvrCeO2DBRmoxrRcQ";

    let ids = IdCollection::from_url(url)?;

    let downloader = Downloader::new();

    let channel = downloader
        .download_channel(GetId::<ChannelId>::get_id(&ids)?, AudioItag::Highest)
        .await?;

    let path = Path::new("teehee");

    channel.save(path)?;

    Ok(())
}
