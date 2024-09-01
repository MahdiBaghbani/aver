#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]

pub use sea_orm_migration::prelude::*;

mod m2024_000001_users;
mod m2024_000002_roles;
mod m2024_000003_permissions;
mod m2024_000004_organizations;
mod m2024_000005_role_permissions;
mod m2024_000006_user_org_roles;
mod m2024_000007_ocm_users;
mod m2024_000008_ocm_contacts;
mod m2024_000009_ocm_invite_tokens;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m2024_000001_users::Migration),
            Box::new(m2024_000002_roles::Migration),
            Box::new(m2024_000003_permissions::Migration),
            Box::new(m2024_000004_organizations::Migration),
            Box::new(m2024_000005_role_permissions::Migration),
            Box::new(m2024_000006_user_org_roles::Migration),
            Box::new(m2024_000007_ocm_users::Migration),
            Box::new(m2024_000008_ocm_contacts::Migration),
            Box::new(m2024_000009_ocm_invite_tokens::Migration),
        ]
    }
}
