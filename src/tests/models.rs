use crate::{
    downloader::ChannelContentBrowse,
    id_resolver::{
        id::Id,
        id_types::{ChannelId, ChannelPlaylistId, FastBrowseId, VideoId},
    },
    models::{
        channel_browse::ChannelBrowseResponse,
        fast_browse::FastBrowseResponse,
        player::PlayerResponse,
        response::{BrowseResponse, Response, Status},
        slow_browse::SlowBrowseResponse,
    },
    request::core::captcha_bypass,
};

#[tokio::test]
async fn browse_resp() {
    let id = FastBrowseId::new("OLAK5uy_lrCrcAdxFG4aMzMrebs7o9TU384xyF240");
    let res: FastBrowseResponse = captcha_bypass(&id, 2).await.unwrap();

    let expected_ids = vec![
        VideoId::new("CDko2ux1bkE"),
        VideoId::new("_396y7Vk9NA"),
        VideoId::new("0Qhjiz_Ceu4"),
        VideoId::new("80LhtHY_fHw"),
        VideoId::new("hOhTDRdcx4o"),
        VideoId::new("q9NtWkikVAE"),
        VideoId::new("5wth9BScmGY"),
        VideoId::new("e3n4fRYTc94"),
        VideoId::new("Rly0qaXeSPg"),
        VideoId::new("4wYd-sqzY-k"),
    ];

    assert_eq!(res.get_status(), Status::Success);
    assert_eq!(res.get_album_title().unwrap(), "Album - The Dark Side of the Moon");
    assert_eq!(res.get_video_ids().unwrap(), expected_ids);
}

#[tokio::test]
async fn channel_resp() {
    let id = ChannelId::new("MPADUC6Tg7GWjZw48EiZ8m5bRtWg");
    let res: ChannelBrowseResponse = captcha_bypass(&id, 2).await.unwrap();
    let expected_ids = ChannelContentBrowse {
        albums: vec![
            ChannelPlaylistId::new("MPREb_k6mvR8GoZj2"),
            ChannelPlaylistId::new("MPREb_G8nEqIvDW9M"),
            ChannelPlaylistId::new("MPREb_Vvj6JfTmMlY"),
            ChannelPlaylistId::new("MPREb_dQoH7BxK35k"),
        ],
        eps: vec![
            ChannelPlaylistId::new("MPREb_LFnPmBcUjuV"),
            ChannelPlaylistId::new("MPREb_oISILrKA9sv"),
            ChannelPlaylistId::new("MPREb_qILj8yFtGlq"),
            ChannelPlaylistId::new("MPREb_1SUvJCAz0vy"),
            ChannelPlaylistId::new("MPREb_4h5D508LokG"),
            ChannelPlaylistId::new("MPREb_fQugXxg1l2u"),
            ChannelPlaylistId::new("MPREb_jC5m5CzjH3o"),
        ],
        singles: vec![
            ChannelPlaylistId::new("MPREb_ZFVkxH6MkHf"),
            ChannelPlaylistId::new("MPREb_z8dMLQ5i9HY"),
            ChannelPlaylistId::new("MPREb_BusmQgmK0Yh"),
            ChannelPlaylistId::new("MPREb_IHoyHVbLKKG"),
            ChannelPlaylistId::new("MPREb_cuIaNQtJI3Z"),
            ChannelPlaylistId::new("MPREb_KsqJ2P7F1d0"),
            ChannelPlaylistId::new("MPREb_zgJ4Kq5UzMC"),
            ChannelPlaylistId::new("MPREb_nNAnQM89Rxr"),
            ChannelPlaylistId::new("MPREb_fiwqfKaaKRI"),
            ChannelPlaylistId::new("MPREb_HbFh4e2oHSk"),
            ChannelPlaylistId::new("MPREb_od5skB7RY19"),
            ChannelPlaylistId::new("MPREb_mYgzDZu6nqv"),
            ChannelPlaylistId::new("MPREb_NbGPVXilPD9"),
            ChannelPlaylistId::new("MPREb_BaBaA4tC861"),
            ChannelPlaylistId::new("MPREb_wrLQj05uiBk"),
            ChannelPlaylistId::new("MPREb_v9bdIQnK92M"),
            ChannelPlaylistId::new("MPREb_GtzO679RJRW"),
            ChannelPlaylistId::new("MPREb_m6zzAyZiA8S"),
            ChannelPlaylistId::new("MPREb_M6nZfHk78uZ"),
            ChannelPlaylistId::new("MPREb_ORPjpTHtnk4"),
            ChannelPlaylistId::new("MPREb_pWCYd1AvRAL"),
            ChannelPlaylistId::new("MPREb_LzWTfSzz5l6"),
            ChannelPlaylistId::new("MPREb_89S15w0RHBR"),
            ChannelPlaylistId::new("MPREb_QYBVwV5zYI9"),
            ChannelPlaylistId::new("MPREb_Vaf8ebbfepC"),
            ChannelPlaylistId::new("MPREb_6ViEwKHDvHJ"),
            ChannelPlaylistId::new("MPREb_dqn3uTOkI4g"),
            ChannelPlaylistId::new("MPREb_MrWowumy2JU"),
            ChannelPlaylistId::new("MPREb_vFsaN2HuAYw"),
            ChannelPlaylistId::new("MPREb_wwd7jqf2QGu"),
            ChannelPlaylistId::new("MPREb_sbM3fQIDjZ3"),
            ChannelPlaylistId::new("MPREb_WXKhcYS8fV0"),
            ChannelPlaylistId::new("MPREb_WoFvBUvbdEU"),
            ChannelPlaylistId::new("MPREb_SLwOn9T4l5D"),
            ChannelPlaylistId::new("MPREb_r3Ot1WUGZDg"),
            ChannelPlaylistId::new("MPREb_BHovHX7UAEk"),
            ChannelPlaylistId::new("MPREb_rFBEWhTHwox"),
            ChannelPlaylistId::new("MPREb_uSoejieVF4l"),
            ChannelPlaylistId::new("MPREb_zRF1csQFowm"),
            ChannelPlaylistId::new("MPREb_Hd2WyqR1nPa"),
            ChannelPlaylistId::new("MPREb_6ET1ZlC9h6Y"),
        ],
    };

    assert_eq!(res.get_status(), Status::Success);
    assert_eq!(res.extract_all_releases().unwrap(), expected_ids);
}

#[tokio::test]
async fn player_resp() {
    let id = VideoId::new("lndG8BiZCmM");
    let res: PlayerResponse = captcha_bypass(&id, 2).await.unwrap();

    assert_eq!(res.get_status(), Status::Success);
    assert_eq!(res.get_title().unwrap(), "Damnation");
    assert_eq!(res.get_author().unwrap(), "BLIND GUARDIAN - Topic");
}

#[tokio::test]
async fn slow_browse_resp() {
    let id = ChannelPlaylistId::new("MPREb_dQoH7BxK35k");
    let res: SlowBrowseResponse = captcha_bypass(&id, 2).await.unwrap();

    let expected_ids = vec![
        VideoId::new("7akaJCRoOYs"),
        VideoId::new("vhhVL4bAYbA"),
        VideoId::new("f6QFfmwFT3k"),
        VideoId::new("KYfADOlstwc"),
        VideoId::new("p-xsLux959Y"),
        VideoId::new("3PfFcsO-sr0"),
        VideoId::new("7cnxK99HC58"),
        VideoId::new("qd_8mm906GA"),
        VideoId::new("JvqXj2cAaY4"),
        VideoId::new("wnyS_1agGqc"),
        VideoId::new("2_OwXAzy3Z4"),
        VideoId::new("EtEOl-xJTg8"),
        VideoId::new("PhusP6wn5p0"),
        VideoId::new("8eh6HEmBxLs"),
        VideoId::new("reaYCEUyhw4"),
    ];

    assert_eq!(res.get_status(), Status::Success);
    assert_eq!(res.get_album_title().unwrap(), "Hungry 5 (The Best of 5 Years)");
    assert_eq!(res.get_video_ids().unwrap(), expected_ids);
}
