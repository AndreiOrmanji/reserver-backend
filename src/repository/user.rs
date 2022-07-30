use entity::{country, prelude::*, user};
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};

pub async fn find_user_with_country_by_id(
    conn: &DatabaseConnection,
    user_id: i32,
) -> Result<Option<(user::Model, Option<country::Model>)>, DbErr> {
    User::find()
        .find_also_related(Country)
        .filter(user::Column::Id.eq(user_id))
        .one(conn)
        .await
}

#[cfg(test)]
mod test {
    use super::*;

    use sea_orm::{prelude::TimeDateTimeWithTimeZone, DatabaseBackend, MockDatabase, Transaction};
    use time::format_description::well_known::Rfc3339;
    // use time_tz::{DateTime, NaiveDateTime, Utc};

    #[actix_web::test]
    async fn test_find_user_with_country_by_id() {
        let user_id = 1i32;

        let c = country::Model {
            id: user_id,
            name: "Moldova".into(),
        };

        let u = user::Model {
            id: user_id,
            email: Some("a@n.com".into()),
            first_name: Some("mF_9.B5f-4.1JqM".into()),
            last_name: Some("TestNameLast".into()),
            country_id: Some(c.id),
            created_at: Some(
                TimeDateTimeWithTimeZone::parse("2022-01-01T16:46:28Z", &Rfc3339).unwrap(),
            ),
        };

        let db_backend = DatabaseBackend::MySql;
        let db_conn = MockDatabase::new(db_backend)
            .append_query_results(vec![vec![(u.clone(), c.clone())]])
            .into_connection();

        assert_eq!(
            Ok(Some((u.clone(), Some(c.clone())))),
            super::find_user_with_country_by_id(&db_conn, user_id).await
        );

        assert_eq!(
            db_conn.into_transaction_log(),
            vec![Transaction::from_sql_and_values(
                DatabaseBackend::MySql,
                r#"SELECT `users`.`id` AS `A_id`, `users`.`email` AS `A_email`, `users`.`first_name` AS `A_first_name`, `users`.`last_name` AS `A_last_name`, `users`.`country_id` AS `A_country_id`, `users`.`created_at` AS `A_created_at`, `countries`.`id` AS `B_id`, `countries`.`name` AS `B_name` FROM `users` LEFT JOIN `countries` ON `users`.`country_id` = `countries`.`id` WHERE `users`.`id` = ? LIMIT ?"#,
                vec![Value::Int(Some(user_id)), Value::BigUnsigned(Some(1u64))]
            ),]
        );

        let db_backend = DatabaseBackend::Postgres;
        let db_conn = MockDatabase::new(db_backend)
            .append_query_results(vec![vec![(u.clone(), c.clone())]])
            .into_connection();

        assert_eq!(
            Ok(Some((u, Some(c)))),
            super::find_user_with_country_by_id(&db_conn, user_id).await
        );

        assert_eq!(
            db_conn.into_transaction_log(),
            vec![Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "users"."id" AS "A_id", "users"."email" AS "A_email", "users"."first_name" AS "A_first_name", "users"."last_name" AS "A_last_name", "users"."country_id" AS "A_country_id", "users"."created_at" AS "A_created_at", "countries"."id" AS "B_id", "countries"."name" AS "B_name" FROM "users" LEFT JOIN "countries" ON "users"."country_id" = "countries"."id" WHERE "users"."id" = $1 LIMIT $2"#,
                vec![Value::Int(Some(user_id)), Value::BigUnsigned(Some(1u64))]
            ),]
        );
    }
}
