use crate::{
    Result,
    downloader::{
        media::{Media, MediaBrowse},
        playlist::{Playlist, PlaylistBrowse},
    },
    id_resolver::ChannelId,
};

#[derive(Debug)]
pub struct ChannelBrowse {
    channel_id: ChannelId,
}

#[derive(Debug)]
pub struct ChannelConentBrowse {
    singles: Vec<MediaBrowse>,
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
    pub async fn browse(mut self) -> Result<Channel> {
        let mut browsed_singles = Vec::new();
        let mut browsed_eps = Vec::new();
        let mut browsed_albums = Vec::new();

        for single in self.singles.drain(..) {
            browsed_singles.push(single.browse().await?);
        }

        for ep in self.eps.drain(..) {
            browsed_eps.push(ep.browse().await?.browse().await?);
        }

        for album in self.albums.drain(..) {
            browsed_albums.push(album.browse().await?.browse().await?);
        }
        Ok(Channel::new(browsed_singles, browsed_albums, browsed_eps))
    }
}

impl Channel {
    fn new(singles: Vec<Media>, albums: Vec<Playlist>, eps: Vec<Playlist>) -> Self {
        Self { singles, albums, eps }
    }
}
