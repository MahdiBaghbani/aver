use sea_orm::*;
use uuid::Uuid;

use aver_database_entity::{users, users::Entity as Users};

pub struct Query;

impl Query {
    pub async fn find_user_by_id(
        database: &DbConn,
        id: Uuid,
    ) -> Result<Option<users::Model>, DbErr> {
        Users::find_by_id(id)
            .one(database)
            .await
    }

    pub async fn find_user_by_username(
        database: &DbConn,
        username: &str,
    ) -> Result<Option<users::Model>, DbErr> {
        Users::find()
            .filter(
                users::Column::Username.eq(username)
            )
            .one(database)
            .await
    }
}
