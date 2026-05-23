use std::fs;

use bytes::Bytes;

use crate::downloader::media_stream::{AudioStream, MediaStream};

#[test]
fn save_audio_stream() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    let bytes = Bytes::from("meow");
    let expected_path = path.join("audio_stream_opus.webm");

    let mut audio_stream = AudioStream::new(crate::itag::AudioItag::OpusMedium);
    audio_stream.push_data(bytes.clone());
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
