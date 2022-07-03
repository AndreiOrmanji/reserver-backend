use crate::m20220701_000001_create_table::*;
use futures::{future::TryFutureExt, Future};
use sea_orm::strum::EnumProperty;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{JsonValue, QueryFilter},
};
use serde::Serialize;
use serde_json::json;

const COUNTRIES: [(i32, &'static str); 3] = [(1, "Moldova"), (2, "UK"), (3, "Ukraine")];

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220701_000002_fill_tables"
    }
}

#[derive(EnumProperty)]
enum CountryEnum {
    #[strum(props(id = "1", name = "Forum Chisinau"))]
    Moldova,

    #[strum(props(id = "2", name = "BC Bristol"))]
    UnitedKingdom,

    #[strum(props(id = "3", name = "UBC Romania"))]
    Romania,

    #[strum(props(id = "4", name = "Burj Office"))]
    Dubai,

    #[strum(props(id = "3", name = "Kokon Ukraine"))]
    Ukraine,

    #[strum(props(id = "18", name = "Sofia Bulgaria"))]
    Bulgaria,
}

impl Serialize for CountryEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(
            self.get_str("id").unwrap()
                .parse::<i32>().unwrap()
        )
    }
}

impl Serialize for FloorEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(
            self.get_str("id").unwrap()
                .parse::<i32>().unwrap()
        )
    }
}

#[derive(EnumProperty)]
enum FloorEnum {
    #[strum(props(id = "1"))]
    MoldavaFloor6,

    #[strum(props(id = "2"))]
    MoldavaFloor8,

    #[strum(props(id = "3"))]
    MoldavaFloor9,

    #[strum(props(id = "5"))]
    MoldavaFloor10,

    #[strum(props(id = "6"))]
    MoldavaFloor11,

    #[strum(props(id = "7"))]
    MoldavaFloor12,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut query = Query::insert()
            .into_table(Country::Table)
            .columns([Country::Id, Country::Name])
            .to_owned();

        COUNTRIES.iter().for_each(|&c| {
            query.values_panic(vec![c.0.into(), c.1.into()]);
        });

        println!(
            "{:?}",
            manager.get_database_backend().build(&query).to_string()
        );
        manager.exec_stmt(query).await?;

        let test: JsonValue = json!(
            [
                {
                    "id": FloorEnum::MoldavaFloor6,
                    "name": "Floor 6",
                    "delivery_center_id": CountryEnum::Moldova
                },
                {
                    "id": FloorEnum::MoldavaFloor8,
                    "name": "Floor 8",
                    "delivery_center_id": CountryEnum::Moldova
                },
                {
                    "id": FloorEnum::MoldavaFloor9,
                    "name": "Floor 9",
                    "delivery_center_id": CountryEnum::Moldova
                },
                {
                    "id": FloorEnum::MoldavaFloor10,
                    "name": "Floor 10",
                    "delivery_center_id": CountryEnum::Moldova
                },
                {
                    "id": FloorEnum::MoldavaFloor11,
                    "name": "Floor 11",
                    "delivery_center_id": CountryEnum::Moldova
                },
                {
                    "id": FloorEnum::MoldavaFloor12,
                    "name": "Floor 12",
                    "delivery_center_id": CountryEnum::Moldova
                }
            ]
        );

        println!("{:?}", test.to_string());
        Ok(())
    }

    // if you are against backward migrations, you do not have to impl this
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // manager
        //     .truncate_table(Table::truncate().table(Country::Table).to_owned())
        //     .await
        Ok(())
    }
}

impl Migration {
    /*
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

    fn drop_table_if_exists_fut<'a, 'b: 'a, T: 'static>(
        &'b self,
        manager: &'a SchemaManager,
        table: T,
    ) -> impl Future<Output = Result<(), DbErr>> + 'a
    where
        T: 'static + IntoTableRef,
    {
        manager.drop_table(Table::drop().table(table).if_exists().to_owned())
    }
     */
}
