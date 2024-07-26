use crate::api::{API_BASE_URL, AUTH_URL, SCOPE_CORPORATE, SCOPE_PERSONAL};

#[derive(Clone, Debug)]
pub struct GigaChatConfig {
    pub auth_token: Option<String>,
    pub scope: String,
    pub auth_url: String,
    pub api_base_url: String,
}

impl GigaChatConfig {
    pub fn new() -> Self {
        Self {
            auth_token: Some(
                std::env::var("GIGACHAT_AUTH_TOKEN")
                    .expect("The environment variable GIGACHAT_AUTH_TOKEN is not set"),
            ),
            ..GigaChatConfig::default()
        }
    }

    pub fn builder() -> GigaChatConfigBuilder {
        GigaChatConfigBuilder::new()
    }
}

impl Default for GigaChatConfig {
    fn default() -> Self {
        Self {
            auth_token: None,
            scope: std::env::var("GIGACHAT_API_SCOPE").unwrap_or(SCOPE_PERSONAL.into()),
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

    pub fn auth_token<S>(mut self, token: S) -> Self
    where
        S: Into<String>,
    {
        self.auth_token = Some(token.into());
        self
    }

    pub fn scope<S>(mut self, scope: S) -> Self
    where
        S: Into<String>,
    {
        self.scope = Some(scope.into());
        self
    }

    pub fn with_scope_pers(self) -> Self {
        self.scope(SCOPE_PERSONAL)
    }

    pub fn with_scope_corp(self) -> Self {
        self.scope(SCOPE_CORPORATE)
    }

    pub fn auth_url<S>(mut self, url: S) -> Self
    where
        S: Into<String>,
    {
        self.auth_url = Some(url.into());
        self
    }

    pub fn api_base_url<S>(mut self, url: S) -> Self
    where
        S: Into<String>,
    {
        self.api_base_url = Some(url.into());
        self
    }

    pub fn build(self) -> GigaChatConfig {
        let config = GigaChatConfig::default();

        GigaChatConfig {
            auth_token: match self.auth_token {
                Some(token) => Some(token),
                None => config.auth_token,
            },
            scope: self.scope.unwrap_or(config.scope),
            auth_url: self.auth_url.unwrap_or(AUTH_URL.to_owned()),
            api_base_url: self.api_base_url.unwrap_or(API_BASE_URL.to_owned()),
        }
    }
}
