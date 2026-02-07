use crate::{
    errors::CoreResult,
    models::user::{CreateUserInput, User},
    types::{
        auth::AuthContext,
        user::{PasswordHash, UserId, Username},
    },
};

#[async_trait::async_trait]
trait UserService {
    async fn create_user(&self, input: &CreateUserInput) -> CoreResult<()>;

    async fn get_by_auth(&self, auth: &AuthContext) -> CoreResult<User>;
    async fn get_by_id(&self, id: UserId) -> CoreResult<User>;
    async fn get_by_username(&self, username: Username) -> CoreResult<User>;

    async fn update_username(&self, auth: &AuthContext, new_username: Username) -> CoreResult<()>;
    async fn update_password(
        &self,
        auth: &AuthContext,
        old_pwd: String,
        new_pwd: String,
    ) -> CoreResult<()>;

    async fn delete_by_auth(&self, auth: &AuthContext) -> CoreResult<()>;
    async fn delete_user(&self, id: UserId, auth: &AuthContext) -> CoreResult<()>;
}

#[async_trait::async_trait]
trait UserRepository {
    async fn insert(&self, user: &User) -> CoreResult<()>;

    async fn get_by_id(&self, id: UserId) -> CoreResult<Option<User>>;
    async fn get_by_username(&self, username: &Username) -> CoreResult<Option<User>>;

    async fn update_username(&self, id: UserId, new_username: &Username) -> CoreResult<()>;
    async fn update_password(&self, id: UserId, new_password: &PasswordHash) -> CoreResult<()>;

    async fn delete_by_id(&self, id: UserId) -> CoreResult<()>;
}
