use crate::{
    GetId, Id, IdCollection,
    id_types::{FastBrowseId, VideoId},
};

#[test]
fn playlist_urls() {
    let music_com_pure = "https://music.youtube.com/playlist?list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs";
    let music_com_with_video_elem = "https://music.youtube.com/watch?v=BGyHcS408as&list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs";
    let youtube_pure = "https://www.youtube.com/watch?v=BGyHcS408as&list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs";
    let youtube_with_video_elem = "https://www.youtube.com/watch?v=76jzZjtsHIc&list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs&index=1";

    let browse_id = FastBrowseId::new("OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs").unwrap();

    assert_eq!(browse_id.as_str(), "VLOLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs");

    assert_eq!(GetId::<FastBrowseId>::get_id(&IdCollection::from_url(music_com_pure).unwrap()).unwrap(), browse_id);
    assert_eq!(GetId::<FastBrowseId>::get_id(&IdCollection::from_url(music_com_with_video_elem).unwrap()).unwrap(), browse_id);
    assert_eq!(GetId::<FastBrowseId>::get_id(&IdCollection::from_url(youtube_pure).unwrap()).unwrap(), browse_id);
    assert_eq!(GetId::<FastBrowseId>::get_id(&IdCollection::from_url(youtube_with_video_elem).unwrap()).unwrap(), browse_id);
}

#[test]
fn video_urls() {
    let music_com_pure = "https://music.youtube.com/watch?v=S_v172PgToE&list=OLAK5uy_lnznEbydNUwGNsvtlTFBzjLpQeMsd1p80";
    let music_com_with_video_elem = "https://music.youtube.com/watch?v=S_v172PgToE";
    let youtube_pure = "https://www.youtube.com/watch?v=S_v172PgToE&list=OLAK5uy_lnznEbydNUwGNsvtlTFBzjLpQeMsd1p80";
    let youtube_with_video_elem = "https://www.youtube.com/watch?v=S_v172PgToE";
    let youtu_be = "https://youtu.be/S_v172PgToE";
    let youtube_e_param = "https://www.youtube.com/e/S_v172PgToE";
    let youtube_v_param = "https://www.youtube.com/v/S_v172PgToE";
    let youtube_embed = "https://www.youtube.com/embed/S_v172PgToE";

    let video_id = VideoId::new("S_v172PgToE").unwrap();

    assert_eq!(video_id.as_str(), "S_v172PgToE");

    assert_eq!(GetId::<VideoId>::get_id(&IdCollection::from_url(music_com_pure).unwrap()).unwrap(), video_id);
    assert_eq!(GetId::<VideoId>::get_id(&IdCollection::from_url(music_com_with_video_elem).unwrap()).unwrap(), video_id);
    assert_eq!(GetId::<VideoId>::get_id(&IdCollection::from_url(youtube_pure).unwrap()).unwrap(), video_id);
    assert_eq!(GetId::<VideoId>::get_id(&IdCollection::from_url(youtube_with_video_elem).unwrap()).unwrap(), video_id);
    assert_eq!(GetId::<VideoId>::get_id(&IdCollection::from_url(youtu_be).unwrap()).unwrap(), video_id);
    assert_eq!(GetId::<VideoId>::get_id(&IdCollection::from_url(youtube_e_param).unwrap()).unwrap(), video_id);
    assert_eq!(GetId::<VideoId>::get_id(&IdCollection::from_url(youtube_v_param).unwrap()).unwrap(), video_id);
    assert_eq!(GetId::<VideoId>::get_id(&IdCollection::from_url(youtube_embed).unwrap()).unwrap(), video_id);
}

#[test]
fn channel_urls() {}
