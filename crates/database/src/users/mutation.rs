use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};
use uuid::Uuid;

use aver_database_entity::users;
use aver_database_entity::users::Model;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        first_name: String,
        last_name: String,
        username: String,
        password: String,
    ) -> Result<Model, DbErr> {
        users::ActiveModel {
            id: Set(Uuid::new_v4()),
            first_name: Set(first_name),
            last_name: Set(last_name),
            username: Set(username),
            password: Set(password),
            ..Default::default()
        }
            .insert(db)
            .await
    }
}
