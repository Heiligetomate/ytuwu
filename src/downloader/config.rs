use std::usize;

/// Holds all important config option that are important for configuring the downloader.
#[derive(Debug, Default)]
pub struct Config {
    /// Holds all options that are used when downloading a full channel.
    pub(crate) channel: ChannelConfig,
    /// Holds all options that are used when downloading anything such as chunk size.
    pub(crate) download: DwnConfig,
}

/// Controls all downloading processes and the values used within this process such as chunk size or
/// the amount of concurrently downloaded chunks at once.  
#[derive(Debug)]
pub struct DwnConfig {
    /// Controls the chunk size for the chunks that are used for downloading streams.
    /// Set to 1024 * 1024 by default.
    pub(crate) chunk_size: u64,
    /// Controls the maximum concurrently downloading chunks at once.
    /// Setting this to a very high number could cause the youtube api to stop sending responses or
    /// slowing down.
    /// Set to 8 by default.
    pub(crate) concurrent_chunks: u8,
    /// Controls the maximum concurrently downloading streams at once.
    /// Setting this to a high number could cause the youtube api to stop sending responses or
    /// slowinf down.
    /// The number of requests ar once can be calculated by multiplying this value with the
    /// chunk_size value.
    /// Set to 4 by default.
    pub(crate) concurrent_streams: u8,
    /// Randomizes the chunk size a bit so that the requests to the youtube api are less suspicious.
    /// Uses the randomize_range to do so.
    /// Set to false by default.
    pub(crate) randomize_size: bool,
    /// Defines the range for randomizing the chunk size.
    /// Does not affect anything if randomize_size is set to false.
    /// Set to 100 by default.
    pub(crate) randomize_range: u32,
}

/// Controls the whole downloading process of a channel such as maximum singles downloaded or wether
/// or not albums should be downloaded or not.
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
    /// Set to max of usize by default.
    pub(crate) max_albums: usize,
    /// Controls how many singles in channels should be downloaded.
    /// Always starts from the beginning.
    /// Set to max of usize by default.
    pub(crate) max_singles: usize,
    /// Controls how many eps in channels should be downloaded.
    /// Always starts from the beginning.
    /// Set to max of usize by default.
    pub(crate) max_eps: usize,
    /// Conrtols wether all eps, singles and albums should be merged into one big list of songs.
    /// Set to false by default.
    pub(crate) merge_songs: bool,
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

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            albums: true,
            singles: true,
            eps: true,
            max_eps: usize::MAX,
            max_singles: usize::MAX,
            max_albums: usize::MAX,
            merge_songs: false,
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
    pub fn max_albums(mut self, max: usize) -> Self {
        self.max_albums = max;
        self
    }

    /// Consumes self as mutable and takes the number of singles that should be downloaded.
    /// Changes max_singles to the given max value and returns self.
    pub fn max_singles(mut self, max: usize) -> Self {
        self.max_singles = max;
        self
    }

    /// Consumes self as mutable and takes the number of eps that should be downloaded.
    /// Changes max_eps to the given max value and returns self.
    pub fn max_eps(mut self, max: usize) -> Self {
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

impl DwnConfig {
    /// Consumes self as mutable and takes the new chunk size.
    /// Sets the chunk_size value to the given chunk size and returns self.
    fn chunk_size(mut self, chunk_size: u64) -> Self {
        self.chunk_size = chunk_size;
        self
    }

    /// Consumes self as mutable and takes the new number of concurrent chunks.
    /// Sets the concurrent_chunks value to the given concurrent chunks and returns self.
    fn concurrent_chunks(mut self, concurrent_chunks: u8) -> Self {
        self.concurrent_chunks = concurrent_chunks;
        self
    }

    /// Consumes self as mutable and takes the new number of concurrent streams.
    /// Sets the concurrent_streams value to the given number of concurrent streams and returns self.
    fn concurrent_streams(mut self, concurrent_streams: u8) -> Self {
        self.concurrent_streams = concurrent_streams;
        self
    }

    /// Consumes self as mutable and takes a bool that controls wether the chunk size should be
    /// randomized or not.
    fn randomize_size(mut self, randomize: bool) -> Self {
        self.randomize_size = randomize;
        self
    }

    /// Consumes self as mutable and takes the new chunk size randomize range.
    /// Sets the range value to the given range value and returns self.
    fn randomize_range(mut self, range: u32) -> Self {
        self.randomize_range = range;
        self
    }
}
