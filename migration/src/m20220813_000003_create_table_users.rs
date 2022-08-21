use crate::{Migrator, m20220701_000001_create_table::Country};
use futures::Future;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220813_000003_create_table_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.create_users_table_fut(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Migrator::drop_table_if_exists_fut(manager, User::Table).await
    }
}

impl Migration {
    fn create_users_table_fut<'a, 'b: 'a>(
        &'b self,
        manager: &'a SchemaManager,
    ) -> impl Future<Output = Result<(), DbErr>> + 'a {
        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(User::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(User::Email).string())
                .col(ColumnDef::new(User::FirstName).string())
                .col(ColumnDef::new(User::LastName).string())
                .col(ColumnDef::new(User::CountryId).integer())
                .col(
                    ColumnDef::new(User::CreatedAt)
                        .timestamp_with_time_zone()
                        .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_user_country")
                        .from(User::Table, User::CountryId)
                        .to(Country::Table, Country::Id)
                        .on_delete(ForeignKeyAction::NoAction)
                        .on_update(ForeignKeyAction::NoAction),
                )
                .to_owned(),
        )
    }
}

#[derive(Iden)]
pub enum User {
    #[iden = "users"]
    Table,
    Id,
    Email,
    FirstName,
    LastName,
    CountryId,
    CreatedAt,
}
