/*
 * Created on Tue Dec 07 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Deserialize, Serialize};

use crate::credential::TalkAuthCredential;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginData {
    #[serde(rename = "userId")]
    pub user_id: i64,

    #[serde(rename = "countryIso")]
    pub country_iso: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,

    #[serde(rename = "accountId")]
    pub account_id: i64,

    pub server_time: i64,

    #[serde(rename = "resetUserData")]
    pub reset_user_data: bool,

    pub story_url: Option<String>,

    #[serde(flatten)]
    pub credential: TalkAuthCredential,
    pub token_type: String,

    #[serde(rename = "autoLoginAccountId")]
    pub auto_login_account_id: String,
    #[serde(rename = "displayAccountId")]
    pub display_account_id: String,

    #[serde(rename = "mainDeviceAgentName")]
    pub main_device_agent_name: String,
    #[serde(rename = "mainDeviceAppVersion")]
    pub main_device_app_version: String,
}
