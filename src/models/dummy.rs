use crate::models::response::Response;

/// This response is a dummy and panics if used as an actual response
/// Its needed because some traits need a type: Response
/// Some of the types implemening the traits dont send anything meaning there is a dummy needed.
/// A dummy that panics is still better than using a random type implementing response
pub struct DummyResponse {}

impl Response for DummyResponse {
    fn get_status(&self) -> super::response::Status {
        panic!("this is a dummy and should never be called");
    }
}
