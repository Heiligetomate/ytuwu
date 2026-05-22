use serde::Deserialize;

use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{id::Id, id_types::ChannelId},
    models::response::Response,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelNameToIdResponse {
    response_context: Option<ResponseContext>,
    endpoint: Option<Endpoint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseContext {
    visitor_data: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Endpoint {
    browse_endpoint: Option<BrowseEndpoint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrowseEndpoint {
    browse_id: Option<String>,
}

impl ChannelNameToIdResponse {
    pub fn get_id(&self) -> Result<ChannelId> {
        let id: &str = &self
            .endpoint
            .as_ref()
            .ok_or(YtuwuError::ChannelDataNotFound("endpoint"))?
            .browse_endpoint
            .as_ref()
            .ok_or(YtuwuError::ChannelDataNotFound("browse endpoint"))?
            .browse_id
            .as_ref()
            .ok_or(YtuwuError::ChannelDataNotFound("channel id"))?;
        Ok(ChannelId::new(id))
    }
}

impl Response for ChannelNameToIdResponse {
    fn get_status(&self) -> super::response::Status {
        if self.endpoint.is_some() {
            return super::response::Status::Success;
        }
        super::response::Status::Error
    }

    fn get_visitor_data(&self) -> Option<String> {
        if let Some(response_context) = &self.response_context {
            if let Some(visitor_data) = &response_context.visitor_data {
                return Some(visitor_data.to_owned());
            }
        }
        None
    }
}
