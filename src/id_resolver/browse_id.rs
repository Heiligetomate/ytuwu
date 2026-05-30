use crate::{Id, models::response::BrowseResponse};

pub trait BrowseId: Id {
    type BrowseResponse: BrowseResponse;
}
