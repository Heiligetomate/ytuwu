use crate::{
    browse_model::browse_response::BrowseResponse,
    error::Result,
    id_resolver::{self, BrowseId, GetId, Id, VideoId},
    player_model::player_response::PlayerResponse,
    request::shared::{Endpoint, captcha_bypass},
    shared_traits::{Response, Status},
};

#[test]
fn test_id_resolver() {
    let playlist_url =
        "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
    let mixed_url = "https://music.youtube.com/watch?v=lndG8BiZCmM&list=OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU";
    let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";

    let video_id_collecton = id_resolver::IdCollection::from_url(media_url);
    let playlist_id_collection = id_resolver::IdCollection::from_url(playlist_url);
    let mixed_id_collection = id_resolver::IdCollection::from_url(mixed_url);
    let invalid_id_collection = id_resolver::IdCollection::from_url("tehe");

    assert!(video_id_collecton.is_ok());
    assert!(playlist_id_collection.is_ok());
    assert!(mixed_id_collection.is_ok());
    assert!(invalid_id_collection.is_err());

    let video_id_collecton = video_id_collecton.unwrap();
    let playlist_id_collection = playlist_id_collection.unwrap();
    let mixed_id_collection = mixed_id_collection.unwrap();

    assert_eq!(
        video_id_collecton
            .get_id()
            .ok(),
        Some(VideoId::new("lndG8BiZCmM"))
    );
    assert_eq!(
        playlist_id_collection
            .get_id()
            .ok(),
        Some(BrowseId::new("OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ"))
    );
    assert_eq!(
        mixed_id_collection
            .get_id()
            .ok(),
        Some(VideoId::new("lndG8BiZCmM"))
    );
    assert_eq!(
        mixed_id_collection
            .get_id()
            .ok(),
        Some(BrowseId::new("OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU"))
    )
}

#[tokio::test]
async fn test_player_endpoint() {
    let video_id = VideoId::new("lndG8BiZCmM");
    let response: Result<PlayerResponse> = captcha_bypass(Endpoint::Player(video_id), 2).await;

    assert!(response.is_ok());

    let response = response.unwrap();

    //assert_eq!(response.get_title().to_lowercase(), "damnation");
    assert_eq!(response.get_status(), Status::Success);
}

#[tokio::test]
async fn test_browse_endpoint() {
    let browse_id = BrowseId::new("OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ");
    let response: Result<BrowseResponse> = captcha_bypass(Endpoint::Browse(browse_id), 2).await;

    assert!(response.is_ok());

    let response = response.unwrap();
    let ids = response.get_ids();

    assert!(ids.is_ok());

    let ids = ids.unwrap().len();

    assert_eq!(ids, 20); // eh idk
}
