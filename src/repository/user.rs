use crate::entity::{country, prelude::*, user};

use sea_orm::{entity::*, query::*};
use sea_orm::{DatabaseConnection, DbErr};

pub async fn find_user_with_country_by_id(
    user_id: i32,
    conn: &DatabaseConnection,
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

    use chrono::NaiveDateTime;
    use sea_orm::{DatabaseBackend, MockDatabase, Transaction};
    use std::str::FromStr;

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
            age: Some(22),
            country_id: Some(c.id),
            created_at: Some(NaiveDateTime::from_str("2022-01-01T16:46:28").unwrap()),
        };

        let db_conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![(u.clone(), c.clone())]])
            .into_connection();

        assert_eq!(
            Ok(Some((u, Some(c)))),
            super::find_user_with_country_by_id(user_id, &db_conn).await
        );

        assert_eq!(
            db_conn.into_transaction_log(),
            vec![Transaction::from_sql_and_values(
                DatabaseBackend::MySql,
                r#"SELECT `users`.`id` AS `A_id`, `users`.`email` AS `A_email`, `users`.`first_name` AS `A_first_name`, `users`.`last_name` AS `A_last_name`, `users`.`age` AS `A_age`, `users`.`country_id` AS `A_country_id`, `users`.`created_at` AS `A_created_at`, `countries`.`id` AS `B_id`, `countries`.`name` AS `B_name` FROM `users` LEFT JOIN `countries` ON `users`.`country_id` = `countries`.`id` WHERE `users`.`id` = ? LIMIT ?"#,
                vec![Value::Int(Some(user_id)), Value::BigUnsigned(Some(1u64))]
            ),]
        );
    }
}
