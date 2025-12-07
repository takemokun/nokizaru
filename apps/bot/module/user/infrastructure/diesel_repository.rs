use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use shared_infrastructure::{DbPool, schema::users};
use crate::domain::{User, CreateUserDto, UpdateUserDto, UserRepository, UserError};

#[derive(Debug, Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: Uuid,
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserModel> for User {
    fn from(model: UserModel) -> Self {
        Self {
            id: model.id,
            slack_user_id: model.slack_user_id,
            slack_team_id: model.slack_team_id,
            display_name: model.display_name,
            email: model.email,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            slack_user_id: user.slack_user_id,
            slack_team_id: user.slack_team_id,
            display_name: user.display_name,
            email: user.email,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// Diesel実装のユーザーリポジトリ
pub struct DieselUserRepository {
    pool: DbPool,
}

impl DieselUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for DieselUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
        let mut conn = self.pool.get().await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let result = users::table
            .filter(users::id.eq(id))
            .select(UserModel::as_select())
            .first::<UserModel>(&mut conn)
            .await
            .optional()
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(result.map(User::from))
    }

    async fn find_by_slack_user_id(&self, slack_user_id: &str) -> Result<Option<User>, UserError> {
        let mut conn = self.pool.get().await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let result = users::table
            .filter(users::slack_user_id.eq(slack_user_id))
            .select(UserModel::as_select())
            .first::<UserModel>(&mut conn)
            .await
            .optional()
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(result.map(User::from))
    }

    async fn create(&self, dto: CreateUserDto) -> Result<User, UserError> {
        let mut conn = self.pool.get().await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let user = User::new(dto);
        let model = UserModel::from(user);

        let created = diesel::insert_into(users::table)
            .values(&model)
            .returning(UserModel::as_returning())
            .get_result::<UserModel>(&mut conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(User::from(created))
    }

    async fn update(&self, id: Uuid, dto: UpdateUserDto) -> Result<User, UserError> {
        let mut conn = self.pool.get().await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        #[derive(AsChangeset)]
        #[diesel(table_name = users)]
        struct UpdateData {
            display_name: Option<String>,
            email: Option<String>,
            is_active: Option<bool>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }

        let update_data = UpdateData {
            display_name: dto.display_name,
            email: dto.email,
            is_active: dto.is_active,
            updated_at: chrono::Utc::now(),
        };

        let updated = diesel::update(users::table.filter(users::id.eq(id)))
            .set(&update_data)
            .returning(UserModel::as_returning())
            .get_result::<UserModel>(&mut conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(User::from(updated))
    }

    async fn delete(&self, id: Uuid) -> Result<(), UserError> {
        let mut conn = self.pool.get().await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(&mut conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn list_active_users(&self) -> Result<Vec<User>, UserError> {
        let mut conn = self.pool.get().await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let results = users::table
            .filter(users::is_active.eq(true))
            .select(UserModel::as_select())
            .load::<UserModel>(&mut conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(results.into_iter().map(User::from).collect())
    }
}
