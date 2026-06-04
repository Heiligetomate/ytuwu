use crate::{Downloader, Id, downloader::channel::browse::ChannelBrowse, types::ChannelId};

#[tokio::test]
async fn test_normal_channel_browse_creation() {
    let downloader = Downloader::testing();
    let normal_id = ChannelId::new("MPADUC6Tg7GWjZw48EiZ8m5bRtWg").unwrap();

    ChannelBrowse::new(normal_id, downloader)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_name_channel_browse_creation() {
    let downloader = Downloader::testing();

    let name_id = ChannelId::new("@ntomusic").unwrap();

    ChannelBrowse::new(name_id, downloader)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_channel_browse() {
    let browse = ChannelBrowse::new(ChannelId::new("@ntomusic").unwrap(), Downloader::testing())
        .await
        .unwrap();

    let browsed = browse.browse().await.unwrap();
    assert!(browsed.singles.len() >= 42);
    assert!(browsed.eps.len() >= 7);
    assert!(browsed.albums.len() >= 4);
}

#[tokio::test]
async fn test_channel_download() {
    todo!();
}
