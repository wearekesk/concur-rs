use crate::error::{ConcurError, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// OAuth2 token response from SAP Concur.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub geolocation: Option<String>,
    pub id_token: Option<String>,
    #[serde(skip)]
    pub obtained_at: Option<Instant>,
}

impl Token {
    /// Returns true if the access token is expired or about to expire within the buffer.
    pub fn is_expired(&self, buffer: Duration) -> bool {
        match self.obtained_at {
            Some(t) => {
                let elapsed = t.elapsed();
                let lifetime = Duration::from_secs(self.expires_in);
                elapsed + buffer >= lifetime
            }
            None => true,
        }
    }
}

/// Trait for token management strategies.
#[async_trait]
pub trait TokenManager {
    /// Return a valid access token, refreshing if necessary.
    async fn access_token(&self) -> Result<String>;
}

/// Static token manager that always returns the same token.
pub struct StaticToken {
    token: String,
}

impl StaticToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

#[async_trait]
impl TokenManager for StaticToken {
    async fn access_token(&self) -> Result<String> {
        Ok(self.token.clone())
    }
}

/// OAuth2 password / client-credentials grant token manager with refresh support.
pub struct OAuth2TokenManager {
    http: reqwest::Client,
    token_url: String,
    client_id: String,
    client_secret: String,
    grant: Grant,
    state: Mutex<TokenState>,
}

struct TokenState {
    token: Option<Token>,
}

#[derive(Debug, Clone)]
pub enum Grant {
    /// Client credentials grant (company / application token).
    ClientCredentials { scope: Option<String> },
    /// Password grant.
    Password {
        username: String,
        password: String,
        credtype: Option<String>,
        scope: Option<String>,
    },
    /// Refresh token grant.
    RefreshToken {
        refresh_token: String,
        scope: Option<String>,
    },
}

impl OAuth2TokenManager {
    pub fn new(
        token_url: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        grant: Grant,
    ) -> Self {
        Self {
            http: reqwest::Client::new(),
            token_url: token_url.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            grant,
            state: Mutex::new(TokenState { token: None }),
        }
    }

    async fn fetch_token(&self) -> Result<Token> {
        let mut params = vec![
            ("client_id", self.client_id.clone()),
            ("client_secret", self.client_secret.clone()),
        ];

        match &self.grant {
            Grant::ClientCredentials { scope } => {
                params.push(("grant_type", "client_credentials".to_string()));
                if let Some(s) = scope {
                    params.push(("scope", s.clone()));
                }
            }
            Grant::Password {
                username,
                password,
                credtype,
                scope,
            } => {
                params.push(("grant_type", "password".to_string()));
                params.push(("username", username.clone()));
                params.push(("password", password.clone()));
                if let Some(c) = credtype {
                    params.push(("credtype", c.clone()));
                }
                if let Some(s) = scope {
                    params.push(("scope", s.clone()));
                }
            }
            Grant::RefreshToken {
                refresh_token,
                scope,
            } => {
                params.push(("grant_type", "refresh_token".to_string()));
                params.push(("refresh_token", refresh_token.clone()));
                if let Some(s) = scope {
                    params.push(("scope", s.clone()));
                }
            }
        }

        let response = self
            .http
            .post(&self.token_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(ConcurError::Auth(format!("Token request failed: {}", text)));
        }

        let mut token: Token = response.json().await?;
        token.obtained_at = Some(Instant::now());
        Ok(token)
    }
}

#[async_trait]
impl TokenManager for OAuth2TokenManager {
    async fn access_token(&self) -> Result<String> {
        {
            let state = self
                .state
                .lock()
                .map_err(|e| ConcurError::Other(e.to_string()))?;
            if let Some(token) = &state.token
                && !token.is_expired(Duration::from_secs(60))
            {
                return Ok(token.access_token.clone());
            }
        }

        let token = self.fetch_token().await?;
        let access = token.access_token.clone();

        let mut state = self
            .state
            .lock()
            .map_err(|e| ConcurError::Other(e.to_string()))?;
        state.token = Some(token);
        Ok(access)
    }
}

/// Revoke all refresh tokens for a user/application.
pub async fn revoke_tokens(
    http: &reqwest::Client,
    base_url: &str,
    access_token: &str,
) -> Result<()> {
    let url = format!("{}/app-mgmt/v0/connections", base_url.trim_end_matches('/'));
    let response = http
        .delete(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let text = response.text().await.unwrap_or_default();
        Err(ConcurError::Auth(format!("Revoke failed: {}", text)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn token_expiry_with_buffer() {
        let mut token = Token {
            access_token: "abc".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 300,
            refresh_token: None,
            scope: None,
            geolocation: None,
            id_token: None,
            obtained_at: Some(Instant::now() - Duration::from_secs(200)),
        };

        assert!(!token.is_expired(Duration::from_secs(60)));

        token.obtained_at = Some(Instant::now() - Duration::from_secs(245));
        assert!(token.is_expired(Duration::from_secs(60)));
    }

    #[test]
    fn token_expired_when_no_obtained_at() {
        let token = Token {
            access_token: "abc".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 300,
            refresh_token: None,
            scope: None,
            geolocation: None,
            id_token: None,
            obtained_at: None,
        };

        assert!(token.is_expired(Duration::from_secs(60)));
    }

    #[tokio::test]
    async fn static_token_returns_value() {
        let manager = StaticToken::new("test-token");
        assert_eq!(manager.access_token().await.unwrap(), "test-token");
    }
}
