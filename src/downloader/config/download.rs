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
