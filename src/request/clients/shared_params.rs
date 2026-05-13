type Header = (&'static str, &'static str);

// Header names
pub const ORIGIN_HEADER_NAME: &str = "Origin";
pub const CONTENT_TYPE_HEADER_NAME: &str = "Content-Type";
pub const CLIENT_VERSION_HEADER_NAME: &str = "X-YouTube-Client-Version";
pub const CLIENT_NAME_HEADER_NAME: &str = "X-YouTube-Client-Name";
pub const USER_AGENT_HEADER_NAME: &str = "User-Agent";

// Headers
pub const ORIGIN_HEADER: Header = (ORIGIN_HEADER_NAME, "https://www.youtube.com");
pub const CONTENT_TYPE_HEADER: Header = (CONTENT_TYPE_HEADER_NAME, "application/json");
