//use std::{fs::create_dir_all, path::Path};

use ytuwu::Downloader;

// https://music.youtube.com/@TOOLmusic
// https://music.youtube.com/watch?v=-206erZYyBo&list=OLAK5uy_m-fARVi6vzn4fK0Xvwu21P6KdhfwtOPEQ

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
    let _dwn = downloader
        .from_url("https://music.youtube.com/watch?v=-206erZYyBo")?
        .as_media()?
        .audio()
        .download()
        .await?;

    // dwn.(Path::new("teehee"))?;
    Ok(())
}
