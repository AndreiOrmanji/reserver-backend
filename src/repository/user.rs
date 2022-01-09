use crate::entity::{prelude::*, country, user};

use sea_orm::{entity::*, query::*};
use sea_orm::{DatabaseConnection, DbErr};

pub async fn find_user_with_country_by_id(
    user_id: i32,
    conn: &DatabaseConnection,
) -> Result<Option<(user::Model, Option<country::Model>)>, DbErr> {
    match User::find()
        .filter(user::Column::Id.eq(user_id))
        .one(conn)
        .await
    {
        Err(e) => Err(e),
        Ok(user) => match user {
            None => Ok(None),
            Some(u) => match u.find_related(Country).one(conn).await {
                Err(e) => Err(e),
                Ok(c) => Ok(Some((u, c))),
            },
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use chrono::NaiveDateTime;
    use sea_orm::{MockDatabase, DatabaseBackend, Transaction};
    use std::str::FromStr;

    #[actix_web::test]
    async fn test_find_user_with_country_by_id() {
        let user_id = 1i32;

        let c = country::Model {
            id: 1i32,
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
            .append_query_results(vec![vec![u.clone()]])
            .append_query_results(vec![vec![c.clone()]])
            .into_connection();

        assert_eq!(Ok(Some((u, Some(c)))), super::find_user_with_country_by_id(user_id, &db_conn).await);

        assert_eq!(
            db_conn.into_transaction_log(),
            vec![
                Transaction::from_sql_and_values(
                    DatabaseBackend::MySql,
                    r#"SELECT `users2`.`id`, `users2`.`email`, `users2`.`first_name`, `users2`.`last_name`, `users2`.`age`, `users2`.`country_id`, `users2`.`created_at` FROM `users2` WHERE `users2`.`id` = ? LIMIT ?"#,
                    vec![1i32.into(), Value::BigUnsigned(Some(1u64))]
                ),
                Transaction::from_sql_and_values(
                    DatabaseBackend::MySql,
                    r#"SELECT `countries2`.`id`, `countries2`.`name` FROM `countries2` INNER JOIN `users2` ON `users2`.`country_id` = `countries2`.`id` WHERE `users2`.`id` = ? LIMIT ?"#,
                    vec![1i32.into(), Value::BigUnsigned(Some(1u64))]
                ),
            ]
        );
    }
}