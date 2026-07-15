/// Holds all important config option that are important for configuring the downloader.
#[derive(Debug, Default)]
pub struct DwnConfig {
    /// Holds all options that are used when downloading a full channel.
    pub(crate) channel: ChannelConfig,
}

#[derive(Debug)]
pub struct ChannelConfig {
    /// Controls wether albums in channels should be downloaded or not.
    /// Set to true by default.
    pub(crate) albums: bool,
    /// Controls wether singles in channels should be downloaded or not.
    /// Set to true by default.
    pub(crate) singles: bool,
    /// Controls wether eps in channels should be downloaded or not.
    /// Set to true by default.
    pub(crate) eps: bool,
    /// Controls how many albums in channels should be downloaded.
    /// Always starts from the beginning.
    /// Set to max of u16 by default.
    pub(crate) max_albums: u16,
    /// Controls how many singles in channels should be downloaded.
    /// Always starts from the beginning.
    /// Set to max of u16 by default.
    pub(crate) max_singles: u16,
    /// Controls how many eps in channels should be downloaded.
    /// Always starts from the beginning.
    /// Set to max of u16 by default.
    pub(crate) max_eps: u16,
    /// Conrtols wether all eps, singles and albums should be merged into one big list of songs.
    /// Set to false by default.
    pub(crate) merge_songs: bool,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            albums: true,
            singles: true,
            eps: true,
            max_eps: u16::MAX,
            max_singles: u16::MAX,
            max_albums: u16::MAX,
            merge_songs: false,
        }
    }
}

impl DwnConfig {
    /// Consumes self as mutuble and takes a function that changes the inner channel config to a new
    /// channel config.
    /// Returns itself with the new channel config.
    pub fn channel(mut self, cls: impl Fn(ChannelConfig) -> ChannelConfig) -> Self {
        let channel_cfg = std::mem::take(&mut self.channel);
        let channel_cfg = cls(channel_cfg);
        Self { channel: channel_cfg, ..self }
    }
}

impl ChannelConfig {
    /// Consumes self as mutable and takes a download bool.
    /// Sets the albums value to the given download bool and returns self.
    pub fn albums(mut self, download: bool) -> Self {
        self.albums = download;
        self
    }

    /// Consumes self as mutable and takes a download bool.
    /// Sets the singles value to the given download bool and returns self.
    pub fn singles(mut self, download: bool) -> Self {
        self.singles = download;
        self
    }

    /// Consumes self as mutable and takes a download bool.
    /// Sets the eps value to the given download bool and returns self.
    pub fn eps(mut self, download: bool) -> Self {
        self.eps = download;
        self
    }

    /// Consumes self as mutable and takes the number of albums that should be downloaded.
    /// Changes max_albums to the given max value and returns self.
    pub fn max_albums(mut self, max: u16) -> Self {
        self.max_albums = max;
        self
    }

    /// Consumes self as mutable and takes the number of singles that should be downloaded.
    /// Changes max_singles to the given max value and returns self.
    pub fn max_singles(mut self, max: u16) -> Self {
        self.max_singles = max;
        self
    }

    /// Consumes self as mutable and takes the number of eps that should be downloaded.
    /// Changes max_eps to the given max value and returns self.
    pub fn max_eps(mut self, max: u16) -> Self {
        self.max_eps = max;
        self
    }

    /// Consumes self as mutable and takes a merge bool.
    /// Sets the merge value to the given merge bool and returns self.
    pub fn merge(mut self, merge: bool) -> Self {
        self.merge_songs = merge;
        self
    }
}
