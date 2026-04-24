pub type Header = (&'static str, &'static str);

pub const CLIENT_NAME:        &str = "ANDROID_VR"; 
pub const CLIENT_VERSION:     &str = "1.60.19";
pub const DEVICE_MAKE:        &str = "Oculus";
pub const DEVICE_MODEL:       &str = "Quest 2";
pub const HL:                 &str = "en";
pub const GL:                 &str = "US";
pub const TIMEZONE:           &str = "UTC";
pub const ANDROID_SDK_VERSION: u16 = 29;
pub const UTC_OFFSET_MINUTES:  u16 = 0; 



pub const ENDPOINT: &str               = "https://music.youtube.com/youtubei/v1/player";

pub const ORIGIN_HEADER: Header         = ("Origin", "https://music.youtube.com");

pub const CONTENT_TYPE_HEADER: Header   = ("Content-Type", "application/json"); 
pub const CLIENT_NAME_HEADER: Header    = ("X-YouTube-Client-Name", "28");
pub const CLIENT_VERSION_HEADER: Header = ("X-YouTube-Client-Version", "1.60.19"); 
pub const USER_AGENT_HEADER: Header     = ("User-Agent", "User-Agent: Mozilla/5.0 (Linux; Android 10; Quest 2) AppleWebKit/537.36 (KHTML, like Gecko) OculusBrowser/32.0.0.3.65 SamsungBrowser/4.3 Chrome/137.0.7151.61 Mobile VR Safari/537.36");

