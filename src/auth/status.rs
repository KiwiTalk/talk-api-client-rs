/*
 * Created on Mon Dec 06 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde_repr::*;

/// Known auth api response status codes
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum Status {
	Success = 0,
	InvalidPhoneNumber = 1,
	SuccessWithAccount = 10,
	SuccessWithDeviceChanged = 11,
	MismatchPassword = 12,
	ExceedLoginLimit = 13,
	MismatchPhoneNumber = 14,
	ExceedPhoneNumberCheckLimit = 15,
	NotExistAccount = 16,
	NeedCheckPhoneNumber = 20,
	NeedCheckQuiz = 25,
	DormantAccount = 26,
	RestrictedAccount = 27,
	LoginFailed = 30,
	NotVerifiedEmail = 31,
	MobileUnregistered = 32,
	UnknownPhoneNumber = 99,
	SuccessSameUser = 100,
	SuccessSameUserByMigration = 101,
	TooManyRequestADay = -20,
	TooManyRequestAtATime = -30,
	MismatchPasscode = -31,
	ExceedDailyRequestLimit = -32,
	ExceedDailyRequestLimitVoiceCall = -33,
	ExceedDailyRequestLimitWithoutToken = -34,
	DeviceNotRegistered = -100,
	AnotherLogon = -101,
	DeviceRegisterFailed = -102,
	InvalidDeviceRegister = -110,
	InvalidPasscode = -111,
	PasscodeRequestFailed = -112,
	NeedTermsAgree = -126,
	DeniedDeviceModel = -132,
	ResetStep = -940,
	NeedProtectorAgree = -991,
	AccountRestricted = -997,
	InvalidStageError = -998,
	UpgradeRequired = -999,
	VoiceCallOnly = -10002,
	AccessibilityArsOnly = -10003,
	MigrationFailure = -100001,
	InvalidToken = -100002,
}