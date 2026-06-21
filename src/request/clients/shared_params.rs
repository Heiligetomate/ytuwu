type Header = (&'static str, &'static str);

/// Host language client parameter
pub(super) const HL: &str = "en";
/// Geolocation client parameter
pub(super) const GL: &str = "US";
/// Time zone client parameter
pub(super) const TIME_ZONE: &str = "UTC";
/// Time offset client parameter
pub(super) const UTC_OFFSET_MINUTES: &str = "0";

/// Origin header key
pub(super) const ORIGIN_HEADER_NAME: &str = "Origin";
/// Content-Type header key
pub(super) const CONTENT_TYPE_HEADER_NAME: &str = "Content-Type";
/// Client Version header key
pub(super) const CLIENT_VERSION_HEADER_NAME: &str = "X-YouTube-Client-Version";
/// Client name header key
pub(super) const CLIENT_NAME_HEADER_NAME: &str = "X-YouTube-Client-Name";
/// User agent header key
pub(super) const USER_AGENT_HEADER_NAME: &str = "User-Agent";

/// Origin header containing a header key and the youtube url
pub(super) const ORIGIN_HEADER: Header = (ORIGIN_HEADER_NAME, "https://www.youtube.com");
/// Content type header containing a header key and json as the content type
pub(super) const CONTENT_TYPE_HEADER: Header = (CONTENT_TYPE_HEADER_NAME, "application/json");
