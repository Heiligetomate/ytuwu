use crate::{Result, id_resolver::id_types::VideoId};

pub trait Response {
    fn get_status(&self) -> Status;
}

pub trait BrowseResponse: Response {
    fn get_video_ids(&self) -> Result<Vec<VideoId>>;
    fn get_album_title(&self) -> Result<&str>;
}

#[derive(PartialEq, Eq, Debug)]
pub enum Status {
    Login,
    Success,
    Error,
}
