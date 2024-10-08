//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "ocm_users"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub id: Uuid,
    pub remote_id: String,
    pub name: String,
    pub email: String,
    pub provider: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    CreatedAt,
    UpdatedAt,
    Id,
    RemoteId,
    Name,
    Email,
    Provider,
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
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::CreatedAt => ColumnType::DateTime.def(),
            Self::UpdatedAt => ColumnType::DateTime.def(),
            Self::Id => ColumnType::Uuid.def(),
            Self::RemoteId => ColumnType::String(StringLen::None).def(),
            Self::Name => ColumnType::String(StringLen::None).def(),
            Self::Email => ColumnType::String(StringLen::None).def(),
            Self::Provider => ColumnType::String(StringLen::None).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::OcmContacts => Entity::has_many(super::ocm_contacts::Entity).into(),
        }
    }
}

impl Related<super::ocm_contacts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OcmContacts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
