//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "users"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    CreatedAt,
    UpdatedAt,
    Id,
    FirstName,
    LastName,
    Username,
    Password,
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
    OcmContacts,
    OcmInviteTokens,
    UserOrgRoles,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::CreatedAt => ColumnType::DateTime.def(),
            Self::UpdatedAt => ColumnType::DateTime.def(),
            Self::Id => ColumnType::Uuid.def(),
            Self::FirstName => ColumnType::String(StringLen::None).def(),
            Self::LastName => ColumnType::String(StringLen::None).def(),
            Self::Username => ColumnType::String(StringLen::None).def().unique(),
            Self::Password => ColumnType::String(StringLen::None).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::OcmContacts => Entity::has_many(super::ocm_contacts::Entity).into(),
            Self::OcmInviteTokens => Entity::has_many(super::ocm_invite_tokens::Entity).into(),
            Self::UserOrgRoles => Entity::has_many(super::user_org_roles::Entity).into(),
        }
    }
}

impl Related<super::ocm_contacts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OcmContacts.def()
    }
}

impl Related<super::ocm_invite_tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OcmInviteTokens.def()
    }
}

impl Related<super::user_org_roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserOrgRoles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
