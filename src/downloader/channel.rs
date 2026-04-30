use crate::{
    Result,
    downloader::{
        media::{Media, MediaBrowse},
        playlist::{Playlist, PlaylistBrowse},
    },
    id_resolver::{ChannelId, VideoId},
};

#[derive(Debug)]
pub struct ChannelBrowse {
    channel_id: ChannelId,
}

#[derive(Debug)]
pub struct ChannelConentBrowse {
    sigles: Vec<MediaBrowse>,
    albums: Vec<PlaylistBrowse>,
    eps: Vec<PlaylistBrowse>,
}

pub struct Channel {
    singles: Vec<Media>,
    albums: Vec<Playlist>,
    eps: Vec<Playlist>,
}

impl ChannelBrowse {
    pub async fn browse(self) -> Result<ChannelConentBrowse> {
        todo!()
    }
}

impl ChannelConentBrowse {
    pub async fn browse(self) -> Result<Channel> {
        todo!()
    }
}
