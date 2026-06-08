use crate::{
    downloader::mime_types::MimeType,
    itags::{AudioItag, Itag, MuxedItag, ShortItag, VideoItag},
};

#[test]
fn test_highest_itags() {
    assert_eq!(AudioItag::highest(), AudioItag::OpusMedium);
    assert_eq!(MuxedItag::highest(), MuxedItag::MuxedMP4);
    assert_eq!(ShortItag::highest(), ShortItag::High);
    assert_eq!(VideoItag::highest(), VideoItag::WebM1080p);
}

#[test]
fn test_is_highest_itag() {
    assert!(AudioItag::Highest.is_highest());
    assert!(MuxedItag::Highest.is_highest());
    assert!(ShortItag::Highest.is_highest());
    assert!(VideoItag::Highest.is_highest());
}

#[test]
fn test_itag_mime_types() {
    assert_eq!(AudioItag::OpusMedium.get_mime_type(), MimeType::Webm);
    assert_eq!(AudioItag::AacMedium.get_mime_type(), MimeType::M4A);
    assert_eq!(MuxedItag::MuxedMP4.get_mime_type(), MimeType::MP4);
    assert_eq!(ShortItag::High.get_mime_type(), MimeType::MP4);
    assert_eq!(VideoItag::WebM1080p.get_mime_type(), MimeType::Webm);
    assert_eq!(VideoItag::MP41080p.get_mime_type(), MimeType::MP4);
}

#[test]
fn test_video_itag_next_best() {
    assert_eq!(VideoItag::Highest.next_best().unwrap(), VideoItag::WebM1080p);
    assert_eq!(
        VideoItag::WebM1080p
            .next_best()
            .unwrap(),
        VideoItag::MP41080p
    );
    assert_eq!(VideoItag::MP41080p.next_best().unwrap(), VideoItag::Webm1080p50);
    assert_eq!(
        VideoItag::Webm1080p50
            .next_best()
            .unwrap(),
        VideoItag::MP41080p50
    );
    assert_eq!(
        VideoItag::MP41080p50
            .next_best()
            .unwrap(),
        VideoItag::WebM720p
    );
    assert_eq!(VideoItag::WebM720p.next_best().unwrap(), VideoItag::MP4720p);
    assert_eq!(VideoItag::MP4720p.next_best().unwrap(), VideoItag::Webm720p50);
    assert_eq!(
        VideoItag::Webm720p50
            .next_best()
            .unwrap(),
        VideoItag::MP4720p50
    );
    assert_eq!(
        VideoItag::MP4720p50
            .next_best()
            .unwrap(),
        VideoItag::Webm480p
    );
    assert_eq!(VideoItag::Webm480p.next_best().unwrap(), VideoItag::MP4480p);
    assert_eq!(VideoItag::MP4480p.next_best().unwrap(), VideoItag::WebM360p);
    assert_eq!(VideoItag::WebM360p.next_best().unwrap(), VideoItag::MP4360p);
    assert_eq!(VideoItag::MP4360p.next_best().unwrap(), VideoItag::WebM240p);
    assert_eq!(VideoItag::WebM240p.next_best().unwrap(), VideoItag::MP4240p);
    assert_eq!(VideoItag::MP4240p.next_best().unwrap(), VideoItag::Webm144p);
    assert_eq!(VideoItag::Webm144p.next_best().unwrap(), VideoItag::MP4144p);
    assert!(VideoItag::MP4144p.next_best().is_err());
}

#[test]
fn test_audio_itag_next_best() {
    assert_eq!(AudioItag::Highest.next_best().unwrap(), AudioItag::OpusMedium);
    assert_eq!(
        AudioItag::OpusMedium
            .next_best()
            .unwrap(),
        AudioItag::AacMedium
    );
    assert_eq!(
        AudioItag::AacMedium
            .next_best()
            .unwrap(),
        AudioItag::OpusLow
    );
    assert_eq!(AudioItag::OpusLow.next_best().unwrap(), AudioItag::AacLow);
    assert!(AudioItag::AacLow.next_best().is_err());
}

#[test]
fn test_muxed_itag_next_best() {
    assert!(MuxedItag::MuxedMP4.next_best().is_err())
}

#[test]
fn test_short_video_itag_next_best() {
    assert_eq!(ShortItag::High.next_best().unwrap(), ShortItag::Low);
    assert!(ShortItag::Low.next_best().is_err());
}
