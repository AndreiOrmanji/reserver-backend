use futures::Future;
pub use sea_orm_migration::prelude::*;

mod m20220701_000001_create_table;
mod m20220701_000002_fill_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220701_000001_create_table::Migration),
            Box::new(m20220701_000002_fill_tables::Migration),
        ]
    }
}

impl Migrator {
    fn drop_table_if_exists_fut<'a, 'b: 'a, T: 'static>(
        manager: &'a SchemaManager,
        table: T,
    ) -> impl Future<Output = Result<(), DbErr>> + 'a
    where
        T: 'static + IntoTableRef,
    {
        manager.drop_table(Table::drop().table(table).if_exists().to_owned())
    }
}
