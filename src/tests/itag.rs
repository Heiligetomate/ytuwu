use crate::itag::{AudioItag, Itag, LongVideoItag, MuxedItag, ShortVideoItag};

#[test]
fn test_video_mime_type() {
    let video_webm = vec![
        LongVideoItag::WebM1080p,
        LongVideoItag::WebM720p,
        LongVideoItag::Webm480p,
        LongVideoItag::WebM360p,
        LongVideoItag::WebM240p,
        LongVideoItag::Webm144p,
    ];

    let video_mp4 = vec![
        LongVideoItag::MP41080p,
        LongVideoItag::MP4720p,
        LongVideoItag::MP4480p,
        LongVideoItag::MP4360p,
        LongVideoItag::MP4240p,
        LongVideoItag::MP4144p,
    ];

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
    let short_mp4 = vec![ShortVideoItag::High, ShortVideoItag::Low];

    for itag in short_mp4 {
        assert_eq!(itag.get_mime_type().as_str(), "mp4", "{:?} should have mp4 as ending", itag);
    }
}

#[test]
fn test_long_video_itag_to_int() {
    let cases = [
        (LongVideoItag::WebM1080p, 248),
        (LongVideoItag::MP41080p, 137),
        (LongVideoItag::WebM720p, 247),
        (LongVideoItag::MP4720p, 136),
        (LongVideoItag::Webm480p, 244),
        (LongVideoItag::MP4480p, 135),
        (LongVideoItag::WebM360p, 243),
        (LongVideoItag::MP4360p, 134),
        (LongVideoItag::WebM240p, 242),
        (LongVideoItag::MP4240p, 133),
        (LongVideoItag::Webm144p, 278),
        (LongVideoItag::MP4144p, 160),
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
    assert_eq!(ShortVideoItag::High.to_int(), 780);
    assert_eq!(ShortVideoItag::Low.to_int(), 779);
}

#[test]
fn test_muxed_itag_to_int() {
    assert_eq!(MuxedItag::MuxedMP4.to_int(), 18);
}

#[test]
fn test_long_video_next_best() {
    assert_eq!(
        LongVideoItag::WebM1080p
            .next_best()
            .unwrap(),
        LongVideoItag::MP41080p
    );
    assert_eq!(
        LongVideoItag::MP41080p
            .next_best()
            .unwrap(),
        LongVideoItag::WebM720p
    );
    assert_eq!(
        LongVideoItag::MP4144p
            .next_best()
            .is_err(),
        true
    );
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
    assert_eq!(
        ShortVideoItag::High
            .next_best()
            .unwrap(),
        ShortVideoItag::Low
    );
    assert_eq!(ShortVideoItag::Low.next_best().is_err(), true);
}

#[test]
fn test_muxed_next_best() {
    assert!(MuxedItag::MuxedMP4.next_best().is_err());
}

#[test]
fn test_highest() {
    assert_eq!(LongVideoItag::highest(), LongVideoItag::WebM1080p);
    assert_eq!(AudioItag::highest(), AudioItag::OpusMedium);
    assert_eq!(ShortVideoItag::highest(), ShortVideoItag::High);
    assert_eq!(MuxedItag::highest(), MuxedItag::MuxedMP4);
}
