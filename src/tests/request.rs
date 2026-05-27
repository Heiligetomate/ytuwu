use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    id_resolver::{
        id::Id,
        id_types::{ChannelId, ChannelPlaylistId, FastBrowseId, VideoId},
    },
    models::{
        channel_browse::ChannelBrowseResponse,
        fast_browse::FastBrowseResponse,
        player::PlayerResponse,
        response::{Response, Status},
        slow_browse::SlowBrowseResponse,
    },
    request::core::{api_captcha_bypass, api_request},
};

#[tokio::test]
async fn browse_client() {
    let id = FastBrowseId::new("OLAK5uy_lrCrcAdxFG4aMzMrebs7o9TU384xyF240").unwrap();
    let res: FastBrowseResponse = api_request(&id).await.unwrap();
    assert_eq!(res.get_status(), Status::Success);
}

#[tokio::test]
async fn channel_client() {
    let id = ChannelId::new("MPADUC6Tg7GWjZw48EiZ8m5bRtWg").unwrap();
    let res: ChannelBrowseResponse = api_request(&id).await.unwrap();
    assert_eq!(res.get_status(), Status::Success);
}

#[tokio::test]
async fn player_client() {
    let id = VideoId::new("lndG8BiZCmM").unwrap();
    let res: PlayerResponse = api_captcha_bypass(&id, 2, &Arc::new(Mutex::new(None)))
        .await
        .unwrap();
    assert_eq!(res.get_status(), Status::Success);
}

#[tokio::test]
async fn slow_browse_client() {
    let id = ChannelPlaylistId::new("MPREb_dQoH7BxK35k").unwrap();
    let res: SlowBrowseResponse = api_request(&id).await.unwrap();
    assert_eq!(res.get_status(), Status::Success);
}
