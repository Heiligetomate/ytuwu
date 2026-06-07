use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    Downloader,
    id_resolver::{
        id::Id,
        types::{AlbumId, ChannelId, ChannelPlaylistId, VideoId},
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
    let id = AlbumId::new("OLAK5uy_lrCrcAdxFG4aMzMrebs7o9TU384xyF240").unwrap();
    let res: FastBrowseResponse = api_request(&id, &reqwest::Client::new())
        .await
        .unwrap();

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
    let res: ChannelBrowseResponse = api_request(&id, &reqwest::Client::new())
        .await
        .unwrap();
    let extracted = res
        .extract_all_releases(Downloader::testing(), Uuid::new_v4())
        .unwrap();
    assert_eq!(res.get_status(), Status::Success);
    assert!(extracted.singles.len() >= 40);
    assert!(extracted.eps.len() >= 7);
    assert!(extracted.albums.len() >= 4);
}

#[tokio::test]
async fn player_resp() {
    let id = VideoId::new("lndG8BiZCmM").unwrap();
    let res: PlayerResponse = api_captcha_bypass(&id, 2, &Arc::new(Mutex::new(None)), &reqwest::Client::new())
        .await
        .unwrap();

    assert_eq!(res.get_status(), Status::Success);

    let extr_res = res
        .extract(Downloader::testing())
        .unwrap();

    assert_eq!(extr_res.metadata.title, "Damnation");
    assert_eq!(extr_res.metadata.author, "BLIND GUARDIAN - Topic");
}

#[tokio::test]
async fn slow_browse_resp() {
    let id = ChannelPlaylistId::new("MPREb_dQoH7BxK35k").unwrap();
    let res: SlowBrowseResponse = api_request(&id, &reqwest::Client::new())
        .await
        .unwrap();

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
