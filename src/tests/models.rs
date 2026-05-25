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
    request::core::{api_captcha_bypass, api_request},
};

#[tokio::test]
async fn browse_resp() {
    let id = FastBrowseId::new("OLAK5uy_lrCrcAdxFG4aMzMrebs7o9TU384xyF240").unwrap();
    let res: FastBrowseResponse = api_request(&id).await.unwrap();

    let expected_ids = vec![
        VideoId::new("CDko2ux1bkE").unwrap(),
        VideoId::new("_396y7Vk9NA").unwrap(),
        VideoId::new("0Qhjiz_Ceu4").unwrap(),
        VideoId::new("80LhtHY_fHw").unwrap(),
        VideoId::new("hOhTDRdcx4o").unwrap(),
        VideoId::new("q9NtWkikVAE").unwrap(),
        VideoId::new("5wth9BScmGY").unwrap(),
        VideoId::new("e3n4fRYTc94").unwrap(),
        VideoId::new("Rly0qaXeSPg").unwrap(),
        VideoId::new("4wYd-sqzY-k").unwrap(),
    ];

    assert_eq!(res.get_status(), Status::Success);
    assert_eq!(res.get_album_title().unwrap(), "Album - The Dark Side of the Moon");
    assert_eq!(res.get_video_ids().unwrap(), expected_ids);
}

#[tokio::test]
async fn channel_resp() {
    let id = ChannelId::new("MPADUC6Tg7GWjZw48EiZ8m5bRtWg").unwrap();
    let res: ChannelBrowseResponse = api_request(&id).await.unwrap();
    let expected_ids = ChannelContentBrowse {
        albums: vec![
            ChannelPlaylistId::new("MPREb_k6mvR8GoZj2").unwrap(),
            ChannelPlaylistId::new("MPREb_G8nEqIvDW9M").unwrap(),
            ChannelPlaylistId::new("MPREb_Vvj6JfTmMlY").unwrap(),
            ChannelPlaylistId::new("MPREb_dQoH7BxK35k").unwrap(),
        ],
        eps: vec![
            ChannelPlaylistId::new("MPREb_LFnPmBcUjuV").unwrap(),
            ChannelPlaylistId::new("MPREb_oISILrKA9sv").unwrap(),
            ChannelPlaylistId::new("MPREb_qILj8yFtGlq").unwrap(),
            ChannelPlaylistId::new("MPREb_1SUvJCAz0vy").unwrap(),
            ChannelPlaylistId::new("MPREb_4h5D508LokG").unwrap(),
            ChannelPlaylistId::new("MPREb_fQugXxg1l2u").unwrap(),
            ChannelPlaylistId::new("MPREb_jC5m5CzjH3o").unwrap(),
        ],
        singles: vec![
            ChannelPlaylistId::new("MPREb_ZFVkxH6MkHf").unwrap(),
            ChannelPlaylistId::new("MPREb_z8dMLQ5i9HY").unwrap(),
            ChannelPlaylistId::new("MPREb_BusmQgmK0Yh").unwrap(),
            ChannelPlaylistId::new("MPREb_IHoyHVbLKKG").unwrap(),
            ChannelPlaylistId::new("MPREb_cuIaNQtJI3Z").unwrap(),
            ChannelPlaylistId::new("MPREb_KsqJ2P7F1d0").unwrap(),
            ChannelPlaylistId::new("MPREb_zgJ4Kq5UzMC").unwrap(),
            ChannelPlaylistId::new("MPREb_nNAnQM89Rxr").unwrap(),
            ChannelPlaylistId::new("MPREb_fiwqfKaaKRI").unwrap(),
            ChannelPlaylistId::new("MPREb_HbFh4e2oHSk").unwrap(),
            ChannelPlaylistId::new("MPREb_od5skB7RY19").unwrap(),
            ChannelPlaylistId::new("MPREb_mYgzDZu6nqv").unwrap(),
            ChannelPlaylistId::new("MPREb_NbGPVXilPD9").unwrap(),
            ChannelPlaylistId::new("MPREb_BaBaA4tC861").unwrap(),
            ChannelPlaylistId::new("MPREb_wrLQj05uiBk").unwrap(),
            ChannelPlaylistId::new("MPREb_v9bdIQnK92M").unwrap(),
            ChannelPlaylistId::new("MPREb_GtzO679RJRW").unwrap(),
            ChannelPlaylistId::new("MPREb_m6zzAyZiA8S").unwrap(),
            ChannelPlaylistId::new("MPREb_M6nZfHk78uZ").unwrap(),
            ChannelPlaylistId::new("MPREb_ORPjpTHtnk4").unwrap(),
            ChannelPlaylistId::new("MPREb_pWCYd1AvRAL").unwrap(),
            ChannelPlaylistId::new("MPREb_LzWTfSzz5l6").unwrap(),
            ChannelPlaylistId::new("MPREb_89S15w0RHBR").unwrap(),
            ChannelPlaylistId::new("MPREb_QYBVwV5zYI9").unwrap(),
            ChannelPlaylistId::new("MPREb_Vaf8ebbfepC").unwrap(),
            ChannelPlaylistId::new("MPREb_6ViEwKHDvHJ").unwrap(),
            ChannelPlaylistId::new("MPREb_dqn3uTOkI4g").unwrap(),
            ChannelPlaylistId::new("MPREb_MrWowumy2JU").unwrap(),
            ChannelPlaylistId::new("MPREb_vFsaN2HuAYw").unwrap(),
            ChannelPlaylistId::new("MPREb_wwd7jqf2QGu").unwrap(),
            ChannelPlaylistId::new("MPREb_sbM3fQIDjZ3").unwrap(),
            ChannelPlaylistId::new("MPREb_WXKhcYS8fV0").unwrap(),
            ChannelPlaylistId::new("MPREb_WoFvBUvbdEU").unwrap(),
            ChannelPlaylistId::new("MPREb_SLwOn9T4l5D").unwrap(),
            ChannelPlaylistId::new("MPREb_r3Ot1WUGZDg").unwrap(),
            ChannelPlaylistId::new("MPREb_BHovHX7UAEk").unwrap(),
            ChannelPlaylistId::new("MPREb_rFBEWhTHwox").unwrap(),
            ChannelPlaylistId::new("MPREb_uSoejieVF4l").unwrap(),
            ChannelPlaylistId::new("MPREb_zRF1csQFowm").unwrap(),
            ChannelPlaylistId::new("MPREb_Hd2WyqR1nPa").unwrap(),
            ChannelPlaylistId::new("MPREb_6ET1ZlC9h6Y").unwrap(),
        ],
    };

    assert_eq!(res.get_status(), Status::Success);
    assert_eq!(res.extract_all_releases().unwrap(), expected_ids);
}

#[tokio::test]
async fn player_resp() {
    let id = VideoId::new("lndG8BiZCmM").unwrap();
    let res: PlayerResponse = api_captcha_bypass(&id, 2)
        .await
        .unwrap();

    assert_eq!(res.get_status(), Status::Success);

    let extr_res = res.extract().unwrap();

    assert_eq!(extr_res.metadata.title, "Damnation");
    assert_eq!(extr_res.metadata.author, "BLIND GUARDIAN - Topic");
}

#[tokio::test]
async fn slow_browse_resp() {
    let id = ChannelPlaylistId::new("MPREb_dQoH7BxK35k").unwrap();
    let res: SlowBrowseResponse = api_request(&id).await.unwrap();

    let expected_ids = vec![
        VideoId::new("7akaJCRoOYs").unwrap(),
        VideoId::new("vhhVL4bAYbA").unwrap(),
        VideoId::new("f6QFfmwFT3k").unwrap(),
        VideoId::new("KYfADOlstwc").unwrap(),
        VideoId::new("p-xsLux959Y").unwrap(),
        VideoId::new("3PfFcsO-sr0").unwrap(),
        VideoId::new("7cnxK99HC58").unwrap(),
        VideoId::new("qd_8mm906GA").unwrap(),
        VideoId::new("JvqXj2cAaY4").unwrap(),
        VideoId::new("wnyS_1agGqc").unwrap(),
        VideoId::new("2_OwXAzy3Z4").unwrap(),
        VideoId::new("EtEOl-xJTg8").unwrap(),
        VideoId::new("PhusP6wn5p0").unwrap(),
        VideoId::new("8eh6HEmBxLs").unwrap(),
        VideoId::new("reaYCEUyhw4").unwrap(),
    ];

    assert_eq!(res.get_status(), Status::Success);
    assert_eq!(res.get_album_title().unwrap(), "Hungry 5 (The Best of 5 Years)");
    assert_eq!(res.get_video_ids().unwrap(), expected_ids);
}
