#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{id_resolver::{self, BrowseId, Id, VideoId}, player_model::player_response::PlayerResponse, request::{shared::{Endpoint, captcha_bypass}}, shared_traits::{Response, Status}};

    #[test]
    fn test_id_resolver() {
        let playlist_url = "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
        let mixed_url = "https://music.youtube.com/watch?v=lndG8BiZCmM&list=OLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU";
        let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";

        let video_id_collecton = id_resolver::IdCollection::from_url(media_url);
        let playlist_id_collection = id_resolver::IdCollection::from_url(playlist_url);
        let mixed_id_collection = id_resolver::IdCollection::from_url(mixed_url);
        let invalid_id_collection = id_resolver::IdCollection::from_url("tehe");
        
        assert!(video_id_collecton.is_some());
        assert!(playlist_id_collection.is_some());
        assert!(mixed_id_collection.is_some());
        assert!(invalid_id_collection.is_none());
        
        let video_id_collecton = video_id_collecton.unwrap();
        let playlist_id_collection = playlist_id_collection.unwrap();
        let mixed_id_collection = mixed_id_collection.unwrap();

        assert_eq!(video_id_collecton.get_browse_id(), None);
        assert_eq!(video_id_collecton.get_video_id(), Some(&VideoId::new("lndG8BiZCmM")));
        assert_eq!(playlist_id_collection.get_video_id(), None);
        assert_eq!(playlist_id_collection.get_browse_id(), Some(&BrowseId::new("VLOLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ")));
        assert_eq!(mixed_id_collection.get_video_id(), Some(&VideoId::new("lndG8BiZCmM")));
        assert_eq!(mixed_id_collection.get_browse_id(), Some(&BrowseId::new("VLOLAK5uy_mrUmnJrX4QzJd6GeOuqcqT8EUMH1C0eTU")))            
    }
 
    #[tokio::test]
    async fn test_player_endpoint() {
        let video_id = VideoId::new("lndG8BiZCmM");
        let response: Result<PlayerResponse> = captcha_bypass(Endpoint::Player(video_id), 2).await;
        
        assert!(response.is_ok());

        let response = response.unwrap();
        
        assert_eq!(response.get_title().to_lowercase(), "damnation");
        assert_eq!(response.get_status(), Status::Success);
    }
}
