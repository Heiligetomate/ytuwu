use std::fmt::Debug;

use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use crate::{id_resolver::{BrowseId, Id, VideoId}, request::request_builder::RequestBody, shared_traits::{Response, Status}};
use anyhow::{Result, anyhow};

#[derive(Debug)]
pub enum Endpoint {
    Browse(BrowseId),
    Player(VideoId),
}

impl Endpoint {
    pub fn as_str(&self) -> &str {
        match &self {
            Self::Browse(_) => "https://music.youtube.com/youtubei/v1/browse", // music? music!
            Self::Player(_) => "https://www.youtube.com/youtubei/v1/player",
        }
    }
    pub fn origin(&self) -> &str {
        match &self {
            Self::Browse(_) => "https://music.youtube.com",
            Self::Player(_) => "https://www.youtube.com",
        } 
    }
    pub fn get_id(&self) -> &str {
        match &self {
            Self::Player(id) => id.as_str(),
            Self::Browse(id) => id.as_str(),
        }
    }
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Context {
    client: Client,
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    client_name: String,
    client_version: String,
    device_make: String,
    device_model: String,
    android_sdk_version: u16,
    hl: String,
    gl: String,
    time_zone: String,
    utc_offset_minutes: u16,
    visitor_data: Option<String>,
}



impl Context {
    pub fn default_downloader_body(visitor_data: Option<String>) -> Self {
        Self {
            client: Client {             
                client_name: String::from("ANDROID_VR"),
                client_version: String::from("1.60.19"),
                device_make: String::from("Oculus"),
                device_model: String::from("Quest 2"),
                android_sdk_version: 29,
                hl: String::from("en"),
                gl: String::from("US"),
                time_zone: String::from("UTC"),
                utc_offset_minutes: 0,
                visitor_data: visitor_data,
            }
        }
    }
}

fn builder_headers(endpoint: &Endpoint) -> Result<RequestBuilder> {
    let client = reqwest::Client::new();
    Ok(
        client.post(endpoint.as_str())
            .header("Content-Type", "application/json")
            .header("User-Agent", "User-Agent: Mozilla/5.0 (Linux; Android 10; Quest 2) AppleWebKit/537.36 (KHTML, like Gecko) OculusBrowser/32.0.0.3.65 SamsungBrowser/4.3 Chrome/137.0.7151.61 Mobile VR Safari/537.36")
            .header("X-YouTube-Client-Name", "28")
            .header("X-YouTube-Client-Version", "1.60.19")
            .header("Origin", endpoint.origin())
    )
}
fn build_body(endpoint: &Endpoint, visitor_data: Option<String>) -> RequestBody {
    match endpoint {
        Endpoint::Player(video_id)  => RequestBody::new_player_request(video_id.clone(), visitor_data),
        Endpoint::Browse(browse_id) => RequestBody::new_browse_request(browse_id.clone(), visitor_data),
    }
} 

async fn make_request<R>(body: &RequestBody, endpoint: &Endpoint) -> Result<R>
where 
    R: Response + DeserializeOwned + Debug, 
{
    let headers = builder_headers(endpoint)?;
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
        let resp: R = make_request(&body, &endpoint).await?;
        match resp.get_status() {
            Status::Error => return Err(anyhow!(format!("Returned an error"))),
            Status::Success => return Ok(resp),
            Status::Login => println!("trying to bypass captcha for {}", endpoint.get_id()),
        }
        visitor_data = resp.get_visitor_data();
    }
    Err(anyhow!(format!("Couldnt bypass captcha after {} tries", max_tries)))
}


