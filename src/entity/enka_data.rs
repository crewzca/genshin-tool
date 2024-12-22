use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlayerInfo {
    pub nickname: String,
    pub level: i32,
}

#[derive(Deserialize, Debug)]
pub struct Resbody {
    #[serde(rename = "playerInfo")]
    pub player_info: PlayerInfo,
    pub uid: String,
}
