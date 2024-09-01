use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table: TableCreateStatement = table_auto(OcmUsers::Table)
            .col(uuid(OcmUsers::Id).unique_key().primary_key())
            .col(string(OcmUsers::RemoteId))
            .col(string(OcmUsers::Name))
            .col(string(OcmUsers::Email))
            .col(string(OcmUsers::Provider))
            .to_owned();

        manager.create_table(table).await?;

        let idx_email: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-ocm-users-email")
            .table(OcmUsers::Table)
            .col(OcmUsers::Email)
            .to_owned();

        manager.create_index(idx_email).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name("idx-ocm-users-email")
                .to_owned()
        ).await?;

        manager.drop_table(
            Table::drop()
                .table(OcmUsers::Table)
                .to_owned()
        ).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum OcmUsers {
    Table,
    Id,
    RemoteId,
    Name,
    Email,
    Provider,
}
