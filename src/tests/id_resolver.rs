use crate::{
    GetId, Id, IdCollection,
    id_types::{FastBrowseId, VideoId},
};

#[test]
fn playlist_urls() {
    let valid_urls = vec![
        "https://music.youtube.com/playlist?list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs",
        "https://music.youtube.com/watch?v=BGyHcS408as&list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs",
        "https://www.youtube.com/watch?v=BGyHcS408as&list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs",
        "https://www.youtube.com/watch?v=76jzZjtsHIc&list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs&index=1",
    ];

    let browse_id = FastBrowseId::new("OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs").unwrap();

    assert_eq!(browse_id.as_str(), "VLOLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs");

    for url in valid_urls.iter() {
        assert_eq!(GetId::<FastBrowseId>::get_id(&IdCollection::from_url(*url).unwrap()).unwrap(), browse_id);
    }
}

#[test]
fn video_urls() {
    let valid_urls = vec![
        "https://music.youtube.com/watch?v=S_v172PgToE&list=OLAK5uy_lnznEbydNUwGNsvtlTFBzjLpQeMsd1p80",
        "https://music.youtube.com/watch?v=S_v172PgToE",
        "https://www.youtube.com/watch?v=S_v172PgToE&list=OLAK5uy_lnznEbydNUwGNsvtlTFBzjLpQeMsd1p80",
        "https://www.youtube.com/watch?v=S_v172PgToE",
        "https://youtu.be/S_v172PgToE",
        "https://www.youtube.com/e/S_v172PgToE",
        "https://www.youtube.com/v/S_v172PgToE",
        "https://www.youtube.com/embed/S_v172PgToE",
    ];

    let video_id = VideoId::new("S_v172PgToE").unwrap();

    assert_eq!(video_id.as_str(), "S_v172PgToE");

    for url in valid_urls.iter() {
        assert_eq!(GetId::<VideoId>::get_id(&IdCollection::from_url(*url).unwrap()).unwrap(), video_id);
    }
}

#[test]
fn channel_urls() {}
