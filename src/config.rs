use crate::api::{API_BASE_URL, AUTH_URL, SCOPE_CORP, SCOPE_PERS};

#[derive(Clone)]
pub struct GigaChatConfig {
    pub auth_token: String,
    pub scope: String,
    pub auth_url: String,
    pub api_base_url: String,
}

impl GigaChatConfig {
    pub fn new() -> Self {
        GigaChatConfig::default()
    }

    pub fn builder() -> GigaChatConfigBuilder {
        GigaChatConfigBuilder::new()
    }
}

impl Default for GigaChatConfig {
    fn default() -> Self {
        Self {
            auth_token: std::env::var("GIGACHAT_AUTH_TOKEN")
                .expect("The environment variable GIGACHAT_AUTH_TOKEN is not set"),
            scope: std::env::var("GIGACHAT_API_SCOPE").unwrap_or(SCOPE_PERS.into()),
            auth_url: AUTH_URL.to_owned(),
            api_base_url: API_BASE_URL.to_owned(),
        }
    }
}

#[derive(Default)]
pub struct GigaChatConfigBuilder {
    auth_token: Option<String>,
    scope: Option<String>,
    auth_url: Option<String>,
    api_base_url: Option<String>,
}

impl GigaChatConfigBuilder {
    pub fn new() -> Self {
        GigaChatConfigBuilder {
            auth_token: None,
            scope: None,
            auth_url: None,
            api_base_url: None,
        }
    }

    pub fn auth_token(mut self, token: String) -> Self {
        self.auth_token = token.into();
        self
    }

    pub fn scope(mut self, scope: String) -> Self {
        self.scope = scope.into();
        self
    }

    pub fn with_scope_pers(self) -> Self {
        self.scope(SCOPE_PERS.to_owned())
    }

    pub fn with_scope_corp(self) -> Self {
        self.scope(SCOPE_CORP.to_owned())
    }

    pub fn auth_url(mut self, url: String) -> Self {
        self.auth_url = url.into();
        self
    }

    pub fn api_base_url(mut self, url: String) -> Self {
        self.api_base_url = url.into();
        self
    }

    pub fn build(self) -> GigaChatConfig {
        let config = GigaChatConfig::default();

        GigaChatConfig {
            auth_token: self.auth_token.unwrap_or(config.auth_token),
            scope: self.scope.unwrap_or(config.scope),
            auth_url: self.auth_url.unwrap_or(AUTH_URL.to_owned()),
            api_base_url: self.api_base_url.unwrap_or(API_BASE_URL.to_owned()),
        }
    }
}
