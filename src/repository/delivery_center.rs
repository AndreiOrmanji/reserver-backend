use entity::{delivery_center::Model as DeliveryCenterM, prelude::*};
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<DeliveryCenterM>, DbErr> {
    DeliveryCenter::find().all(conn).await
}

#[cfg(test)]
mod test {
    use super::*;

    use sea_orm::{DatabaseBackend, MockDatabase, Transaction};

    #[actix_web::test]
    async fn test_find_all() {
        let expected_result = Vec::<DeliveryCenterM>::with_capacity(0);
        let db_backend = DatabaseBackend::MySql;
        let db_conn = MockDatabase::new(db_backend.clone())
            .append_query_results(vec![expected_result])
            .into_connection();

        assert_eq!(
            Ok(vec![]),
            super::find_all(&db_conn).await
        );

        assert_eq!(
            db_conn.into_transaction_log(),
            vec![Transaction::from_sql_and_values(
                db_backend.clone(),
                r#"SELECT `users`.`id` AS `A_id`, `users`.`email` AS `A_email`, `users`.`first_name` AS `A_first_name`, `users`.`last_name` AS `A_last_name`, `users`.`country_id` AS `A_country_id`, `users`.`created_at` AS `A_created_at`, `countries`.`id` AS `B_id`, `countries`.`name` AS `B_name` FROM `users` LEFT JOIN `countries` ON `users`.`country_id` = `countries`.`id` WHERE `users`.`id` = ? LIMIT ?"#,
                vec![]
            ),]
        );
        unimplemented!()
    }
}
