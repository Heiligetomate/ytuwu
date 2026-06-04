use crate::models::response::Response;

pub struct DummyResponse {}

impl Response for DummyResponse {
    fn get_status(&self) -> super::response::Status {
        panic!("this is a dummy and should never be called");
    }
}
