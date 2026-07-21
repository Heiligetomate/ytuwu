use std::{fs::create_dir_all, path::Path};

use ytuwu::{Downloader, config::Config};

#[tokio::main]
async fn main() -> ytuwu::Result<()> {
    let downloader = Downloader::default();
    // let config = DwnConfig::default().channel(|chn| {
    //     chn.singles(false)
    //         .eps(true)
    //         .albums(false)
    //         .max_albums(1)
    //         .max_eps(1)
    //         .max_singles(1)
    // });

    // let downloader = downloader.config(config);
    let dwn = downloader
        .from_url("https://music.youtube.com/@Kanonenfieber.official")?
        .as_channel()?
        .audio()
        .download()
        .await?;

    dwn.save_with_dir(Path::new("teehee"))?;
    Ok(())
}
