use crate::{Result, id_resolver::video_id::VideoId};

pub trait Response {
    fn get_visitor_data(&self) -> Option<String>;
    fn get_status(&self) -> Status;
}

pub trait BrowseResponse {
    fn get_video_ids(&self) -> Result<Vec<VideoId>>;
}

#[derive(PartialEq, Eq, Debug)]
pub enum Status {
    Login,
    Success,
    Error,
}
