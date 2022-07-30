use crate::Migrator;
use futures::{future::TryFutureExt, Future};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220701_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.create_country_table_fut(manager)
            .and_then(|_| self.create_delivery_centers_table_fut(manager))
            .and_then(|_| self.create_delivery_center_floors_table_fut(manager))
            .and_then(|_| self.create_work_desks_table_fut(manager))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Migrator::drop_table_if_exists_fut(manager, WorkDesk::Table)
            .and_then(|_| Migrator::drop_table_if_exists_fut(manager, DeliveryCenterFloor::Table))
            .and_then(|_| Migrator::drop_table_if_exists_fut(manager, DeliveryCenter::Table))
            .and_then(|_| Migrator::drop_table_if_exists_fut(manager, Country::Table))
            .await
    }
}

impl Migration {
    fn create_country_table_fut<'a, 'b: 'a>(
        &'b self,
        manager: &'a SchemaManager,
    ) -> impl Future<Output = Result<(), DbErr>> + 'a {
        manager.create_table(
            Table::create()
                .table(Country::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Country::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Country::Name).string().not_null())
                .to_owned(),
        )
    }

    fn create_delivery_centers_table_fut<'a, 'b: 'a>(
        &'b self,
        manager: &'a SchemaManager,
    ) -> impl Future<Output = Result<(), DbErr>> + 'a {
        manager.create_table(
            Table::create()
                .table(DeliveryCenter::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(DeliveryCenter::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(DeliveryCenter::Name).string().not_null())
                .col(
                    ColumnDef::new(DeliveryCenter::CountryId)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(DeliveryCenter::CreatedAt)
                        .timestamp_with_time_zone()
                        .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_delivery_center_country")
                        .from(DeliveryCenter::Table, DeliveryCenter::CountryId)
                        .to(Country::Table, Country::Id)
                        .on_delete(ForeignKeyAction::NoAction)
                        .on_update(ForeignKeyAction::NoAction),
                )
                .to_owned(),
        )
    }

    fn create_delivery_center_floors_table_fut<'a, 'b: 'a>(
        &'b self,
        manager: &'a SchemaManager,
    ) -> impl Future<Output = Result<(), DbErr>> + 'a {
        manager.create_table(
            Table::create()
                .table(DeliveryCenterFloor::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(DeliveryCenterFloor::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(DeliveryCenterFloor::Name)
                        .string()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(DeliveryCenterFloor::DeliveryCenterId)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(DeliveryCenterFloor::CreatedAt)
                        .timestamp_with_time_zone()
                        .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_delivery_center_floor_delivery_center")
                        .from(
                            DeliveryCenterFloor::Table,
                            DeliveryCenterFloor::DeliveryCenterId,
                        )
                        .to(DeliveryCenter::Table, DeliveryCenter::Id)
                        .on_delete(ForeignKeyAction::NoAction)
                        .on_update(ForeignKeyAction::NoAction),
                )
                .to_owned(),
        )
    }

    fn create_work_desks_table_fut<'a, 'b: 'a>(
        &'b self,
        manager: &'a SchemaManager,
    ) -> impl Future<Output = Result<(), DbErr>> + 'a {
        manager.create_table(
            Table::create()
                .table(WorkDesk::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(WorkDesk::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(WorkDesk::FloorId).integer().not_null())
                .col(ColumnDef::new(WorkDesk::Name).string().not_null())
                .col(
                    ColumnDef::new(WorkDesk::IsAvailable)
                        .boolean()
                        .default(Value::Bool(Some(false))),
                )
                .col(ColumnDef::new(WorkDesk::LocationX).integer())
                .col(ColumnDef::new(WorkDesk::LocationY).integer())
                .col(
                    ColumnDef::new(WorkDesk::CreatedAt)
                        .timestamp_with_time_zone()
                        .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_work_desks_delivery_center_floor")
                        .from(WorkDesk::Table, WorkDesk::FloorId)
                        .to(DeliveryCenterFloor::Table, DeliveryCenterFloor::Id)
                        .on_delete(ForeignKeyAction::NoAction)
                        .on_update(ForeignKeyAction::NoAction),
                )
                .to_owned(),
        )
    }
}

#[derive(Iden)]
pub enum Country {
    #[iden = "countries"]
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum DeliveryCenter {
    #[iden = "delivery_centers"]
    Table,
    Id,
    Name,
    CountryId,
    CreatedAt,
}

#[derive(Iden)]
pub enum DeliveryCenterFloor {
    #[iden = "delivery_center_floors"]
    Table,
    Id,
    Name,
    DeliveryCenterId,
    CreatedAt,
}

#[derive(Iden)]
pub enum WorkDesk {
    #[iden = "work_desks"]
    Table,
    Id,
    FloorId,
    Name,
    LocationX,
    LocationY,
    IsAvailable,
    CreatedAt,
}
