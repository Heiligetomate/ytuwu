use crate::{error::YtuwuError, id_resolver::IdCollection};

/// Custom restul for ytuwu which should be used for every funciton in this library that returns a
/// result.
/// Holds a generic value T and a variant of the enum YtuwuError
pub type Result<T> = std::result::Result<T, YtuwuError>;

/// This type is for errors and should be used with the fmt_err_inf function to implement Display
/// cleanly.
/// This allows the usage of the errors without forcing an extra error message
pub type ErrInf = Option<String>;

/// Adds a colon with space infront of the error message if it is Some, returns a . if the error
/// message is None.
/// Use this for implementing Display and do not add a . at the end of the first part of the error
/// message because this would break the format
pub(super) fn fmt_err_inf(opt_err: &ErrInf) -> String {
    match opt_err {
        Some(e) => format!(": {}", e),
        None => String::from("."),
    }
}

/// Builds a clean error message stating what id was expected and what ids are available
/// Always returns YtuwuError::NoIdFound
/// Example
///```rust
//#impl GetId<VideoId> for IdCollection {
//#    fn get_id(&self) -> Result<VideoId> {
//#        Ok(self
//#            .video_id
//#            .clone()
//#            .ok_or(crate::error::get_id_err("videoId", &self))?)
//#    }
//#}
///```
pub fn get_id_err(expected_type: &str, id_collection: &IdCollection) -> YtuwuError {
    let existing_ids = id_collection.info();
    let err_string = format!("Failed to get {} from IdCollection. Existing ids in this IdCollection: {}", expected_type, existing_ids);

    YtuwuError::IdNotContained(Some(err_string))
}
