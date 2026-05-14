use crate::{Result, request::clients::client::ClientWithHeaders};

pub trait GetId<T: Id> {
    fn get_id(&self) -> Result<T>;
}

pub trait Id {
    type Client: ClientWithHeaders;
    fn new<T: Into<String>>(id: T) -> Self;
    fn get_id(self) -> String;
    fn as_str(&self) -> &str;
}
