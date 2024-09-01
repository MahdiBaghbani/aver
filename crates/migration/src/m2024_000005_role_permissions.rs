use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table: TableCreateStatement = table_auto(RolePermissions::Table)
            .col(uuid(RolePermissions::Id).unique_key().primary_key())
            .col(uuid(RolePermissions::RoleId))
            .col(uuid(RolePermissions::PermissionId))
            .col(uuid_null(RolePermissions::CreatedBy))
            .col(uuid_null(RolePermissions::UpdatedBy))
            .foreign_key(
                ForeignKey::create()
                    .name("fk-role_permissions-roles")
                    .from(RolePermissions::Table, RolePermissions::RoleId)
                    .to(Roles::Table, Roles::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-role_permissions-permissions")
                    .from(RolePermissions::Table, RolePermissions::PermissionId)
                    .to(Permissions::Table, Permissions::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await?;


        let idx_role_id: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-role-permissions-role-id")
            .table(RolePermissions::Table)
            .col(RolePermissions::RoleId).to_owned();

        let idx_permission_id: IndexCreateStatement = Index::create()
            .if_not_exists()
            .name("idx-role-permissions-permission-id")
            .table(RolePermissions::Table)
            .col(RolePermissions::PermissionId).to_owned();

        manager.create_index(idx_role_id).await?;
        manager.create_index(idx_permission_id).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(RolePermissions::Table)
                    .to_owned()
            ).await
    }
}

#[derive(DeriveIden)]
enum RolePermissions {
    Table,
    Id,
    RoleId,
    PermissionId,
    CreatedBy,
    UpdatedBy,
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Permissions {
    Table,
    Id,
}
