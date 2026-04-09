pub trait Response {
    fn get_visitor_data(&self)  -> Option<String>; 
    fn get_status(&self)        -> Status;
}



#[derive(PartialEq, Eq)]
pub enum Status {
    Login,
    Success,
    Error,
}

// mhh
impl Status {
    pub fn is_err(&self) -> bool {
        *self == Status::Error
    }
    pub fn is_success(&self) -> bool {
        *self == Status::Success
    }
}
