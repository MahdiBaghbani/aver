//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "user_org_roles"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    CreatedAt,
    UpdatedAt,
    Id,
    UserId,
    OrganizationId,
    RoleId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Organizations,
    Roles,
    Users,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::CreatedAt => ColumnType::DateTime.def(),
            Self::UpdatedAt => ColumnType::DateTime.def(),
            Self::Id => ColumnType::Uuid.def(),
            Self::UserId => ColumnType::Uuid.def(),
            Self::OrganizationId => ColumnType::Uuid.def(),
            Self::RoleId => ColumnType::Uuid.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Organizations => Entity::belongs_to(super::organizations::Entity)
                .from(Column::OrganizationId)
                .to(super::organizations::Column::Id)
                .into(),
            Self::Roles => Entity::belongs_to(super::roles::Entity)
                .from(Column::RoleId)
                .to(super::roles::Column::Id)
                .into(),
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::Id)
                .into(),
        }
    }
}

impl Related<super::organizations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organizations.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Roles.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
