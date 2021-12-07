/*
 * Created on Mon Dec 06 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

//! Known auth api response status codes

pub const INVALID_PHONE_NUMBER: i32 = 1;
pub const SUCCESS_WITH_ACCOUNT: i32 = 10;
pub const SUCCESS_WITH_DEVICE_CHANGED: i32 = 11;
pub const MISMATCH_PASSWORD: i32 = 12;
pub const EXCEED_LOGIN_LIMIT: i32 = 13;
pub const MISMATCH_PHONE_NUMBER: i32 = 14;
pub const EXCEED_PHONE_NUMBER_CHECK_LIMIT: i32 = 15;
pub const NOT_EXIST_ACCOUNT: i32 = 16;
pub const NEED_CHECK_PHONE_NUMBER: i32 = 20;
pub const NEED_CHECK_QUIZ: i32 = 25;
pub const DORMANT_ACCOUNT: i32 = 26;
pub const RESTRICTED_ACCOUNT: i32 = 27;
pub const LOGIN_FAILED: i32 = 30;
pub const NOT_VERIFIED_EMAIL: i32 = 31;
pub const MOBILE_UNREGISTERED: i32 = 32;
pub const UNKNOWN_PHONE_NUMBER: i32 = 99;
pub const SUCCESS_SAME_USER: i32 = 100;
pub const SUCCESS_SAME_USER_BY_MIGRATION: i32 = 101;
pub const TOO_MANY_REQUEST_A_DAY: i32 = -20;
pub const TOO_MANY_REQUEST_AT_A_TIME: i32 = -30;
pub const MISMATCH_PASSCODE: i32 = -31;
pub const EXCEED_DAILY_REQUEST_LIMIT: i32 = -32;
pub const EXCEED_DAILY_REQUEST_LIMIT_VOICECALL: i32 = -33;
pub const EXCEED_DAILY_REQUEST_LIMIT_WITHOUT_TOKEN: i32 = -34;
pub const DEVICE_NOT_REGISTERED: i32 = -100;
pub const ANOTHER_LOGON: i32 = -101;
pub const DEVICE_REGISTER_FAILED: i32 = -102;
pub const INVALID_DEVICE_REGISTER: i32 = -110;
pub const INVALID_PASSCODE: i32 = -111;
pub const PASSCODE_REQUEST_FAILED: i32 = -112;
pub const NEED_TERMS_AGREE: i32 = -126;
pub const DENIED_DEVICE_MODEL: i32 = -132;
pub const RESET_STEP: i32 = -940;
pub const NEED_PROTECTOR_AGREE: i32 = -991;
pub const ACCOUNT_RESTRICTED: i32 = -997;
pub const INVALID_STAGE_ERROR: i32 = -998;
pub const UPGRADE_REQUIRED: i32 = -999;
pub const VOICE_CALL_ONLY: i32 = -10002;
pub const ACCESSIBILITY_ARS_ONLY: i32 = -10003;
pub const MIGRATION_FAILURE: i32 = -100001;
pub const INVAILD_TOKEN: i32 = -100002;
