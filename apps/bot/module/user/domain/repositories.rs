use async_trait::async_trait;
use uuid::Uuid;
use super::{User, CreateUserDto, UpdateUserDto, UserError};

/// ユーザーリポジトリインターフェース
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError>;

    async fn find_by_slack_user_id(&self, slack_user_id: &str) -> Result<Option<User>, UserError>;

    async fn create(&self, dto: CreateUserDto) -> Result<User, UserError>;

    async fn update(&self, id: Uuid, dto: UpdateUserDto) -> Result<User, UserError>;

    async fn delete(&self, id: Uuid) -> Result<(), UserError>;

    async fn list_active_users(&self) -> Result<Vec<User>, UserError>;
}
