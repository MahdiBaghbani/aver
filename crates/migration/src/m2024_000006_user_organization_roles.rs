use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table: TableCreateStatement = table_auto(UserOrganizationRoles::Table)
            .col(uuid(UserOrganizationRoles::Id).unique_key().primary_key())
            .col(uuid(UserOrganizationRoles::UserId))
            .col(uuid(UserOrganizationRoles::OrganizationId))
            .col(uuid(UserOrganizationRoles::RoleId))
            .col(uuid_null(UserOrganizationRoles::CreatedBy))
            .col(uuid_null(UserOrganizationRoles::UpdatedBy))
            .foreign_key(
                ForeignKey::create()
                    .name("fk-user_organization_roles-users")
                    .from(UserOrganizationRoles::Table, UserOrganizationRoles::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-user_organization_roles-organizations")
                    .from(UserOrganizationRoles::Table, UserOrganizationRoles::OrganizationId)
                    .to(Organizations::Table, Organizations::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-user_organization_roles-roles")
                    .from(UserOrganizationRoles::Table, UserOrganizationRoles::RoleId)
                    .to(Roles::Table, Roles::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await?;

        let idx_organization_id: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-user-organization-roles-organization-id")
            .table(UserOrganizationRoles::Table)
            .col(UserOrganizationRoles::OrganizationId).to_owned();

        let idx_role_id: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-user-organization-roles-role-id")
            .table(UserOrganizationRoles::Table)
            .col(UserOrganizationRoles::RoleId).to_owned();

        manager.create_index(idx_organization_id).await?;
        manager.create_index(idx_role_id).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop()
                .table(UserOrganizationRoles::Table)
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum UserOrganizationRoles {
    Table,
    Id,
    UserId,
    OrganizationId,
    RoleId,
    CreatedBy,
    UpdatedBy,
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
