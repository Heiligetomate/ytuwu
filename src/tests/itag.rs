use crate::itag::{AudioItag, Itag, MuxedItag, ShortItag, VideoItag};

#[test]
fn test_video_mime_type() {
    let video_webm = vec![
        VideoItag::WebM1080p,
        VideoItag::WebM720p,
        VideoItag::Webm480p,
        VideoItag::WebM360p,
        VideoItag::WebM240p,
        VideoItag::Webm144p,
    ];

    let video_mp4 = vec![VideoItag::MP41080p, VideoItag::MP4720p, VideoItag::MP4480p, VideoItag::MP4360p, VideoItag::MP4240p, VideoItag::MP4144p];

    for itag in video_webm.iter() {
        assert_eq!(itag.get_mime_type().as_str(), "webm", "{:?} should have webm as ending", itag);
    }

    for itag in video_mp4 {
        assert_eq!(itag.get_mime_type().as_str(), "mp4", "{:?} should have mp4 as ending", itag);
    }
}

#[test]
fn test_audio_mime_type() {
    let audio_webm = vec![AudioItag::OpusMedium, AudioItag::OpusLow];

    let audio_m4a = vec![AudioItag::AacMedium, AudioItag::AacLow];

    for itag in audio_webm {
        assert_eq!(itag.get_mime_type().as_str(), "webm", "{:?} should have webm as ending", itag);
    }

    for itag in audio_m4a {
        assert_eq!(itag.get_mime_type().as_str(), "m4a", "{:?} should have m4a as ending", itag);
    }
}

#[test]
fn test_muxed_mime_type() {
    let itag = MuxedItag::MuxedMP4;
    assert_eq!(itag.get_mime_type().as_str(), "mp4", "{:?} shoud have mp4 as ending", itag);
}
fn test_short_mime_type() {
    let short_mp4 = vec![ShortItag::High, ShortItag::Low];

    for itag in short_mp4 {
        assert_eq!(itag.get_mime_type().as_str(), "mp4", "{:?} should have mp4 as ending", itag);
    }
}

#[test]
fn test_long_video_itag_to_int() {
    let cases = [
        (VideoItag::WebM1080p, 248),
        (VideoItag::MP41080p, 137),
        (VideoItag::WebM720p, 247),
        (VideoItag::MP4720p, 136),
        (VideoItag::Webm480p, 244),
        (VideoItag::MP4480p, 135),
        (VideoItag::WebM360p, 243),
        (VideoItag::MP4360p, 134),
        (VideoItag::WebM240p, 242),
        (VideoItag::MP4240p, 133),
        (VideoItag::Webm144p, 278),
        (VideoItag::MP4144p, 160),
    ];
    for (itag, expected) in cases {
        assert_eq!(itag.to_int(), expected, "{:?} should be {}", itag, expected);
    }
}

#[test]
fn test_audio_itag_to_int() {
    let cases = [(AudioItag::OpusMedium, 251), (AudioItag::OpusLow, 249), (AudioItag::AacMedium, 140), (AudioItag::AacLow, 139)];
    for (itag, expected) in cases {
        assert_eq!(itag.to_int(), expected, "{:?} should be {}", itag, expected);
    }
}

#[test]
fn test_short_video_itag_to_int() {
    assert_eq!(ShortItag::High.to_int(), 780);
    assert_eq!(ShortItag::Low.to_int(), 779);
}

#[test]
fn test_muxed_itag_to_int() {
    assert_eq!(MuxedItag::MuxedMP4.to_int(), 18);
}

#[test]
fn test_long_video_next_best() {
    assert_eq!(
        VideoItag::WebM1080p
            .next_best()
            .unwrap(),
        VideoItag::MP41080p
    );
    assert_eq!(VideoItag::MP41080p.next_best().unwrap(), VideoItag::WebM720p);
    assert_eq!(VideoItag::MP4144p.next_best().is_err(), true);
}

#[test]
fn test_audio_next_best() {
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
    assert_eq!(AudioItag::AacLow.next_best().is_err(), true);
}

#[test]
fn test_short_video_next_best() {
    assert_eq!(ShortItag::High.next_best().unwrap(), ShortItag::Low);
    assert_eq!(ShortItag::Low.next_best().is_err(), true);
}

#[test]
fn test_muxed_next_best() {
    assert!(MuxedItag::MuxedMP4.next_best().is_err());
}

#[test]
fn test_highest() {
    assert_eq!(VideoItag::highest(), VideoItag::WebM1080p);
    assert_eq!(AudioItag::highest(), AudioItag::OpusMedium);
    assert_eq!(ShortItag::highest(), ShortItag::High);
    assert_eq!(MuxedItag::highest(), MuxedItag::MuxedMP4);
}
