use crate::{Result, id_resolver::channel_id::ChannelId, request::core::captcha_bypass};

pub async fn get_channel_ids_blablalba(id: ChannelId) -> Result<()> {
    let resp = captcha_bypass(&id, 1).await?;
    let ids = resp.extract_all_releases()?;

    println!("{:#?}", ids);

    Ok(())
}
