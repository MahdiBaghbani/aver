use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table: TableCreateStatement = table_auto(Users::Table)
            .col(uuid(Users::Id).unique_key().primary_key())
            .col(string(Users::FirstName))
            .col(string(Users::LastName))
            .col(string_uniq(Users::Username))
            .col(string(Users::Password))
            .col(uuid(Users::CreatedBy))
            .col(uuid(Users::UpdatedBy))
            .to_owned();

        manager.create_table(table).await?;

        let idx_username: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-username")
            .table(Users::Table)
            .col(Users::Username)
            .to_owned();

        manager.create_index(idx_username).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name("idx-username")
                .to_owned()
        ).await?;

        manager.drop_table(
            Table::drop()
                .table(Users::Table)
                .to_owned()
        ).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    Username,
    Password,
    CreatedBy,
    UpdatedBy,
}
