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

/// This is the time that should be waited between every captcha bypass attempt
const WAIT_FOR_CAPTCHA_MILLIS: Duration = Duration::from_millis(400);

/// The visitor data is needed for bypassing captchas but only for the player client
/// Using visior data for other clients works too and does not lead to any problems
/// This function builds the headers and the body from the id client and sends it to the youtube api
/// After that, it tries to deserialize response into the matching response stored in the client
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

/// This function is a wrapper for make_request
/// Builds the body and header from the id client
/// Returns the matching response stored in the client
/// Does not take visitor data, use this function for responses other than the player client
pub async fn api_request<I>(id: &I, client: &reqwest::Client) -> Result<<<I as Id>::Client as ClientWithHeaders>::Response>
where
    I: Id,
    I::Client: ClientWithHeaders,
    <<I as Id>::Client as ClientWithHeaders>::Response: DeserializeOwned,
{
    make_request(id, None, client).await
}

/// This function is for the player client and therefore can only accept VideoId  
/// Takes sharedVd which is an alias for Arc<Mutex<Option<String>>>
/// The visior data is shared in the downloader struct and can be used for multiple requests
/// If the request fails with the current visitor data, it changed the shared visitor data to the
/// new visitor data obtained by sending the request. After that, it matches the response status and
/// returns the response if successfull
/// If the status is "Login", it waits for WAIT_FOR_CAPTCHA_MILLIS and tries again with the new
/// visior data until the tries are max_tries   
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
