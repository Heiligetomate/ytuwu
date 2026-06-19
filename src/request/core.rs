use std::time::Duration;

use crate::{
    downloader::SharedVd,
    error::{Result, YtuwuError},
    id_resolver::Id,
    models::{
        PlayerResponse,
        response::{Response, Status},
    },
    request::clients::ClientWithHeaders,
    types::VideoId,
};
use serde::de::DeserializeOwned;

const WAIT_FOR_CAPTCHA_MILLIS: Duration = Duration::from_millis(400);

async fn make_request<I>(id: &I, visitor_data: Option<String>, client: &reqwest::Client) -> Result<<<I as Id>::Client as ClientWithHeaders>::Response>
where
    I: Id,
    I::Client: ClientWithHeaders,
    <<I as Id>::Client as ClientWithHeaders>::Response: DeserializeOwned,
{
    let headers = I::Client::build_headers(client);
    let body = I::Client::build_body(id.as_str(), visitor_data);
    let response: &str = &headers
        .json(&body)
        .send()
        .await?
        .text()
        .await?;

    let result: <<I as Id>::Client as ClientWithHeaders>::Response = serde_json::from_str(&response)?;
    Ok(result)
}

pub async fn api_request<I>(id: &I, client: &reqwest::Client) -> Result<<<I as Id>::Client as ClientWithHeaders>::Response>
where
    I: Id,
    I::Client: ClientWithHeaders,
    <<I as Id>::Client as ClientWithHeaders>::Response: DeserializeOwned,
{
    make_request(id, None, client).await
}

pub async fn api_captcha_bypass(id: &VideoId, max_tries: u16, visitor_data: &SharedVd, client: &reqwest::Client) -> Result<PlayerResponse>
where {
    let mut tries: u16 = 0;

    let mut error_message = String::from("unknown");

    while tries < max_tries {
        tries += 1;
        let vd = visitor_data.lock().await.clone();
        let resp: PlayerResponse = make_request(id, vd, client).await?;
        match resp.get_status() {
            Status::Error => return Err(YtuwuError::YoutubeAPIReturn),
            Status::Success => return Ok(resp),
            Status::Login => {
                error_message = resp
                    .get_playability_reason()
                    .unwrap_or("unknown")
                    .to_string();
                tokio::time::sleep(WAIT_FOR_CAPTCHA_MILLIS).await;
            }
        }
        *visitor_data.lock().await = resp
            .get_visitor_data()
            .map(|vd| vd.to_owned());
    }
    println!("Could not bypass the captcha. Reason: {}", error_message);
    Err(YtuwuError::CaptchaBypassFailed(max_tries))
}
