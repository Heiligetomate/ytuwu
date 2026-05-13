use reqwest::RequestBuilder;

pub trait Response {
    fn get_visitor_data(&self) -> Option<String>;
    fn get_status(&self) -> Status;
}

pub type ClientPrebuild = RequestBuilder;

pub trait ClientWithHeaders {
    type Response: Response;
    fn get_client() -> ClientPrebuild;
}

#[derive(PartialEq, Eq, Debug)]
pub enum Status {
    Login,
    Success,
    Error,
}
