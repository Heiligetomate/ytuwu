use std::fs;

use bytes::Bytes;

use crate::{
    DwnBundleMedia, DwnMedia,
    downloader::streams::{AudioStream, LongVideoStream, MediaStream, MuxedStream, ShortVideoStream},
    itags::*,
    metadata::MediaMetadata,
    streams::{AnyStream, Thumbnail},
};

#[test]
fn test_save_audio_stream() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    let bytes = Bytes::from("meow");
    let expected_path = path.join("audio_stream_opus.webm");

    let mut audio_stream = AudioStream::new(AudioItag::OpusMedium);
    audio_stream.push_data(bytes);
    audio_stream
        .save(path, "audio_stream_opus")
        .unwrap();
    assert!(expected_path.exists());
    assert_eq!(
        fs::read_to_string(expected_path)
            .unwrap()
            .as_str(),
        "meow"
    );
}

#[test]
fn test_save_long_video_stream() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    let bytes = Bytes::from("meow");
    let expected_path = path.join("long_video_stream.webm");
    let mut stream = LongVideoStream::new(VideoItag::WebM1080p);
    stream.push_data(bytes);
    stream
        .save(path, "long_video_stream")
        .unwrap();
    assert!(expected_path.exists());
    assert_eq!(
        fs::read_to_string(expected_path)
            .unwrap()
            .as_str(),
        "meow"
    );
}

#[test]
fn test_save_short_video_stream() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    let bytes = Bytes::from("meow");
    let expected_path = path.join("short_video_stream.mp4");
    let mut stream = ShortVideoStream::new(ShortItag::High);
    stream.push_data(bytes);
    stream
        .save(path, "short_video_stream")
        .unwrap();
    assert!(expected_path.exists());
    assert_eq!(
        fs::read_to_string(expected_path)
            .unwrap()
            .as_str(),
        "meow"
    );
}

#[test]
fn test_save_muxed_stream() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    let bytes = Bytes::from("meow");
    let expected_path = path.join("muxed_stream.mp4");
    let mut stream = MuxedStream::new(MuxedItag::MuxedMP4);
    stream.push_data(bytes);
    stream
        .save(path, "muxed_stream")
        .unwrap();
    assert!(expected_path.exists());
    assert_eq!(
        fs::read_to_string(expected_path)
            .unwrap()
            .as_str(),
        "meow"
    );
}

#[test]
fn test_save_thumbnail() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    let bytes = Bytes::from("meow");
    let expected_path = path.join("thumbnail.png");
    let stream = Thumbnail::new(bytes);
    stream.save(path, "thumbnail").unwrap();
    assert!(expected_path.exists());
    assert_eq!(
        fs::read_to_string(expected_path)
            .unwrap()
            .as_str(),
        "meow"
    );
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
