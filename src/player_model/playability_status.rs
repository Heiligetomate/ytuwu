use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayabilityStatusValue {
    Ok,
    LoginRequired,
    Error,
    Unplayable,
    LiveStreamOffline,
    ContentCheckRequired,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayabilityStatus {
    pub status: PlayabilityStatusValue,
    reason: Option<String>,
    playable_in_embed: Option<bool>,
    context_params: Option<String>,
}
