use std::sync::Arc;
use uuid::Uuid;

use super::{User, CreateUserDto, UpdateUserDto, UserRepository, UserError};

/// ユーザー管理ドメインサービス
pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    /// ユーザーを取得または作成
    pub async fn get_or_create_user(
        &self,
        slack_user_id: &str,
        slack_team_id: &str,
        display_name: Option<String>,
        email: Option<String>,
    ) -> Result<User, UserError> {
        // 既存ユーザーを検索
        if let Some(user) = self.repository.find_by_slack_user_id(slack_user_id).await? {
            return Ok(user);
        }

        // 新規ユーザー作成
        let create_dto = CreateUserDto {
            slack_user_id: slack_user_id.to_string(),
            slack_team_id: slack_team_id.to_string(),
            display_name,
            email,
        };

        self.repository.create(create_dto).await
    }

    /// ユーザー情報更新
    pub async fn update_user(
        &self,
        id: Uuid,
        dto: UpdateUserDto,
    ) -> Result<User, UserError> {
        // ユーザーの存在確認
        if self.repository.find_by_id(id).await?.is_none() {
            return Err(UserError::NotFound(id));
        }

        self.repository.update(id, dto).await
    }

    /// アクティブユーザー一覧取得
    pub async fn list_active_users(&self) -> Result<Vec<User>, UserError> {
        self.repository.list_active_users().await
    }
}
