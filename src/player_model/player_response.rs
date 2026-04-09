use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{player_model::{
    itag::Itag,
    playability_status::{PlayabilityStatus, PlayabilityStatusValue},
    response_context::ResponseContext,
    streaming_data::StreamingData,
    video_details::{ThumbnailResolution, VideoDetails},
}, shared_traits::{self, Response}};
use crate::Result;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse { // has too many useless fields
    response_context: Option<ResponseContext>,
    pub playability_status: Option<PlayabilityStatus>,
    pub streaming_data: Option<StreamingData>,
    pub video_details: Option<VideoDetails>,
}

impl PlayerResponse {
    pub fn status_is_login_required(&self) -> bool {
        // temp
        self.playability_status.as_ref().unwrap().status == PlayabilityStatusValue::LoginRequired
    }
    pub fn get_visitor_data(&self) -> Option<&str> {
        if let Some(response_context) = &self.response_context {
            return response_context.visitor_data.as_deref();
        }
        None
    }
    pub fn get_url_by_itag(&self, itag: &Itag) -> Option<&str> {
        match &self.streaming_data {
            Some(dat) => return dat.get_url_by_itag(itag),
            None => return None,
        }
    }
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

    pub fn get_streaming_data(&self) -> Result<&StreamingData> {
        Ok(self.streaming_data.as_ref().ok_or(anyhow!("no streaming data found"))?)
    }
    pub fn get_thumbnail_url(&self, resolution: ThumbnailResolution) -> Result<&str> {
        let video_details = &self.video_details.as_ref().ok_or(anyhow!("no video details found"))?;
        let thumbnail_url = video_details.thumbnail.url_by_resolution(&resolution).ok_or(anyhow!("invalid resolution"))?; 
        Ok(thumbnail_url)
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
