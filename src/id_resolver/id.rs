use crate::{Result, request::clients::ClientWithHeaders};

/// This trait is for a clean api and allows to call .get_id() on an idcollection as long as its
/// clear what type the function using the value from .get_id() expects
pub trait GetId<T: Id> {
    fn get_id(&self) -> Result<T>;
}

/// This trait is needed for clean api requests.
pub trait Id: Sized {
    /// This is needed so that a youtube api request function just has to take a generic parameter
    /// representing the ID to know what client it needs to use and the client knows what response
    /// it shoud expect.
    type Client: ClientWithHeaders;
    /// This function creaters a new if from anything implementing into String. It also validates
    /// the id (the format is different for every id)
    fn new<T: Into<String>>(id: T) -> Result<Self>;
    /// Consumes itself and returns the inner id value as String
    fn get_id(self) -> String;
    /// Returns a string slice referencing the inner id
    fn as_str(&self) -> &str;
}
