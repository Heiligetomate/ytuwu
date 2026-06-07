use std::sync::Arc;

use crate::{
    Downloader, Id, ThumbRes,
    downloader::{media::browse::MediaBrowse, playlist::browse::PlaylistBrowse},
    itags::{AnyItag, AudioItag, VideoItag},
    types::{AlbumId, BrowseId, VideoId},
};

#[tokio::test]
async fn test_playlist_browse_request() {
    let id = AlbumId::new("OLAK5uy_nmq4-rfcWad4OIuBpBnZxpXjeg8Fx9MvA").unwrap();
    let invalid_id = AlbumId::new("OLAK5uy_nmq4-AGWWad4OIuBpBnZxpXjeg8Fx9MvA").unwrap();

    let downloader = Downloader::testing();

    assert!(
        PlaylistBrowse::new(BrowseId::AlbumId(id), Arc::clone(&downloader))
            .browse()
            .await
            .is_ok()
    );

    assert!(
        PlaylistBrowse::new(BrowseId::AlbumId(invalid_id), downloader)
            .browse()
            .await
            .is_err()
    )
}

#[tokio::test]
async fn test_playlist_content_browse() {
    let id = AlbumId::new("OLAK5uy_nmq4-rfcWad4OIuBpBnZxpXjeg8Fx9MvA").unwrap();

    let downloader = Downloader::testing();

    let browsed = PlaylistBrowse::new(BrowseId::AlbumId(id), downloader)
        .browse()
        .await
        .unwrap();

    assert_eq!(browsed.title.as_str(), "album-amen");
    assert_eq!(
        browsed.media,
        vec![
            MediaBrowse::new(VideoId::new("535-Bxyf_OY").unwrap()),
            MediaBrowse::new(VideoId::new("5gM58GXt6mw").unwrap()),
            MediaBrowse::new(VideoId::new("gB25L_TZ2JI").unwrap()),
            MediaBrowse::new(VideoId::new("lwZ5kD2XyQ4").unwrap()),
            MediaBrowse::new(VideoId::new("iE-gWOoQwCA").unwrap()),
            MediaBrowse::new(VideoId::new("wRG9RUhk04c").unwrap()),
            MediaBrowse::new(VideoId::new("0TlhLb4MnbQ").unwrap()),
            MediaBrowse::new(VideoId::new("F0Tq8RAIXpA").unwrap()),
            MediaBrowse::new(VideoId::new("06wKiEOItfM").unwrap()),
            MediaBrowse::new(VideoId::new("O9g7RUOYbGM").unwrap()),
            MediaBrowse::new(VideoId::new("A_fXr89fAfk").unwrap()),
            MediaBrowse::new(VideoId::new("78efw-z8tc0").unwrap()),
        ]
    )
}

#[tokio::test]
async fn test_playlist_content_browse_request() {
    let id = AlbumId::new("OLAK5uy_nmq4-rfcWad4OIuBpBnZxpXjeg8Fx9MvA").unwrap();
    let invalid_id = AlbumId::new("OLAK5uy_nmq4-AGWWad4OIuBpBnZxpXjeg8Fx9MvA").unwrap();

    let downloader = Downloader::testing();

    assert!(
        PlaylistBrowse::new(BrowseId::AlbumId(id), Arc::clone(&downloader))
            .browse()
            .await
            .unwrap()
            .browse()
            .await
            .is_ok()
    );

    assert!(
        PlaylistBrowse::new(BrowseId::AlbumId(invalid_id), downloader)
            .browse()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_playlist() {
    let id = AlbumId::new("OLAK5uy_nmq4-rfcWad4OIuBpBnZxpXjeg8Fx9MvA").unwrap();

    let downloader = Downloader::testing();

    let browsed = PlaylistBrowse::new(BrowseId::AlbumId(id), downloader)
        .browse()
        .await
        .unwrap()
        .browse()
        .await
        .unwrap();

    assert_eq!(
        browsed.get_titles(),
        vec![
            "Daemoni",
            "Headbutt",
            "Limbo",
            "Blastbeat Falafel",
            "ADHD",
            "2020",
            "Mustard Mucous",
            "Infestis",
            "Ancient Sun",
            "Pure Disproportionate Black and White Nihilism",
            "E\u{301}tude n°120",
            "Silence"
        ]
    );

    assert_eq!(
        browsed
            .get_first()
            .unwrap()
            .metadata
            .title,
        "Daemoni"
    );
}

#[tokio::test]
#[ignore = "takes long"]
async fn test_download_playlist_stream() {
    let id = AlbumId::new("OLAK5uy_nmq4-rfcWad4OIuBpBnZxpXjeg8Fx9MvA").unwrap();

    let downloader = Downloader::testing();

    let downloaded = PlaylistBrowse::new(BrowseId::AlbumId(id), downloader)
        .browse()
        .await
        .unwrap()
        .browse()
        .await
        .unwrap()
        .download(AudioItag::AacLow, None)
        .await
        .unwrap();

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    let expected_paths = vec![
        path.join("silence.m4a"),
        path.join("e\u{301}tude-n°120.m4a"),
        path.join("pure-disproportionate-black-and-white-nihilism.m4a"),
        path.join("ancient-sun.m4a"),
        path.join("infestis.m4a"),
        path.join("mustard-mucous.m4a"),
        path.join("2020.m4a"),
        path.join("adhd.m4a"),
        path.join("blastbeat-falafel.m4a"),
        path.join("limbo.m4a"),
        path.join("headbutt.m4a"),
        path.join("daemoni.m4a"),
    ];

    downloaded.save(&path).unwrap();

    for path in expected_paths {
        assert!(path.exists())
    }
}

#[tokio::test]
#[ignore = "takes long"]
async fn test_download_playlist_streams() {
    let id = AlbumId::new("OLAK5uy_nmq4-rfcWad4OIuBpBnZxpXjeg8Fx9MvA").unwrap();

    let downloader = Downloader::testing();

    let downloaded = PlaylistBrowse::new(BrowseId::AlbumId(id), downloader)
        .browse()
        .await
        .unwrap()
        .browse()
        .await
        .unwrap()
        .download_bundle(&[AnyItag::Audio(AudioItag::AacLow), AnyItag::Video(VideoItag::MP4144p)], Some(ThumbRes::Low))
        .await
        .unwrap();

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    downloaded.save(&path).unwrap();

    let expected_paths = vec![
        path.join("silence.m4a"),
        path.join("e\u{301}tude-n°120.m4a"),
        path.join("pure-disproportionate-black-and-white-nihilism.m4a"),
        path.join("ancient-sun.m4a"),
        path.join("infestis.m4a"),
        path.join("mustard-mucous.m4a"),
        path.join("2020.m4a"),
        path.join("adhd.m4a"),
        path.join("blastbeat-falafel.m4a"),
        path.join("limbo.m4a"),
        path.join("headbutt.m4a"),
        path.join("daemoni.m4a"),
        path.join("silence.mp4"),
        path.join("e\u{301}tude-n°120.mp4"),
        path.join("pure-disproportionate-black-and-white-nihilism.mp4"),
        path.join("ancient-sun.mp4"),
        path.join("infestis.mp4"),
        path.join("mustard-mucous.mp4"),
        path.join("2020.mp4"),
        path.join("adhd.mp4"),
        path.join("blastbeat-falafel.mp4"),
        path.join("limbo.mp4"),
        path.join("headbutt.mp4"),
        path.join("daemoni.mp4"),
    ];

    for path in expected_paths {
        assert!(path.exists())
    }
}
