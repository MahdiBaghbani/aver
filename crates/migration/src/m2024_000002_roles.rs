use sea_orm_migration::{prelude::*, schema::*};
use crate::m2024_000001_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table: TableCreateStatement = table_auto(Roles::Table)
            .col(uuid(Roles::Id).unique_key().primary_key())
            .col(string(Roles::Name))
            .col(uuid(Roles::CreatedBy))
            .col(uuid(Roles::UpdatedBy))
            .to_owned();

        manager.create_table(table).await?;

        let idx_name: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-role-name")
            .table(Users::Table)
            .col(Users::Username)
            .to_owned();

        manager.create_index(idx_name).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name("idx-role-name")
                .to_owned()
        ).await?;
        
        manager.drop_table(
            Table::drop()
                .table(Roles::Table)
                .to_owned()
        ).await?;
        
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    Name,
    CreatedBy,
    UpdatedBy,
}
