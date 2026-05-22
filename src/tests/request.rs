use crate::{
    id_resolver::{
        id::Id,
        id_types::{channel_id::ChannelId, channel_playlist_id::ChannelPlaylistId, playlist_id::FastBrowseId, video_id::VideoId},
    },
    models::{
        channel_browse::ChannelBrowseResponse,
        fast_browse::FastBrowseResponse,
        player::PlayerResponse,
        response::{Response, Status},
        slow_browse::SlowBrowseResponse,
    },
    request::core::captcha_bypass,
};

#[tokio::test]
async fn browse_client() {
    let id = FastBrowseId::new("OLAK5uy_lrCrcAdxFG4aMzMrebs7o9TU384xyF240");
    let res: FastBrowseResponse = captcha_bypass(&id, 2).await.unwrap();
    assert_eq!(res.get_status(), Status::Success);
}

#[tokio::test]
async fn channel_client() {
    let id = ChannelId::new("MPADUC6Tg7GWjZw48EiZ8m5bRtWg");
    let res: ChannelBrowseResponse = captcha_bypass(&id, 2).await.unwrap();
    assert_eq!(res.get_status(), Status::Success);
}

#[tokio::test]
async fn player_client() {
    let id = VideoId::new("lndG8BiZCmM");
    let res: PlayerResponse = captcha_bypass(&id, 2).await.unwrap();
    assert_eq!(res.get_status(), Status::Success);
}

#[tokio::test]
async fn slow_browse_client() {
    let id = ChannelPlaylistId::new("MPREb_dQoH7BxK35k");
    let res: SlowBrowseResponse = captcha_bypass(&id, 2).await.unwrap();
    assert_eq!(res.get_status(), Status::Success);
}
