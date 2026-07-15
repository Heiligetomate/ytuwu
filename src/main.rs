use ytuwu::{Downloader, config::DwnConfig};

#[tokio::main]
async fn main() -> ytuwu::Result<()> {
    let downloader = Downloader::default();

    let config = DwnConfig::default().channel(|chn| {
        chn.singles(false)
            .eps(false)
            .max_albums(3)
    });

    let downloader = downloader.config(config);
    downloader
        .from_url("https://music.youtube.com/@ntomusic")?
        .as_channel()?
        .audio()
        .download()
        .await?;
    Ok(())
}
