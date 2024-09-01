use crate::m2024_000001_users::Users;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table: TableCreateStatement = table_auto(Permissions::Table)
            .col(uuid(Permissions::Id).unique_key().primary_key())
            .col(string(Permissions::Name))
            .col(uuid(Permissions::CreatedBy))
            .col(uuid(Permissions::UpdatedBy))
            .to_owned();

        manager.create_table(table).await?;

        let idx_name: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-permission-name")
            .table(Users::Table)
            .col(Users::Username)
            .to_owned();

        manager.create_index(idx_name).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name("idx-permission-name")
                .to_owned()
        ).await?;

        manager.drop_table(
            Table::drop()
                .table(Permissions::Table)
                .to_owned()
        ).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Permissions {
    Table,
    Id,
    Name,
    CreatedBy,
    UpdatedBy,
}
