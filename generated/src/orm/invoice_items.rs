//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "invoice_items"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub invoice_line_id: i32,
    pub invoice_id: i32,
    pub track_id: i32,
    pub unit_price: Decimal,
    pub quantity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    #[sea_orm(column_name = "InvoiceLineId")]
    InvoiceLineId,
    #[sea_orm(column_name = "InvoiceId")]
    InvoiceId,
    #[sea_orm(column_name = "TrackId")]
    TrackId,
    #[sea_orm(column_name = "UnitPrice")]
    UnitPrice,
    #[sea_orm(column_name = "Quantity")]
    Quantity,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    InvoiceLineId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Tracks,
    Invoices,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::InvoiceLineId => ColumnType::Integer.def(),
            Self::InvoiceId => ColumnType::Integer.def(),
            Self::TrackId => ColumnType::Integer.def(),
            Self::UnitPrice => ColumnType::Decimal(None).def(),
            Self::Quantity => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Tracks => Entity::belongs_to(super::tracks::Entity)
                .from(Column::TrackId)
                .to(super::tracks::Column::TrackId)
                .into(),
            Self::Invoices => Entity::belongs_to(super::invoices::Entity)
                .from(Column::InvoiceId)
                .to(super::invoices::Column::InvoiceId)
                .into(),
        }
    }
}

impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tracks.def()
    }
}

impl Related<super::invoices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invoices.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}