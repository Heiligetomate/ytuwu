use crate::{
    error::{Result, YtuwuError},
    id_resolver::id::Id,
    id_types::VideoId,
    models::{
        player::PlayerResponse,
        response::{Response, Status},
    },
    request::clients::client::ClientWithHeaders,
};
use serde::de::DeserializeOwned;

async fn make_request<I>(id: &I, visitor_data: Option<String>) -> Result<<<I as Id>::Client as ClientWithHeaders>::Response>
where
    I: Id,
    I::Client: ClientWithHeaders,
    <<I as Id>::Client as ClientWithHeaders>::Response: DeserializeOwned,
{
    let headers = I::Client::build_headers();
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

pub async fn api_request<I>(id: &I) -> Result<<<I as Id>::Client as ClientWithHeaders>::Response>
where
    I: Id,
    I::Client: ClientWithHeaders,
    <<I as Id>::Client as ClientWithHeaders>::Response: DeserializeOwned,
{
    make_request(id, None).await
}

pub async fn api_captcha_bypass(id: &VideoId, max_tries: u16) -> Result<PlayerResponse>
where {
    let mut tries: u16 = 0;
    let mut visitor_data: Option<String> = None;

    let mut error_message = String::from("unknown");

    while tries < max_tries {
        tries += 1;
        let resp: PlayerResponse = make_request(id, visitor_data).await?;
        match resp.get_status() {
            Status::Error => return Err(YtuwuError::YoutubeAPIReturn),
            Status::Success => return Ok(resp),
            Status::Login => {
                error_message = resp
                    .get_playability_reason()
                    .unwrap_or("unknown")
                    .to_string();
                println!("trying to bypass captcha for {}", id.as_str())
            }
        }
        visitor_data = resp
            .get_visitor_data()
            .map(|vd| vd.to_owned());
    }
    println!("Could not bypass the captcha. Reason: {}", error_message);
    Err(YtuwuError::CaptchaBypassFailed(max_tries))
}
