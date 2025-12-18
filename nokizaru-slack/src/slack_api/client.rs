use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::slack_api::error::SlackError;

pub type ClientResult<T> = std::result::Result<T, SlackError>;

pub struct SlackHttpClient {
    http_client: Client,
    token: String,
}

impl SlackHttpClient {
    pub fn new(token: String) -> Self {
        Self {
            http_client: Client::new(),
            token,
        }
    }

    pub async fn http_get<RS>(
        &self,
        method: &str,
        params: &[(&str, String)],
    ) -> ClientResult<RS>
    where
        RS: DeserializeOwned,
    {
        let url = format!("https://slack.com/api/{}", method);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .query(params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(SlackError::ApiError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let result: serde_json::Value = response.json().await?;

        // Slack API の ok フィールドをチェック
        if !result.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            let error = result
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error");
            return Err(SlackError::ApiError(error.to_string()));
        }

        // 型パラメータ RS に自動変換
        serde_json::from_value(result).map_err(|e| {
            SlackError::ParseError(e)
        })
    }

    pub async fn http_post<RQ, RS>(
        &self,
        method: &str,
        request: &RQ,
    ) -> ClientResult<RS>
    where
        RQ: Serialize,
        RS: DeserializeOwned,
    {
        let url = format!("https://slack.com/api/{}", method);

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(SlackError::ApiError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let result: serde_json::Value = response.json().await?;

        if !result.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            let error = result
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error");
            return Err(SlackError::ApiError(error.to_string()));
        }

        serde_json::from_value(result).map_err(|e| {
            SlackError::ParseError(e)
        })
    }
}
