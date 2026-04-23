use serde::Deserialize;

use crate::player_model::itag::Itag;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    formats: Vec<Stream>,
    adaptive_formats: Vec<Stream>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    itag: u16,
    url: String,
}

impl Stream {
    pub fn get_url(&self) -> &str {
        self.url.as_ref()
    }
}

impl StreamingData {
    pub fn get_url_by_itag(&self, itag: &impl Itag) -> Option<&str> {
        for format in self.adaptive_formats.iter() {
            if format.itag == itag.to_int() {
                return Some(format.get_url());
            }
        }
        for adaptive_format in self.formats.iter() {
            if adaptive_format.itag == itag.to_int() {
                return Some(adaptive_format.get_url());
            }
        }
        None
    }
}
