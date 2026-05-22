use ytuwu::{
    Result,
    id_resolver::{
        id::{Id, MakeChannelId},
        id_types::channel_name_id::ChannelNameId,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let id = ChannelNameId::new("@ntomusic");
    let transformed = id.transform().await?;
    println!("{:#?}", transformed);
    Ok(())
}
