use crate::{
    error::Result,
    id_resolver::{
        id::{GetId, Id},
        id_collection::IdCollection,
        id_types::playlist_id::FastBrowseId,
        id_types::video_id::VideoId,
    },
    models::{
        fast_browse::FastBrowseResponse,
        player::PlayerResponse,
        response::{BrowseResponse, Response, Status},
    },
    request::core::captcha_bypass,
};

#[test]
fn test_id_resolver() {
    let playlist_url = "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
    let mixed_url = "https://music.youtube.com/watch?v=lndG8BiZCmM&list=OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU";
    let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";

    let video_id_collecton = IdCollection::from_url(media_url);
    let playlist_id_collection = IdCollection::from_url(playlist_url);
    let mixed_id_collection = IdCollection::from_url(mixed_url);
    let invalid_id_collection = IdCollection::from_url("tehe");

    assert!(video_id_collecton.is_ok());
    assert!(playlist_id_collection.is_ok());
    assert!(mixed_id_collection.is_ok());
    assert!(invalid_id_collection.is_err());

    let video_id_collecton = video_id_collecton.unwrap();
    let playlist_id_collection = playlist_id_collection.unwrap();
    let mixed_id_collection = mixed_id_collection.unwrap();

    assert_eq!(video_id_collecton.get_id().ok(), Some(VideoId::new("lndG8BiZCmM")));
    assert_eq!(playlist_id_collection.get_id().ok(), Some(FastBrowseId::new("OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ")));
    assert_eq!(mixed_id_collection.get_id().ok(), Some(VideoId::new("lndG8BiZCmM")));
    assert_eq!(mixed_id_collection.get_id().ok(), Some(FastBrowseId::new("OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU")))
}

#[tokio::test]
async fn test_player_endpoint() {
    let video_id = VideoId::new("lndG8BiZCmM");
    let response: Result<PlayerResponse> = captcha_bypass(&video_id, 2).await;

    assert!(response.is_ok());

    let response = response.unwrap();

    //assert_eq!(response.get_title().to_lowercase(), "damnation");
    assert_eq!(response.get_status(), Status::Success);
}

#[tokio::test]
async fn test_browse_endpoint() {
    let browse_id = FastBrowseId::new("OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ");
    let response: Result<FastBrowseResponse> = captcha_bypass(&browse_id, 2).await;

    assert!(response.is_ok());

    let response = response.unwrap();
    let ids = response.get_video_ids();

    assert!(ids.is_ok());

    let ids = ids.unwrap().len();

    assert_eq!(ids, 20); // eh idk
}
