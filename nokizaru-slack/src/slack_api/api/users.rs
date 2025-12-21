use serde::{Deserialize, Serialize};
use crate::slack_api::{SlackApi, client::ClientResult};

/// ユーザー情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackUser {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub real_name: Option<String>,
    #[serde(default)]
    pub is_bot: bool,
}

/// users.list レスポンス
#[derive(Debug, Clone, Deserialize)]
pub struct UsersListResponse {
    pub members: Vec<SlackUser>,
}

impl SlackApi {
    /// ユーザーリスト取得
    pub async fn list_users(&self) -> ClientResult<Vec<SlackUser>> {
        let response: UsersListResponse = self
            .client
            .http_get("users.list", &[])
            .await?;

        Ok(response.members)
    }
}
