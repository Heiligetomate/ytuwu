type Header = (&'static str, &'static str);

/// Host language client parameter
pub(super) const HL: &str = "en";
pub(super) const GL: &str = "US";
pub(super) const TIME_ZONE: &str = "UTC";
pub(super) const UTC_OFFSET_MINUTES: &str = "0";

// Header names
pub(super) const ORIGIN_HEADER_NAME: &str = "Origin";
pub(super) const CONTENT_TYPE_HEADER_NAME: &str = "Content-Type";
pub(super) const CLIENT_VERSION_HEADER_NAME: &str = "X-YouTube-Client-Version";
pub(super) const CLIENT_NAME_HEADER_NAME: &str = "X-YouTube-Client-Name";
pub(super) const USER_AGENT_HEADER_NAME: &str = "User-Agent";

// Headers
pub(super) const ORIGIN_HEADER: Header = (ORIGIN_HEADER_NAME, "https://www.youtube.com");
pub(super) const CONTENT_TYPE_HEADER: Header = (CONTENT_TYPE_HEADER_NAME, "application/json");
