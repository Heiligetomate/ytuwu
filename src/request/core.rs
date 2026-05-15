use crate::{
    error::{Result, YtuwuError},
    models::response::{Response, Status},
    request::clients::client::ClientWithHeaders,
};
use serde::de::DeserializeOwned;

async fn make_request<C>(id: &str, visitor_data: Option<String>) -> Result<C::Response>
where
    C: ClientWithHeaders,
    C::Response: DeserializeOwned,
{
    let headers = C::build_headers();
    let body = C::build_body(id, visitor_data);
    let response: &str = &headers
        .json(&body)
        .send()
        .await?
        .text()
        .await?;
    let result: C::Response = serde_json::from_str(&response)?;
    Ok(result)
}

pub async fn captcha_bypass<C>(id: &str, max_tries: u16) -> Result<C::Response>
where
    C: ClientWithHeaders,
    C::Response: DeserializeOwned,
{
    let mut tries: u16 = 0;

    let mut visitor_data: Option<String> = None;
    while tries < max_tries {
        tries += 1;
        let resp: C::Response = make_request::<C>(id, visitor_data).await?;
        match resp.get_status() {
            Status::Error => return Err(YtuwuError::YoutubeAPIReturn),
            Status::Success => return Ok(resp),
            Status::Login => println!("trying to bypass captcha for {}", id),
        }
        visitor_data = resp.get_visitor_data();
    }
    Err(YtuwuError::CaptchaBypassFailed(max_tries))
}
