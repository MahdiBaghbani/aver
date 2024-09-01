use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table: TableCreateStatement = table_auto(OcmInviteTokens::Table)
            .col(uuid(OcmInviteTokens::Id).unique_key().primary_key())
            .col(uuid(OcmInviteTokens::UserId))
            .col(string_uniq(OcmInviteTokens::Token))
            .col(big_integer(OcmInviteTokens::ExpirationTime))
            .foreign_key(
                ForeignKey::create()
                    .name("fk-ocm-invite-tokens-users")
                    .from(
                        OcmInviteTokens::Table,
                        OcmInviteTokens::UserId,
                    )
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await?;

        let idx_token: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-ocm-invite-tokens-token")
            .table(OcmInviteTokens::Table)
            .col(OcmInviteTokens::Token)
            .to_owned();

        manager.create_index(idx_token).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name("idx-ocm-invite-tokens-token")
                .to_owned()
        ).await?;

        manager.drop_table(
            Table::drop()
                .table(OcmInviteTokens::Table)
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum OcmInviteTokens {
    Table,
    Id,
    UserId,
    Token,
    ExpirationTime,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
