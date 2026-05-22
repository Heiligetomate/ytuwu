use ytuwu::{
    Result,
    id_resolver::{
        channel_name_id::ChannelNameId,
        id::{Id, MakeChannelId},
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let id = ChannelNameId::new("@ntomusic");
    let transformed = id.transform().await?;
    println!("{:#?}", transformed);
    Ok(())
}
