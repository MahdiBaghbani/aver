use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table: TableCreateStatement = table_auto(UserOrgRoles::Table)
            .col(uuid(UserOrgRoles::Id).unique_key().primary_key())
            .col(uuid(UserOrgRoles::UserId))
            .col(uuid(UserOrgRoles::OrganizationId))
            .col(uuid(UserOrgRoles::RoleId))
            .foreign_key(
                ForeignKey::create()
                    .name("fk-user-org-roles-users")
                    .from(UserOrgRoles::Table, UserOrgRoles::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-user-org-roles-organizations")
                    .from(UserOrgRoles::Table, UserOrgRoles::OrganizationId)
                    .to(Organizations::Table, Organizations::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-user-org-roles-roles")
                    .from(UserOrgRoles::Table, UserOrgRoles::RoleId)
                    .to(Roles::Table, Roles::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await?;

        let idx_user_id: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-user-org-roles-user-id")
            .table(UserOrgRoles::Table)
            .col(UserOrgRoles::UserId)
            .to_owned();

        let idx_organization_id: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-user-org-roles-organization-id")
            .table(UserOrgRoles::Table)
            .col(UserOrgRoles::OrganizationId)
            .to_owned();

        let idx_role_id: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-user-org-roles-role-id")
            .table(UserOrgRoles::Table)
            .col(UserOrgRoles::RoleId)
            .to_owned();

        manager.create_index(idx_user_id).await?;
        manager.create_index(idx_organization_id).await?;
        manager.create_index(idx_role_id).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name("idx-user-org-roles-user-id")
                .to_owned()
        ).await?;

        manager.drop_index(
            Index::drop()
                .name("idx-user-org-roles-organization-id")
                .to_owned()
        ).await?;

        manager.drop_index(
            Index::drop()
                .name("idx-user-org-roles-role-id")
                .to_owned()
        ).await?;

        manager.drop_table(
            Table::drop()
                .table(UserOrgRoles::Table)
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum UserOrgRoles {
    Table,
    Id,
    UserId,
    OrganizationId,
    RoleId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
}
