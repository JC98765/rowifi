use super::{BlacklistActionType, GuildType};
use crate::{
    bind::{BackupAssetBind, BackupCustomBind, BackupGroupBind, BackupRankBind},
    blacklist::Blacklist,
    events::EventType,
};

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BackupGuild {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    #[serde(rename = "UserId")]
    pub user_id: i64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Prefix")]
    pub command_prefix: Option<String>,

    #[serde(rename = "Settings")]
    pub settings: BackupGuildSettings,

    #[serde(rename = "VerificationRole")]
    pub verification_role: Option<String>,

    #[serde(rename = "VerifiedRole")]
    pub verified_role: Option<String>,

    #[serde(rename = "Rankbinds")]
    pub rankbinds: Vec<BackupRankBind>,

    #[serde(rename = "Groupbinds")]
    pub groupbinds: Vec<BackupGroupBind>,

    #[serde(rename = "Custombinds", default)]
    pub custombinds: Vec<BackupCustomBind>,

    #[serde(rename = "Assetbinds", default)]
    pub assetbinds: Vec<BackupAssetBind>,

    #[serde(rename = "Blacklists", default)]
    pub blacklists: Vec<Blacklist>,

    #[serde(rename = "RegisteredGroups", default)]
    pub registered_groups: Vec<i64>,

    #[serde(rename = "EventTypes", default)]
    pub event_types: Vec<EventType>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BackupGuildSettings {
    #[serde(rename = "AutoDetection")]
    pub auto_detection: bool,

    #[serde(rename = "Type")]
    pub guild_type: GuildType,

    #[serde(rename = "BlacklistAction", default)]
    pub blacklist_action: BlacklistActionType,

    #[serde(rename = "UpdateOnJoin", default)]
    pub update_on_join: bool,

    #[serde(rename = "AdminRoles", default)]
    pub admin_roles: Vec<String>,

    #[serde(rename = "TrainerRoles", default)]
    pub trainer_roles: Vec<String>,

    #[serde(rename = "BypassRoles", default)]
    pub bypass_roles: Vec<String>,

    #[serde(rename = "NicknameBypassRoles", default)]
    pub nickname_bypass_roles: Vec<String>,

    #[serde(rename = "LogChannel", default)]
    pub log_channel: Option<String>,
}
