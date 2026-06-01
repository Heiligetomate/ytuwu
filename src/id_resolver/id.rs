use crate::{Result, id_resolver::types::ChannelId, request::clients::client::ClientWithHeaders};

pub trait GetId<T: Id> {
    fn get_id(&self) -> Result<T>;
}

pub trait Id: Sized {
    type Client: ClientWithHeaders;
    fn new<T: Into<String>>(id: T) -> Result<Self>;
    fn get_id(self) -> String;
    fn as_str(&self) -> &str;
}

#[allow(async_fn_in_trait)]
pub trait MakeChannelId: Id {
    async fn transform(&self, client: &reqwest::Client) -> Result<ChannelId>;
}
