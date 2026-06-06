use std::fs::read_dir;

use crate::{
    Downloader, GetId, Id, IdCollection,
    downloader::channel::{browse::ChannelBrowse, downloaded::create_paths},
    itags::{AnyItag, AudioItag},
    types::ChannelId,
};

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

#[allow(unused_comparisons)] // count might change when the artist uploads eps
#[tokio::test]
#[ignore = "takes very long"]
async fn test_channel_download_and_saving() {
    let downloaded = ChannelBrowse::new(
        IdCollection::from_url("https://music.youtube.com/channel/UCwp0yHVvrCeO2DBRmoxrRcQ")
            .unwrap()
            .get_id()
            .unwrap(),
        Downloader::testing(),
    )
    .await
    .unwrap()
    .browse()
    .await
    .unwrap()
    .download(AudioItag::AacLow)
    .await
    .unwrap();

    assert!(downloaded.singles.len() >= 15);
    assert!(downloaded.eps.len() >= 0);
    assert!(downloaded.albums.len() >= 8);

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    let expected_singles = path.join("singles");
    let expected_eps = path.join("eps");
    let expected_albums = path.join("albums");

    downloaded.save(path).unwrap();

    assert!(expected_singles.exists());
    assert!(expected_eps.exists());
    assert!(expected_albums.exists());

    assert!(
        read_dir(expected_singles)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap()
            .len()
            >= 15,
    );
    assert!(
        read_dir(expected_eps)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap()
            .len()
            >= 0,
    );
    assert!(
        read_dir(expected_albums)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap()
            .len()
            >= 8,
    );
}

#[allow(unused_comparisons)] // count might change when the artist uploads eps
#[tokio::test]
#[ignore = "takes very long"]
async fn test_channel_bundle_download_and_saving() {
    let downloaded = ChannelBrowse::new(
        IdCollection::from_url("https://music.youtube.com/channel/UCwp0yHVvrCeO2DBRmoxrRcQ")
            .unwrap()
            .get_id()
            .unwrap(),
        Downloader::testing(),
    )
    .await
    .unwrap()
    .browse()
    .await
    .unwrap()
    .download_bundle(&[AnyItag::Audio(AudioItag::AacLow), AnyItag::Audio(AudioItag::OpusLow)])
    .await
    .unwrap();

    assert!(downloaded.singles.len() >= 15);
    assert!(downloaded.eps.len() >= 0);
    assert!(downloaded.albums.len() >= 8);

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    let expected_singles = path.join("singles");
    let expected_eps = path.join("eps");
    let expected_albums = path.join("albums");

    downloaded.save(path).unwrap();

    assert!(expected_singles.exists());
    assert!(expected_eps.exists());
    assert!(expected_albums.exists());

    assert!(
        read_dir(expected_singles)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap()
            .len()
            >= 30,
    );
    assert!(
        read_dir(expected_eps)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap()
            .len()
            >= 0,
    );
    assert!(
        read_dir(expected_albums)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap()
            .len()
            >= 16,
    );
}

#[test]
fn test_creation_of_paths() {
    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    let expected_singles = path.join("singles");
    let expected_eps = path.join("eps");
    let expected_albums = path.join("albums");

    create_paths(&path).unwrap();

    assert!(expected_singles.exists());
    assert!(expected_eps.exists());
    assert!(expected_albums.exists());
}
