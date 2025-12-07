use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{User, UpdateUserDto, UserService, UserError};

/// ユーザー取得または作成ユースケース
pub struct GetOrCreateUserUsecase {
    user_service: Arc<UserService>,
}

impl GetOrCreateUserUsecase {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    pub async fn execute(
        &self,
        slack_user_id: &str,
        slack_team_id: &str,
        display_name: Option<String>,
        email: Option<String>,
    ) -> Result<User, UserError> {
        tracing::debug!("Executing get or create user usecase for Slack user: {}", slack_user_id);
        self.user_service
            .get_or_create_user(slack_user_id, slack_team_id, display_name, email)
            .await
    }
}

/// ユーザー更新ユースケース
pub struct UpdateUserUsecase {
    user_service: Arc<UserService>,
}

impl UpdateUserUsecase {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    pub async fn execute(&self, id: Uuid, dto: UpdateUserDto) -> Result<User, UserError> {
        tracing::debug!("Executing update user usecase for user: {}", id);
        self.user_service.update_user(id, dto).await
    }
}

/// アクティブユーザー一覧取得ユースケース
pub struct ListActiveUsersUsecase {
    user_service: Arc<UserService>,
}

impl ListActiveUsersUsecase {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    pub async fn execute(&self) -> Result<Vec<User>, UserError> {
        tracing::debug!("Executing list active users usecase");
        self.user_service.list_active_users().await
    }
}
