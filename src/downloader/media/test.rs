// This file includes some double tests

use std::{fs, sync::Arc};

use bytes::Bytes;
use tokio::sync::Mutex;

use crate::{
    Downloader, DwnBundleMedia, DwnMedia, Id, ThumbRes,
    downloader::{
        media::{browse::MediaBrowse, util::extract_size},
        streams::{AudioStream, LongVideoStream, MediaStream},
    },
    itags::*,
    metadata::MediaMetadata,
    request::core::api_captcha_bypass,
    streams::{AnyStream, Thumbnail},
    types::VideoId,
};

#[tokio::test]
async fn test_browse_media() {
    let downloader = Downloader::testing();
    let id = VideoId::new("HPG7gYoqpHM").unwrap();
    let browsed = MediaBrowse::new(id)
        .browse(downloader)
        .await
        .unwrap();

    assert_eq!(browsed.metadata.title.as_str(), "Imaginations from the Other Side (Remastered 2007)");
    assert_eq!(browsed.metadata.author.as_str(), "BLIND GUARDIAN - Topic");
}

#[tokio::test]
async fn test_download_media_stream() {
    let downloader = Downloader::testing();
    let id = VideoId::new("HPG7gYoqpHM").unwrap();
    let downloaded = MediaBrowse::new(id)
        .browse(downloader)
        .await
        .unwrap()
        .download(AudioItag::AacLow, None)
        .await
        .unwrap();

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    let expected_path = path.join("imaginations-from-the-other-side-remastered-2007.m4a");
    downloaded
        .save_media_stream(&path)
        .unwrap();

    assert!(expected_path.exists())
}

#[tokio::test]
async fn test_download_media_streams() {
    let downloader = Downloader::testing();
    let id = VideoId::new("HPG7gYoqpHM").unwrap();
    let downloaded = MediaBrowse::new(id)
        .browse(downloader)
        .await
        .unwrap()
        .download_streams(vec![AnyItag::Audio(AudioItag::OpusLow), AnyItag::LongVideo(VideoItag::MP4144p)], Some(ThumbRes::Low))
        .await
        .unwrap();

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    let expected_stream_one_path = path.join("imaginations-from-the-other-side-remastered-2007.mp4");
    let expected_stream_two_path = path.join("imaginations-from-the-other-side-remastered-2007.webm");
    let expected_thumnmail_path = path.join("imaginations-from-the-other-side-remastered-2007.png");

    downloaded.save_full(&path).unwrap();

    assert!(expected_stream_one_path.exists());
    assert!(expected_stream_two_path.exists());
    assert!(expected_thumnmail_path.exists());
}

#[tokio::test]
async fn test_download_full_media() {
    let downloader = Downloader::testing();
    let id = VideoId::new("HPG7gYoqpHM").unwrap();
    let downloaded = MediaBrowse::new(id)
        .browse(downloader)
        .await
        .unwrap()
        .download(AudioItag::AacLow, Some(ThumbRes::Low))
        .await
        .unwrap();

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    let expected_stream_path = path.join("imaginations-from-the-other-side-remastered-2007.m4a");
    let expected_thumnmail_path = path.join("imaginations-from-the-other-side-remastered-2007.png");
    downloaded.save_full(&path).unwrap();

    assert!(expected_stream_path.exists());
    assert!(expected_thumnmail_path.exists());
}

#[tokio::test]
async fn test_extracted_streams() {
    let downloader = Downloader::testing();
    let id = VideoId::new("HPG7gYoqpHM").unwrap();
    let resp = api_captcha_bypass(&id, 5, &Arc::new(Mutex::new(None)), &downloader.client)
        .await
        .unwrap();

    let extr = resp.extract(downloader).unwrap();

    let best_stream = extr
        .media_streams
        .get_best_stream(&VideoItag::Highest)
        .unwrap();

    let stream_url = extr
        .media_streams
        .get_url_by_itag(&AudioItag::AacLow)
        .unwrap();

    let thumbnail_url = extr
        .thumbnail_streams
        .get_thumbnail_url_by_res(&ThumbRes::Low)
        .unwrap();

    assert_eq!(extract_size(best_stream).unwrap(), 20296497);
    assert_eq!(extract_size(stream_url).unwrap(), 2670210);
    assert!(extract_size(thumbnail_url).is_err());
}

#[test]
fn test_save_normal_media_full() {
    let metadata = MediaMetadata::new("a", "b");
    let thumbnail = Thumbnail::new(Bytes::from("rawr"));
    let mut stream = AudioStream::new(AudioItag::OpusLow);
    stream.push_data(Bytes::from("meow"));

    let media = DwnMedia::new(stream, metadata, Some(thumbnail));

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    let expected_stream_path = path.join("a.webm");
    let expected_thumbnail_path = path.join("a.png");

    media.save_full(path).unwrap();

    assert!(expected_stream_path.exists());
    assert!(expected_thumbnail_path.exists());

    assert_eq!(
        fs::read_to_string(expected_stream_path)
            .unwrap()
            .as_str(),
        "meow"
    );

    assert_eq!(
        fs::read_to_string(expected_thumbnail_path)
            .unwrap()
            .as_str(),
        "rawr"
    );
}

#[test]
fn test_save_normal_media_stream() {
    let metadata = MediaMetadata::new("a", "b");
    let mut stream = AudioStream::new(AudioItag::OpusLow);
    stream.push_data(Bytes::from("meow"));

    let media = DwnMedia::new(stream, metadata, None);

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    let expected_stream_path = path.join("a.webm");

    media.save_media_stream(path).unwrap();

    assert!(expected_stream_path.exists());

    assert_eq!(
        fs::read_to_string(expected_stream_path)
            .unwrap()
            .as_str(),
        "meow"
    );
}

#[test]
fn test_save_bundle_media_full() {
    let metadata = MediaMetadata::new("a", "b");
    let thumbnail = Thumbnail::new(Bytes::from("rawr"));

    let mut stream_one = AudioStream::new(AudioItag::OpusLow);
    stream_one.push_data(Bytes::from("meow1"));

    let mut stream_two = LongVideoStream::new(VideoItag::Webm144p);
    stream_two.push_data(Bytes::from("meow2"));

    let streams = vec![AnyStream::Audio(stream_one), AnyStream::LongVideo(stream_two)];
    let media = DwnBundleMedia::new(streams, metadata, Some(thumbnail));

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    let expected_stream_one_path = path.join("a.webm");
    let expected_stream_two_path = path.join("a-1.webm");
    let expected_thumbnail_path = path.join("a.png");

    media.save_full(path).unwrap();

    assert!(expected_stream_one_path.exists());
    assert!(expected_stream_two_path.exists());
    assert!(expected_thumbnail_path.exists());

    assert_eq!(
        fs::read_to_string(expected_stream_one_path)
            .unwrap()
            .as_str(),
        "meow1"
    );

    assert_eq!(
        fs::read_to_string(expected_stream_two_path)
            .unwrap()
            .as_str(),
        "meow2"
    );

    assert_eq!(
        fs::read_to_string(expected_thumbnail_path)
            .unwrap()
            .as_str(),
        "rawr"
    );
}

#[test]
fn test_save_bundle_media_streams() {
    let metadata = MediaMetadata::new("a", "b");

    let mut stream_one = AudioStream::new(AudioItag::OpusLow);
    stream_one.push_data(Bytes::from("meow1"));

    let mut stream_two = LongVideoStream::new(VideoItag::Webm144p);
    stream_two.push_data(Bytes::from("meow2"));

    let streams = vec![AnyStream::Audio(stream_one), AnyStream::LongVideo(stream_two)];
    let media = DwnBundleMedia::new(streams, metadata, None);

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path();

    let expected_stream_one_path = path.join("a.webm");
    let expected_stream_two_path = path.join("a-1.webm");

    media.save_media_streams(path).unwrap();

    assert!(expected_stream_one_path.exists());
    assert!(expected_stream_two_path.exists());

    assert_eq!(
        fs::read_to_string(expected_stream_one_path)
            .unwrap()
            .as_str(),
        "meow1"
    );

    assert_eq!(
        fs::read_to_string(expected_stream_two_path)
            .unwrap()
            .as_str(),
        "meow2"
    );
}

#[test]
fn test_merge_two_dwn_media() {
    let metadata = MediaMetadata::new("meow", "kitty");

    let stream_one = AudioStream::new(AudioItag::AacLow);
    let media_one = DwnMedia::new(stream_one, metadata.clone(), None);

    let stream_two = LongVideoStream::new(VideoItag::MP4240p);
    let media_two = DwnMedia::new(stream_two, metadata.clone(), None);

    let merged = DwnBundleMedia::from_dwn_media(media_one, media_two);

    assert_eq!(*merged.streams.get(0).unwrap(), AnyStream::Audio(AudioStream::new(AudioItag::AacLow)));
    assert_eq!(*merged.streams.get(1).unwrap(), AnyStream::LongVideo(LongVideoStream::new(VideoItag::MP4240p)));
    assert_eq!(merged.thumbnail, None);
    assert_eq!(merged.metadata, metadata);
}

#[test]
fn test_extracts_size_from_url() {
    let url = "https://example.com/videoplayback?expire=123&clen=4096&mime=video%2Fmp4";
    assert_eq!(extract_size(url).unwrap(), 4096);
}

#[test]
fn test_extracts_size_when_clen_is_last_param() {
    let url = "https://example.com/videoplayback?expire=123&clen=8192";
    assert_eq!(extract_size(url).unwrap(), 8192);
}

#[test]
fn test_errors_when_clen_is_missing() {
    let url = "https://example.com/videoplayback?expire=123&mime=video%2Fmp4";
    assert!(extract_size(url).is_err());
}

#[test]
fn test_errors_when_clen_is_not_a_number() {
    let url = "https://example.com/videoplayback?clen=abc&mime=video%2Fmp4";
    assert!(extract_size(url).is_err());
}
