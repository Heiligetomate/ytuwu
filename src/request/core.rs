use crate::{
    error::{Result, YtuwuError},
    id_resolver::id::Id,
    models::response::{Response, Status},
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

    //println!("{:#?}", response);
    let result: <<I as Id>::Client as ClientWithHeaders>::Response = serde_json::from_str(&response)?;
    Ok(result)
}

pub async fn captcha_bypass<I>(id: &I, max_tries: u16) -> Result<<<I as Id>::Client as ClientWithHeaders>::Response>
where
    I: Id,
    I::Client: ClientWithHeaders,
    <<I as Id>::Client as ClientWithHeaders>::Response: DeserializeOwned,
{
    let id_type = std::any::type_name::<I>();
    let client_type = std::any::type_name::<<I as Id>::Client>();
    let response_type = std::any::type_name::<<<I as Id>::Client as ClientWithHeaders>::Response>();

    println!("got {} as id", id_type);
    println!("using {} as client", client_type);
    println!("using {} as response", response_type);
    let mut tries: u16 = 0;
    let mut visitor_data: Option<String> = None;
    while tries < max_tries {
        tries += 1;
        let resp: <<I as Id>::Client as ClientWithHeaders>::Response = make_request(id, visitor_data).await?;
        match resp.get_status() {
            Status::Error => return Err(YtuwuError::YoutubeAPIReturn),
            Status::Success => return Ok(resp),
            Status::Login => println!("trying to bypass captcha for {}", id.as_str()),
        }
        visitor_data = resp.get_visitor_data();
    }
    Err(YtuwuError::CaptchaBypassFailed(max_tries))
}
