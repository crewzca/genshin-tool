use std::default;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Resbody {
    #[serde(rename = "playerInfo")]
    pub player_info: PlayerInfo,
    #[serde(rename = "avatarInfoList", default)]
    pub avatar_info_list: Vec<AvatarInfoList>,
    #[serde(rename = "showAvatarInfoList", default)]
    pub show_avatar_info_list: Vec<ShowAvatarInfoList>,
    pub uid: String,
}

#[derive(Deserialize, Debug)]
pub struct PlayerInfo {
    pub nickname: String,
    pub level: u32,
}

#[derive(Deserialize, Debug)]
pub struct AvatarInfoList {
    #[serde(rename = "fightPropMap")]
    pub fight_prop_map: FightPropMap,
}

#[derive(Deserialize, Debug)]
pub struct ShowAvatarInfoList {
    pub avatarId: u32,
    pub level: u32,
    #[serde(default)]
    pub talentLevel: u32,
    pub energyType: u32,
}

#[derive(Deserialize, Debug)]
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
