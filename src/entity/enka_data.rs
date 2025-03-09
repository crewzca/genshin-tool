use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Resbody {
    #[serde(rename = "playerInfo")]
    pub player_info: PlayerInfo,
    #[serde(rename = "avatarInfoList", default)]
    pub avatar_info_list: Vec<AvatarInfoList>,
    pub uid: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PlayerInfo {
    pub nickname: String,
    pub level: u32,
    #[serde(rename = "showAvatarInfoList", default)]
    pub show_avatar_info_list: Vec<ShowAvatarInfoList>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AvatarInfoList {
    pub avatarId: u32,
    #[serde(rename = "fightPropMap")]
    pub fight_prop_map: FightPropMap,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShowAvatarInfoList {
    #[serde(default)]
    pub chara_name: String,
    pub level: u32,
    #[serde(default)]
    pub talentLevel: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FightPropMap {
    #[serde(rename = "20")]
    pub ctk: f64,
    #[serde(rename = "22")]
    pub cdmg: f64,
    #[serde(rename = "23")]
    pub charge: f64,
    #[serde(rename = "26")]
    pub heal: f64,
    #[serde(rename = "28")]
    pub know: f64,
    #[serde(rename = "30")]
    pub physics: f64,
    #[serde(rename = "40")]
    pub flame: f64,
    #[serde(rename = "41")]
    pub thunder: f64,
    #[serde(rename = "42")]
    pub aqua: f64,
    #[serde(rename = "43")]
    pub grass: f64,
    #[serde(rename = "44")]
    pub wind: f64,
    #[serde(rename = "45")]
    pub rock: f64,
    #[serde(rename = "46")]
    pub ice: f64,
    #[serde(rename = "1010")]
    pub hp: f64,
    #[serde(rename = "2001")]
    pub atk: f64,
    #[serde(rename = "2002")]
    pub def: f64,
}
