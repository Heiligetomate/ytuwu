use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{player_model::{
    playability_status::{PlayabilityStatus, PlayabilityStatusValue},
    response_context::ResponseContext,
    streaming_data::StreamingData,
    video_details::VideoDetails,
}, shared_traits::{self, Response}};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse { // has too many useless fields
    response_context: Option<ResponseContext>,
    pub playability_status: Option<PlayabilityStatus>,
    pub streaming_data: Option<StreamingData>,
    pub video_details: Option<VideoDetails>,
}

impl PlayerResponse {
    pub fn get_title(&self) -> &str {
        // ughh
        &self.video_details.as_ref().unwrap().title
    }
    
    pub fn get_author(&self) -> Option<&str> {
        if let Some(video_details) = &self.video_details {
            return Some(&video_details.author)
        }
        None
    }
}

impl Response for PlayerResponse {
    fn get_visitor_data(&self) -> Option<String> {
        if let Some(response_context) = &self.response_context {
            return response_context.visitor_data.clone()
        }
        None
    }

    fn get_status(&self) -> shared_traits::Status {
        if let Some(playability_status) = &self.playability_status {
            return match playability_status.status {
                PlayabilityStatusValue::Ok            => shared_traits::Status::Success,
                PlayabilityStatusValue::LoginRequired => shared_traits::Status::Login,
                _                                     => shared_traits::Status::Error,
            };
        }
        shared_traits::Status::Error
    }
}
