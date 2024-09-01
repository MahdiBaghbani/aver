use crate::m2024_000001_users::Users;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table: TableCreateStatement = table_auto(Organizations::Table)
            .col(uuid(Organizations::Id).unique_key().primary_key())
            .col(string(Organizations::Name))
            .col(uuid_null(Organizations::CreatedBy))
            .col(uuid_null(Organizations::UpdatedBy))
            .to_owned();

        manager.create_table(table).await?;

        let idx_name: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-organization-name")
            .table(Users::Table)
            .col(Users::Username)
            .to_owned();

        manager.create_index(idx_name).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name("idx-organization-name")
                .to_owned()
        ).await?;

        manager.drop_table(
            Table::drop()
                .table(Organizations::Table)
                .to_owned()
        ).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
    Name,
    CreatedBy,
    UpdatedBy,
}
