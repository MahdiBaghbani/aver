use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};
use uuid::Uuid;

use aver_database_entity::ocm_invite_tokens;

pub struct Mutation;

impl Mutation {
    pub async fn create_token(
        database: &DbConn,
        user_id: Uuid,
        token: String,
        expiration_time: i64
    ) -> Result<ocm_invite_tokens::Model, DbErr> {
        ocm_invite_tokens::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            token: Set(token),
            expiration_time: Set(expiration_time),
            ..Default::default()
        }
            .insert(database)
            .await
    }
}
