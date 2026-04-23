use std::fmt::Debug;


use reqwest::{RequestBuilder};
use serde::de::DeserializeOwned;
use crate::{id_resolver::{BrowseId, Id, VideoId}, request::{parameters::*, request_builder::RequestBody}, shared_traits::{Response, Status}};
use anyhow::{Result, anyhow};

#[derive(Debug)]
pub enum Endpoint {
    Browse(BrowseId),
    Player(VideoId),
}

impl Endpoint {
    pub fn get_id(&self) -> &str {
        match &self {
            Self::Player(id) => id.as_str(),
            Self::Browse(id) => id.as_str(),
        }
    }
}

fn builder_headers() -> Result<RequestBuilder> {
    let client = reqwest::Client::new();
    Ok(
        client.post(ENDPOINT)
            .header(CONTENT_TYPE_HEADER.0, CONTENT_TYPE_HEADER.1)
            .header(USER_AGENT_HEADER.0, USER_AGENT_HEADER.1)
            .header(CLIENT_NAME_HEADER.0, CLIENT_NAME_HEADER.1)
            .header(CLIENT_VERSION_HEADER.0, CLIENT_VERSION_HEADER.1)
            .header(ORIGIN_HEADER.0, ORIGIN_HEADER.1)
    )
}
fn build_body<'de>(endpoint: &Endpoint, visitor_data: Option<String>) -> RequestBody<'de> {
    match endpoint {
        Endpoint::Player(video_id)  => RequestBody::new_player_request(video_id.clone(), visitor_data),
        Endpoint::Browse(browse_id) => RequestBody::new_browse_request(browse_id.clone(), visitor_data),
    }
} 

async fn make_request<'de, R>(body: &RequestBody<'de>) -> Result<R>
where 
    R: Response + DeserializeOwned + Debug, 
{
    let headers = builder_headers()?;
    let response: &str = &headers
        .json(body)
        .send()
        .await?
        .text()
        .await?;
    let result: R = serde_json::from_str(&response)?;
    Ok(result)
}

pub async fn captcha_bypass<R>(endpoint: Endpoint, max_tries: u16) -> Result<R>
where 
    R: Response + DeserializeOwned + Debug,
{
    let mut tries: u16 = 0;
     
    let mut visitor_data: Option<String> = None; 
    while tries < max_tries {
        let body = build_body(&endpoint, visitor_data);
        tries += 1;
        let resp: R = make_request(&body).await?;
        match resp.get_status() {
            Status::Error => return Err(anyhow!(format!("Returned an error"))),
            Status::Success => return Ok(resp),
            Status::Login => println!("trying to bypass captcha for {}", endpoint.get_id()),
        }
        visitor_data = resp.get_visitor_data();
    }
    Err(anyhow!(format!("Couldnt bypass captcha after {} tries", max_tries)))
}


