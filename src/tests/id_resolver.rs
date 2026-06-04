use crate::{
    GetId, Id, IdCollection,
    id_resolver::browse_id::BrowseId,
    types::{AlbumId, ChannelId, PlaylistId, VideoId},
};

#[test]
fn playlist_urls() {
    let valid = "RDCLAK5uy_mDr4Gy1V6eV55fj8oQ24KLSUrY9u2QKWA";

    let playlist_id = PlaylistId::new(valid).unwrap();

    assert_eq!(playlist_id.as_str(), "VLRDCLAK5uy_mDr4Gy1V6eV55fj8oQ24KLSUrY9u2QKWA");
}

#[test]
fn album_urls() {
    let valid_urls = vec![
        "https://music.youtube.com/playlist?list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs",
        "https://music.youtube.com/watch?v=BGyHcS408as&list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs",
        "https://www.youtube.com/watch?v=BGyHcS408as&list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs",
        "https://www.youtube.com/watch?v=76jzZjtsHIc&list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs&index=1",
        "https://www.youtube.com/playlist?list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs",
        "https://youtube.com/playlist?list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs",
        "https://m.youtube.com/playlist?list=OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs",
    ];

    let album_id = BrowseId::AlbumId(AlbumId::new("OLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs").unwrap());

    assert_eq!(album_id.as_str(), "VLOLAK5uy_kmI7lE04T73fi905AhF6ml8E4WShlKfNs");

    for url in valid_urls.iter() {
        assert_eq!(GetId::<BrowseId>::get_id(&IdCollection::from_url(*url).unwrap()).unwrap(), album_id);
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
        "https://youtube.com/watch?v=S_v172PgToE",
        "https://m.youtube.com/watch?v=S_v172PgToE",
        "https://youtu.be/S_v172PgToE?si=randomuselessgarbageparams",
    ];

    let video_id = VideoId::new("S_v172PgToE").unwrap();

    assert_eq!(video_id.as_str(), "S_v172PgToE");

    for url in valid_urls.iter() {
        assert_eq!(GetId::<VideoId>::get_id(&IdCollection::from_url(*url).unwrap()).unwrap(), video_id);
    }
}

#[test]
fn channel_urls() {
    let valid_urls = vec![
        "https://music.youtube.com/channel/MPADUC6Tg7GWjZw48EiZ8m5bRtWg",
        "https://music.youtube.com/channel/UC6Tg7GWjZw48EiZ8m5bRtWg",
        "https://www.youtube.com/channel/MPADUC6Tg7GWjZw48EiZ8m5bRtWg",
        "https://www.youtube.com/channel/UC6Tg7GWjZw48EiZ8m5bRtWg",
        "https://music.youtube.com/browse/MPADUC6Tg7GWjZw48EiZ8m5bRtWg",
        "https://music.youtube.com/browse/UC6Tg7GWjZw48EiZ8m5bRtWg",
        "https://youtube.com/channel/UC6Tg7GWjZw48EiZ8m5bRtWg",
        "https://m.youtube.com/channel/UC6Tg7GWjZw48EiZ8m5bRtWg",
    ];

    let channel_id = ChannelId::new("UC6Tg7GWjZw48EiZ8m5bRtWg").unwrap();

    assert_eq!(channel_id.as_str(), "MPADUC6Tg7GWjZw48EiZ8m5bRtWg");

    for url in valid_urls.iter() {
        assert_eq!(GetId::<ChannelId>::get_id(&IdCollection::from_url(*url).unwrap()).unwrap(), channel_id);
    }
}
