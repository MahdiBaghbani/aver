use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table: TableCreateStatement = table_auto(OcmContacts::Table)
            .col(uuid(OcmContacts::Id).unique_key().primary_key())
            .col(uuid(OcmContacts::UserId))
            .col(uuid(OcmContacts::OcmUserId))
            .foreign_key(
                ForeignKey::create()
                    .name("fk-ocm-contacts-users")
                    .from(
                        OcmContacts::Table,
                        OcmContacts::UserId,
                    )
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-ocm-contacts-ocm-users")
                    .from(
                        OcmContacts::Table,
                        OcmContacts::OcmUserId,
                    )
                    .to(OcmUsers::Table, OcmUsers::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await?;

        let idx_user_id: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-ocm-contacts-user-id")
            .table(OcmContacts::Table)
            .col(OcmContacts::UserId)
            .to_owned();

        let idx_federated_user_id: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-ocm-contacts-ocm-user-id")
            .table(OcmContacts::Table)
            .col(OcmContacts::OcmUserId)
            .to_owned();

        manager.create_index(idx_user_id).await?;
        manager.create_index(idx_federated_user_id).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name("idx-ocm-contacts-user-id")
                .to_owned()
        ).await?;

        manager.drop_index(
            Index::drop()
                .name("idx-ocm-contacts-ocm-user-id")
                .to_owned()
        ).await?;

        manager.drop_table(
            Table::drop()
                .table(OcmContacts::Table)
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum OcmContacts {
    Table,
    Id,
    UserId,
    OcmUserId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum OcmUsers {
    Table,
    Id,
}
