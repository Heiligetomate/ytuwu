use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{channel_id::ChannelId, channel_playlist_id::ChannelPlaylistId},
    request::core::captcha_bypass,
};

pub async fn get_first_ep_for_testing_meow(id: ChannelId) -> Result<ChannelPlaylistId> {
    let resp = captcha_bypass(&id, 1).await?;
    let ids = resp.extract_all_releases()?;
    let test_ep = ids
        .albums
        .get(0)
        .ok_or(YtuwuError::SongInPlaylistNotFound)?;

    Ok(test_ep.clone())
}
