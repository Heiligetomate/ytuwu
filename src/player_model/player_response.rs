use serde::{Deserialize};


use crate::{error::{YtuwuError, Result}, player_model::{
    itag::Itag,
    playability_status::{PlayabilityStatus, PlayabilityStatusValue},
    streaming_data::StreamingData,
    video_details::{ThumbnailResolution, VideoDetails},
}, shared_traits::{self, Response}};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse { 
    response_context: Option<ResponseContext>,
    playability_status: Option<PlayabilityStatus>,
    streaming_data: Option<StreamingData>,
    video_details: Option<VideoDetails>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContext {
    pub visitor_data: Option<String>,
}

impl PlayerResponse {

    pub fn get_best_stream<I: Itag + Copy>(&self, itag: &I) -> Result<&str> {
        let streams = self.get_streaming_data()?;         
        let mut current_itag = *itag;
        let mut url: Option<&str> = streams.get_url_by_itag(&current_itag);
        while url.is_none() {
            current_itag = current_itag.next_best()?;
            url = streams.get_url_by_itag(&current_itag);
        }
        Ok(url.ok_or(YtuwuError::NoMatchingItag)?)
    }

    fn get_streaming_data(&self) -> Result<&StreamingData> {
        Ok(
            self
                .streaming_data
                .as_ref()
                .ok_or(YtuwuError::PlayerDataNotFound("streaming data"))?
        )
    }

    fn get_video_deatails(&self) -> Result<&VideoDetails> {
        Ok(
            self.
                video_details
                .as_ref()
                .ok_or(YtuwuError::PlayerDataNotFound("video details"))?
        )
    }

    pub fn get_thumbnail_url_by_res(&self, resolution: &ThumbnailResolution) -> Result<&str> {
        let url = self
            .get_video_deatails()?
            .thumbnail
            .url_by_resolution(resolution)
            .ok_or(YtuwuError::PlayerDataNotFound("thumbnails"))?;
        Ok(url)
    } 

    pub fn get_title(&self) -> Result<&str> {
        Ok(
            &self.get_video_deatails()?.title
        )
    }
    
    pub fn get_author(&self) -> Result<&str> {
        if let Some(video_details) = &self.video_details {
            return Ok(&video_details.author)
        }
        Err(YtuwuError::PlayerDataNotFound("author"))
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
