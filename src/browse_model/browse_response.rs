use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::{browse_model::{error_response::ErrorResponse, full_response::FullResponse, header::BrowseHeader, response_context::ResponseContext}, shared_traits::Response};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BrowseResponse {
    error: Option<ErrorResponse>,
    contents: Option<FullResponse>,
    response_context: Option<ResponseContext>,
    header: Option<BrowseHeader>
}

impl BrowseResponse {
    pub fn get_ids(&self) -> Result<Vec<&str>> {
        let ids = self
            .contents
            .as_ref()
            .ok_or(anyhow!("no contents found"))?
            .get_ids()?;

        Ok(ids)
    }
    pub fn get_album_title(&self) -> Result<&str> {
        let header = self.header.as_ref().ok_or(anyhow!("no header found"))?;
        let title = &header.get_album_title()?;
        Ok(title)
    }
}

impl Response for BrowseResponse {
    fn get_status(&self) -> crate::shared_traits::Status {
        if self.error.is_some() {
            return crate::shared_traits::Status::Error;
        }
        crate::shared_traits::Status::Success
    }

    fn get_visitor_data(&self)  -> Option<String> {
        if let Some(response_context) = &self.response_context {
            return response_context.visitor_data.clone(); 
        }
        None
    }
}
