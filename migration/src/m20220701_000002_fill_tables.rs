use crate::m20220701_000001_create_table::*;
use crate::Migrator;
use futures::{future::TryFutureExt, Future};
use sea_orm::strum::EnumProperty;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{prelude::TimeDateTimeWithTimeZone, JsonValue},
};
use serde::Serialize;
use serde_json::json;
use time_tz::{timezones, OffsetDateTimeExt};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220701_000002_fill_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.insert_countries_fut(manager)
            .and_then(|_| self.insert_delivery_centers_fut(manager))
            .and_then(|_| self.insert_delivery_center_floors_fut(manager))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Migrator::drop_table_if_exists_fut(manager, DeliveryCenterFloor::Table)
            .and_then(|_| Migrator::drop_table_if_exists_fut(manager, DeliveryCenter::Table))
            .and_then(|_| Migrator::drop_table_if_exists_fut(manager, Country::Table))
            .await
    }
}

impl Migration {
    fn insert_countries_fut<'a, 'b: 'a>(
        &'b self,
        manager: &'a SchemaManager,
    ) -> impl Future<Output = Result<(), DbErr>> + 'a {
        let delivery_centers: JsonValue = json!(
            [
                {
                    "id": CountryEnum::Moldova,
                    "name": CountryEnum::Moldova.get_str("name").unwrap()
                },
                {
                    "id": CountryEnum::UnitedKingdom,
                    "name": CountryEnum::UnitedKingdom.get_str("name").unwrap()
                },
                {
                    "id": CountryEnum::Romania,
                    "name": CountryEnum::Romania.get_str("name").unwrap()
                },
                {
                    "id": CountryEnum::Dubai,
                    "name": CountryEnum::Dubai.get_str("name").unwrap()
                },
                {
                    "id": CountryEnum::Ukraine,
                    "name": CountryEnum::Ukraine.get_str("name").unwrap()
                },
                {
                    "id": CountryEnum::Bulgaria,
                    "name": CountryEnum::Bulgaria.get_str("name").unwrap()
                },
            ]
        );

        let mut query = Query::insert()
            .into_table(Country::Table)
            .columns([Country::Id, Country::Name])
            .to_owned();

        delivery_centers.as_array().unwrap().iter().for_each(|c| {
            query.values_panic(vec![
                c["id"].as_u64().unwrap().into(),
                c["name"].as_str().into(),
            ]);
        });

        manager.exec_stmt(query)
    }

    fn insert_delivery_centers_fut<'a, 'b: 'a>(
        &'b self,
        manager: &'a SchemaManager,
    ) -> impl Future<Output = Result<(), DbErr>> + 'a {
        let delivery_centers: JsonValue = json!(
            [
                {
                    "id": DeliveryCenterEnum::ForumChisinau,
                    "name": DeliveryCenterEnum::ForumChisinau.get_str("name").unwrap(),
                    "country_id": CountryEnum::Moldova
                },
                {
                    "id": DeliveryCenterEnum::BcBristol,
                    "name": DeliveryCenterEnum::BcBristol.get_str("name").unwrap(),
                    "country_id": CountryEnum::UnitedKingdom
                },
                {
                    "id": DeliveryCenterEnum::UbcRomania,
                    "name": DeliveryCenterEnum::UbcRomania.get_str("name").unwrap(),
                    "country_id": CountryEnum::Romania
                },
                {
                    "id": DeliveryCenterEnum::BurjOffice,
                    "name": DeliveryCenterEnum::BurjOffice.get_str("name").unwrap(),
                    "country_id": CountryEnum::Dubai
                },
                {
                    "id": DeliveryCenterEnum::KokonUkraine,
                    "name": DeliveryCenterEnum::KokonUkraine.get_str("name").unwrap(),
                    "country_id": CountryEnum::Ukraine
                },
                {
                    "id": DeliveryCenterEnum::SofiaBulgaria,
                    "name": DeliveryCenterEnum::SofiaBulgaria.get_str("name").unwrap(),
                    "country_id": CountryEnum::Bulgaria
                },
            ]
        );

        let mut query = Query::insert()
            .into_table(DeliveryCenter::Table)
            .columns([
                DeliveryCenter::Id,
                DeliveryCenter::Name,
                DeliveryCenter::CountryId,
                DeliveryCenter::CreatedAt,
            ])
            .to_owned();

        delivery_centers.as_array().unwrap().iter().for_each(|c| {
            query.values_panic(vec![
                c["id"].as_u64().unwrap().into(),
                c["name"].as_str().into(),
                c["country_id"].as_u64().into(),
                TimeDateTimeWithTimeZone::now_utc()
                    .to_timezone(timezones::db::europe::LONDON)
                    .into(),
            ]);
        });

        println!(
            "{:?}",
            manager.get_database_backend().build(&query).to_string()
        );
        manager.exec_stmt(query)
    }

    fn insert_delivery_center_floors_fut<'a, 'b: 'a>(
        &'b self,
        manager: &'a SchemaManager,
    ) -> impl Future<Output = Result<(), DbErr>> + 'a {
        let delivery_center_floors: JsonValue = json!(
            [
                {
                    "id": FloorEnum::MoldavaFloor6,
                    "name": FloorEnum::MoldavaFloor6.get_str("name").unwrap(),
                    "delivery_center_id": DeliveryCenterEnum::ForumChisinau
                },
                {
                    "id": FloorEnum::MoldavaFloor8,
                    "name": FloorEnum::MoldavaFloor8.get_str("name").unwrap(),
                    "delivery_center_id": DeliveryCenterEnum::ForumChisinau
                },
                {
                    "id": FloorEnum::MoldavaFloor9,
                    "name": FloorEnum::MoldavaFloor9.get_str("name").unwrap(),
                    "delivery_center_id": DeliveryCenterEnum::ForumChisinau
                },
                {
                    "id": FloorEnum::MoldavaFloor10,
                    "name": FloorEnum::MoldavaFloor10.get_str("name").unwrap(),
                    "delivery_center_id": DeliveryCenterEnum::ForumChisinau
                },
                {
                    "id": FloorEnum::MoldavaFloor11,
                    "name": FloorEnum::MoldavaFloor11.get_str("name").unwrap(),
                    "delivery_center_id": DeliveryCenterEnum::ForumChisinau
                },
                {
                    "id": FloorEnum::MoldavaFloor12,
                    "name": FloorEnum::MoldavaFloor12.get_str("name").unwrap(),
                    "delivery_center_id": CountryEnum::Moldova
                }
            ]
        );

        let mut query = Query::insert()
            .into_table(DeliveryCenterFloor::Table)
            .columns([
                DeliveryCenterFloor::Id,
                DeliveryCenterFloor::Name,
                DeliveryCenterFloor::DeliveryCenterId,
                DeliveryCenterFloor::CreatedAt,
            ])
            .to_owned();

        delivery_center_floors
            .as_array()
            .unwrap()
            .iter()
            .for_each(|c| {
                query.values_panic(vec![
                    c["id"].as_u64().unwrap().into(),
                    c["name"].as_str().into(),
                    c["delivery_center_id"].as_u64().into(),
                    TimeDateTimeWithTimeZone::now_utc()
                        .to_timezone(timezones::db::europe::LONDON)
                        .into(),
                ]);
            });

        manager.exec_stmt(query)
    }
}

#[derive(EnumProperty)]
enum CountryEnum {
    #[strum(props(id = "1", name = "Chisinau"))]
    Moldova,

    #[strum(props(id = "2", name = "Bristol"))]
    UnitedKingdom,

    #[strum(props(id = "3", name = "Romania"))]
    Romania,

    #[strum(props(id = "4", name = "Burj"))]
    Dubai,

    #[strum(props(id = "5", name = "Ukraine"))]
    Ukraine,

    #[strum(props(id = "18", name = "Bulgaria"))]
    Bulgaria,
}

impl Serialize for CountryEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.get_str("id").unwrap().parse::<u64>().unwrap())
    }
}

#[derive(EnumProperty)]
enum DeliveryCenterEnum {
    #[strum(props(id = "1", name = "Forum Chisinau"))]
    ForumChisinau,

    #[strum(props(id = "2", name = "BC Bristol"))]
    BcBristol,

    #[strum(props(id = "3", name = "UBC Romania"))]
    UbcRomania,

    #[strum(props(id = "4", name = "Burj Office"))]
    BurjOffice,

    #[strum(props(id = "5", name = "Kokon Ukraine"))]
    KokonUkraine,

    #[strum(props(id = "18", name = "Sofia Bulgaria"))]
    SofiaBulgaria,
}

impl Serialize for DeliveryCenterEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.get_str("id").unwrap().parse::<u64>().unwrap())
    }
}

#[derive(EnumProperty)]
enum FloorEnum {
    #[strum(props(id = "1", name = "Moldava Floor 6"))]
    MoldavaFloor6,

    #[strum(props(id = "2", name = "Moldava Floor 8"))]
    MoldavaFloor8,

    #[strum(props(id = "3", name = "Moldava Floor 9"))]
    MoldavaFloor9,

    #[strum(props(id = "5", name = "Moldava Floor 10"))]
    MoldavaFloor10,

    #[strum(props(id = "6", name = "Moldava Floor 11"))]
    MoldavaFloor11,

    #[strum(props(id = "7", name = "Moldava Floor 12"))]
    MoldavaFloor12,
}

impl Serialize for FloorEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.get_str("id").unwrap().parse::<u64>().unwrap())
    }
}
