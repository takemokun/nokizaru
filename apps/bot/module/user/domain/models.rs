use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// ユーザーエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// ユーザー作成用DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
}

/// ユーザー更新用DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
}

impl User {
    pub fn new(dto: CreateUserDto) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            slack_user_id: dto.slack_user_id,
            slack_team_id: dto.slack_team_id,
            display_name: dto.display_name,
            email: dto.email,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, dto: UpdateUserDto) {
        if let Some(name) = dto.display_name {
            self.display_name = Some(name);
        }
        if let Some(email) = dto.email {
            self.email = Some(email);
        }
        if let Some(active) = dto.is_active {
            self.is_active = active;
        }
        self.updated_at = Utc::now();
    }
}
