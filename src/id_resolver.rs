use std::fmt;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

pub trait Id {
    fn new<T: Into<String>>(id: T) -> Self;
    fn get_id(self) -> String;
    fn as_str(&self) -> &str;
    //fn new_valid<T>(id: T) -> Self where T: Into<String>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrowseId {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoId {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelId {
    id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdCollection {
    pub video_id : Option<VideoId>,
    pub browse_id: Option<BrowseId>,
}

impl Id for BrowseId {
    fn new<T: Into<String>>(id: T) -> Self {
        Self {
            id: id.into()
        }
    }
    fn get_id(self) -> String {
        self.id
    }
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl Id for VideoId {
    fn new<T: Into<String>>(id: T) -> Self {
        Self {
            id: id.into()
        }
    }
    fn get_id(self) -> String {
        self.id
    }
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl Id for ChannelId {
    fn new<T: Into<String>>(id: T) -> Self {
        Self {
            id: id.into()
        }
    }
    fn get_id(self) -> String {
        self.id
    } 
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl fmt::Display for IdCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let browse_display: String = {
            if let Some(id) = &self.browse_id {
                id.as_str().to_owned()
            } else {
                "None".to_owned()
            }
        };
        let video_display: String = {
            if let Some(id) = &self.video_id {
                id.as_str().to_owned()
            } else {
                "None".to_owned()
            }
        };
        write!(f, "browse_id: {} \nvideo_id : {}", &browse_display, &video_display)
    } 
}

impl IdCollection {
    pub fn from_url<T: Into<String>>(raw_url: T) -> Option<Self> {
        let url: String = raw_url.into(); 
        let mut video_id:  Option<VideoId>  = None;
        let mut browse_id: Option<BrowseId> = None;
        if let Some(vid_id) = video_id_from_raw_url(&url) {
            video_id = Some(vid_id); 
        }
        if let Some(br_id) = playlist_id_from_raw_url(&url) {
            browse_id = Some(br_id);
        }
        if video_id.is_none() && browse_id.is_none() {
            return None
        }
        Some( 
            Self { 
                video_id, 
                browse_id 
            }
        )
    }
    pub fn get_video_id(self) -> Option<VideoId> {
        self.video_id
    }
    pub fn get_browse_id(self) -> Option<BrowseId> {
        self.browse_id
    }
}


fn video_id_from_raw_url(raw_url: &str) -> Option<VideoId> {
    // youtu.be (weird url but has a video id in it)
    let parts: Vec<&str> = raw_url.split("v=").collect();
    let part_res = if let Some(part) = parts.get(1) {
        part
    } else {
        return None;
    };
    let res: Vec<&str> = part_res.split('&').collect();
    if res.is_empty() {
        return None;
    }
    Some(VideoId::new(res[0]))
}

fn playlist_id_from_raw_url(raw_url: &str) -> Option<BrowseId> {
    let parts: Vec<&str> = raw_url.split("list=").collect();
    let res = parts.get(1);
    if res.is_none() { 
        return None;
    }
    Some(BrowseId::new(format!("VL{}", res.unwrap())))
}




