use crate::{Result, id_resolver::types::VideoId};

// TODO: handle status better for the models
/// This triat is needed so that the client trait can hold a response
/// The raw response can then tried to be deserialized into the expected response
pub trait Response {
    /// This function is needed to check what the response status is
    /// Very important for the request methods
    fn get_status(&self) -> Status;
}

/// This trait is needed because there are three different responses used for playlists
pub trait BrowseResponse: Response {
    /// This function extracts all video ids from a BrowseResponse
    fn get_video_ids(&self) -> Result<Vec<VideoId>>;
    /// This funciton extracts the title of the album/single/ep
    fn get_album_title(&self) -> Result<&str>;
}

/// This enum represents the Response status
#[derive(PartialEq, Eq, Debug)]
pub enum Status {
    /// The response did not fail but youtube is telling the client to either solve the captcha or
    /// log in for age verification
    Login,
    /// This means the response is as expected
    Success,
    /// This means the response failed
    Error,
}
