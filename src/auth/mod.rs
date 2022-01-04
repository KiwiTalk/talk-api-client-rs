/*
 * Created on Mon Dec 06 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub mod resources;
pub mod xvc;
mod status;

pub use status::*;

use std::borrow::Cow;

use reqwest::{header, Client, RequestBuilder, Url};

use crate::{agent::TalkApiAgent, response::TalkStatusResponse, ApiURL, ApiResult};

use self::{resources::LoginData, xvc::XVCHasher};

use serde::Serialize;

/// Internal talk api wrapper for authentication
#[derive(Debug)]
pub struct TalkAuthClient<Xvc> {
    pub config: AuthClientConfig<'static>,

    url: ApiURL,
    xvc_hasher: Xvc,

    client: Client,
}

impl<Xvc: XVCHasher> TalkAuthClient<Xvc> {
    pub fn new(config: AuthClientConfig<'static>, xvc_hasher: Xvc) -> Self {
        Self::new_with_url(
            config,
            ApiURL::new("https", "katalk.kakao.com").unwrap(),
            xvc_hasher,
        )
    }

    pub fn new_with_url(config: AuthClientConfig<'static>, url: ApiURL, xvc_hasher: Xvc) -> Self {
        Self {
            config,

            url,
            xvc_hasher,

            client: Client::new(),
        }
    }

    fn build_auth_request(&self, builder: RequestBuilder, email: &str) -> RequestBuilder {
        let user_agent = self
            .config
            .agent
            .get_user_agent(&self.config.version, &self.config.language);

        let mut builder = builder
            .header(header::USER_AGENT, &user_agent)
            .header(
                "A",
                &format!(
                    "{}/{}/{}",
                    self.config.agent.agent(),
                    self.config.version,
                    self.config.language
                ),
            )
            .header(header::ACCEPT, "*/*")
            .header(header::ACCEPT_LANGUAGE, &self.config.language as &str)
            .header("X-VC", self.hash_auth_xvc(&user_agent, email));

        if let Some(host) = self.url.host_str() {
            builder = builder.header(header::HOST, host);
        }

        builder
    }

    fn build_url(&self, end_point: &str) -> Url {
        self.url
            .join(&format!("{}/{}", self.config.agent.agent(), end_point))
            .unwrap()
    }

    fn hash_auth_xvc(&self, user_agent: &str, email: &str) -> String {
        let full_hash = self
            .xvc_hasher
            .full_xvc_hash(&self.config.device.uuid_string_base64, user_agent, email);

        hex::encode(&full_hash[..8])
    }

    fn build_auth_form<'a>(&'a self, email: &'a str, password: &'a str) -> AuthRequestForm<'a> {
        AuthRequestForm {
            email,
            password,
            device_uuid: &self.config.device.uuid_string_base64,
            device_name: &self.config.device.name,
            model_name: self.config.device.model.as_deref(),
        }
    }

    pub async fn login<'a>(
        &'a self,
        method: &LoginMethod<'a>,
        forced: bool,
    ) -> ApiResult<TalkStatusResponse<LoginData>> {
        let response = match method {
            LoginMethod::Account(account_form) => {
                #[derive(Serialize)]
                struct LoginRequestForm<'a> {
                    #[serde(flatten)]
                    auth: AuthRequestForm<'a>,
                    forced: bool,
                }

                self.build_auth_request(
                    self.client.post(self.build_url("account/login.json")),
                    &account_form.email,
                )
                .form(&LoginRequestForm {
                    auth: self.build_auth_form(&account_form.email, &account_form.password),
                    forced,
                })
            }

            LoginMethod::Token(token_form) => {
                #[derive(Serialize)]
                struct TokenLoginRequestForm<'a> {
                    #[serde(flatten)]
                    auth: AuthRequestForm<'a>,
                    auto_login: bool,
                    autowithlock: bool,
                    forced: bool,
                }

                self.build_auth_request(
                    self.client.post(self.build_url("account/login.json")),
                    &token_form.email,
                )
                .form(&TokenLoginRequestForm {
                    auth: self.build_auth_form(&token_form.email, &token_form.auto_login_token),
                    auto_login: true,
                    autowithlock: token_form.locked,
                    forced,
                })
            }
        }
        .send()
        .await?;

        Ok(response.json().await?)
    }

    pub async fn request_passcode<'a>(
        &'a self,
        account_form: &AccountLoginForm<'a>,
    ) -> ApiResult<TalkStatusResponse<()>> {
        let response = self
            .build_auth_request(
                self.client
                    .post(self.build_url("account/request_passcode.json")),
                &account_form.email,
            )
            .form(&self.build_auth_form(&account_form.email, &account_form.password))
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn register_device<'a>(
        &'a self,
        passcode: &str,
        account_form: &AccountLoginForm<'a>,
        permanent: bool,
    ) -> ApiResult<TalkStatusResponse<()>> {
        #[derive(Serialize)]
        struct RegisterDeviceForm<'a> {
            #[serde(flatten)]
            auth: AuthRequestForm<'a>,
            passcode: &'a str,
            permanent: bool,
        }

        let response = self
            .build_auth_request(
                self.client
                    .post(self.build_url("account/register_device.json")),
                &account_form.email,
            )
            .form(&RegisterDeviceForm {
                auth: self.build_auth_form(&account_form.email, &account_form.password),
                passcode,
                permanent,
            })
            .send()
            .await?;

        Ok(response.json().await?)
    }
}

#[derive(Serialize)]
struct AuthRequestForm<'a> {
    email: &'a str,
    password: &'a str,
    device_uuid: &'a str,
    device_name: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    model_name: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct AuthClientConfig<'a> {
    pub device: AuthDeviceConfig<'a>,

    pub language: Cow<'a, str>,
    pub version: Cow<'a, str>,

    pub agent: TalkApiAgent<'a>,
}

impl AuthClientConfig<'static> {
    pub const fn new_const(
        device: AuthDeviceConfig<'static>,
        language: &'static str,
        version: &'static str,
        agent: TalkApiAgent<'static>,
    ) -> Self {
        Self {
            device,
            language: Cow::Borrowed(language),
            version: Cow::Borrowed(version),
            agent,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthDeviceConfig<'a> {
    pub name: Cow<'a, str>,
    pub model: Option<Cow<'a, str>>,
    pub uuid_string_base64: Cow<'a, str>,
}

impl<'a> AuthDeviceConfig<'a> {
    pub const fn new(name: Cow<'a, str>, model: Option<Cow<'a, str>>, uuid: Cow<'a, str>) -> Self {
        Self { name, uuid_string_base64: uuid, model }
    }

    pub const fn new_pc(name: Cow<'a, str>, uuid: Cow<'a, str>) -> Self {
        Self {
            name,
            uuid_string_base64: uuid,
            model: None,
        }
    }
}

impl AuthDeviceConfig<'static> {
    pub const fn new_const_pc(name: &'static str, uuid: &'static str) -> Self {
        Self {
            name: Cow::Borrowed(name),
            uuid_string_base64: Cow::Borrowed(uuid),
            model: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccountLoginForm<'a> {
    pub email: Cow<'a, str>,
    pub password: Cow<'a, str>,
}

impl AccountLoginForm<'static> {
    pub const fn new_const(email: &'static str, password: &'static str) -> Self {
        Self {
            email: Cow::Borrowed(email),
            password: Cow::Borrowed(password),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenLoginForm<'a> {
    pub email: Cow<'a, str>,
    pub auto_login_token: Cow<'a, str>,

    pub locked: bool,
}

impl TokenLoginForm<'static> {
    pub const fn new_const(
        email: &'static str,
        auto_login_token: &'static str,
        locked: bool,
    ) -> Self {
        Self {
            email: Cow::Borrowed(email),
            auto_login_token: Cow::Borrowed(auto_login_token),
            locked,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LoginMethod<'a> {
    Account(AccountLoginForm<'a>),
    Token(TokenLoginForm<'a>),
}
