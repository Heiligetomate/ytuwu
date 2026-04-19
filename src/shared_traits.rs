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

