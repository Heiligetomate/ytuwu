use crate::config::{ChannelConfig, DwnConfig};

/// Holds all important config option that are important for configuring the downloader.
#[derive(Debug, Default)]
pub struct Config {
    /// Holds all options that are used when downloading a full channel.
    pub(crate) channel: ChannelConfig,
    /// Holds all options that are used when downloading anything such as chunk size.
    pub(crate) download: DwnConfig,
}

impl Default for DwnConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1024 * 1024,
            concurrent_chunks: 8,
            concurrent_streams: 4,
            randomize_size: false,
            randomize_range: 100,
        }
    }
}

impl Config {
    /// Consumes self as mutuble and takes a function that changes the inner channel config to a new
    /// channel config.
    /// Returns itself with the new channel config.
    pub fn channel(mut self, cls: impl Fn(ChannelConfig) -> ChannelConfig) -> Self {
        let channel_cfg = std::mem::take(&mut self.channel);
        let channel_cfg = cls(channel_cfg);
        Self { channel: channel_cfg, ..self }
    }

    /// Consumes self as mutuble and takes a function that changes the inner download config to a new
    /// download config.
    /// Returns itself with the new download config.
    pub fn download(mut self, cls: impl Fn(DwnConfig) -> DwnConfig) -> Self {
        let dwn_cfg = std::mem::take(&mut self.download);
        let dwn_cfg = cls(dwn_cfg);
        Self { download: dwn_cfg, ..self }
    }
}
