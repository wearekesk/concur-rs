use crate::auth::TokenManager;
use crate::error::{ConcurApiError, ConcurError, Result};
use reqwest::{Method, RequestBuilder, Response, StatusCode, header};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};
use url::Url;

/// Default HTTP timeout for API calls.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

/// Configuration for the SAP Concur client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for the data center (e.g. `https://us.api.concursolutions.com`).
    pub base_url: String,
    /// HTTP request timeout.
    pub timeout: Duration,
    /// Maximum number of retries for transient failures.
    pub max_retries: u32,
    /// Initial retry backoff.
    pub retry_backoff: Duration,
    /// Optional idempotency key for supported write operations.
    pub idempotency_key: Option<String>,
    /// Default `Accept-Language` header value.
    pub accept_language: Option<String>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "https://us.api.concursolutions.com".to_string(),
            timeout: DEFAULT_TIMEOUT,
            max_retries: 3,
            retry_backoff: Duration::from_millis(500),
            idempotency_key: None,
            accept_language: None,
        }
    }
}

/// Shared HTTP client for SAP Concur APIs.
#[derive(Clone)]
pub struct ConcurClient {
    pub(crate) http: reqwest::Client,
    pub(crate) config: ClientConfig,
    pub(crate) token_manager: Option<Arc<dyn TokenManager + Send + Sync>>,
}

impl ConcurClient {
    /// Create a new client with the provided configuration.
    pub fn new(config: ClientConfig) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .http2_prior_knowledge()
            .build()?;
        Ok(Self {
            http,
            config,
            token_manager: None,
        })
    }

    /// Attach a token manager to the client.
    pub fn with_token_manager<T>(mut self, manager: T) -> Self
    where
        T: TokenManager + Send + Sync + 'static,
    {
        self.token_manager = Some(Arc::new(manager));
        self
    }

    /// Build a fully qualified URL from a path.
    pub fn url(&self, path: &str) -> Result<Url> {
        let base = Url::parse(&self.config.base_url)?;
        base.join(path).map_err(Into::into)
    }

    /// Start building an authenticated request.
    pub async fn request(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let url = self.url(path)?;
        let mut builder = self.http.request(method, url);

        builder = builder
            .header(header::ACCEPT, "application/json")
            .header("concur-correlationid", uuid::Uuid::new_v4().to_string());

        if let Some(lang) = &self.config.accept_language {
            builder = builder.header(header::ACCEPT_LANGUAGE, lang);
        }

        if let Some(token_manager) = &self.token_manager {
            let token = token_manager.access_token().await?;
            builder = builder.header(header::AUTHORIZATION, format!("Bearer {}", token));
        }

        Ok(builder)
    }

    /// Execute a request with retries and idempotency support.
    pub async fn execute<T>(&self, builder: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.execute_with_raw(builder)
            .await?
            .json::<T>()
            .await
            .map_err(Into::into)
    }

    /// Execute a request and return the raw response.
    pub async fn execute_with_raw(&self, builder: RequestBuilder) -> Result<Response> {
        let mut attempts = 0u32;
        let mut backoff = self.config.retry_backoff;

        loop {
            attempts += 1;
            let request = builder.try_clone().ok_or_else(|| {
                ConcurError::Other("Request body cannot be cloned for retry".to_string())
            })?;

            debug!(attempt = attempts, "Executing SAP Concur request");
            let response = request.send().await?;

            let status = response.status();
            if status.is_success() {
                return Ok(response);
            }

            if attempts > self.config.max_retries || !is_retryable(status) {
                return Err(parse_error(response).await);
            }

            warn!(
                status = %status,
                attempt = attempts,
                "Retryable SAP Concur response, backing off"
            );
            sleep(backoff).await;
            backoff *= 2;
        }
    }

    /// Execute a request and ignore the response body (for 204 No Content).
    pub async fn execute_unit(&self, builder: RequestBuilder) -> Result<()> {
        let response = self.execute_with_raw(builder).await?;
        if response.status() == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            // Consume body to avoid connection pool issues.
            let _ = response.text().await?;
            Ok(())
        }
    }

    /// Send a JSON body with the request.
    pub fn with_json<B: Serialize + ?Sized>(
        builder: RequestBuilder,
        body: &B,
    ) -> Result<RequestBuilder> {
        Ok(builder
            .header(header::CONTENT_TYPE, "application/json")
            .json(body))
    }
}

fn is_retryable(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::REQUEST_TIMEOUT
            | StatusCode::TOO_MANY_REQUESTS
            | StatusCode::INTERNAL_SERVER_ERROR
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT
    )
}

async fn parse_error(response: Response) -> ConcurError {
    let status = response.status();
    let correlation_id = response
        .headers()
        .get("concur-correlationid")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    let text = match response.text().await {
        Ok(t) => t,
        Err(e) => {
            return ConcurError::Http(e).with_correlation_id(correlation_id.unwrap_or_default());
        }
    };

    let message = serde_json::from_str::<ConcurApiError>(&text)
        .map(|e| e.to_string())
        .unwrap_or_else(|_| text);

    ConcurError::Api {
        status,
        message,
        correlation_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::StaticToken;
    use wiremock::matchers::{bearer_token, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn retryable_status_codes() {
        assert!(is_retryable(StatusCode::REQUEST_TIMEOUT));
        assert!(is_retryable(StatusCode::TOO_MANY_REQUESTS));
        assert!(is_retryable(StatusCode::INTERNAL_SERVER_ERROR));
        assert!(!is_retryable(StatusCode::BAD_REQUEST));
        assert!(!is_retryable(StatusCode::NOT_FOUND));
    }

    #[tokio::test]
    async fn request_includes_auth_and_correlation_headers() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v4.1/users"))
            .and(bearer_token("test-token"))
            .and(header("Accept", "application/json"))
            .and(wiremock::matchers::header_exists("concur-correlationid"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": "1"})))
            .mount(&server)
            .await;

        let config = ClientConfig {
            base_url: server.uri(),
            ..Default::default()
        };

        let client = ConcurClient::new(config)
            .unwrap()
            .with_token_manager(StaticToken::new("test-token"));

        let request = client
            .request(Method::GET, "/api/v4.1/users")
            .await
            .unwrap();
        let result: serde_json::Value = client.execute(request).await.unwrap();

        assert_eq!(result["id"], "1");
    }

    #[tokio::test]
    async fn retries_on_500_and_succeeds() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/retry"))
            .respond_with(ResponseTemplate::new(500))
            .up_to_n_times(1)
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/retry"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok": true})))
            .mount(&server)
            .await;

        let config = ClientConfig {
            base_url: server.uri(),
            retry_backoff: Duration::from_millis(10),
            ..Default::default()
        };

        let client = ConcurClient::new(config).unwrap();
        let request = client.request(Method::GET, "/retry").await.unwrap();
        let result: serde_json::Value = client.execute(request).await.unwrap();

        assert_eq!(result["ok"], true);
    }
}
