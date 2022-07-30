use entity::{delivery_center::Model as DeliveryCenterM, prelude::*};
use sea_orm::{entity::*, DatabaseConnection, DbErr};

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
        let expected_result_vec = vec![expected_result.clone()];
        let db_backend = DatabaseBackend::MySql;
        let db_conn = MockDatabase::new(db_backend.clone())
            .append_query_results(expected_result_vec.clone())
            .into_connection();

        assert_eq!(Ok(expected_result.clone()), super::find_all(&db_conn).await);

        assert_eq!(
            db_conn.into_transaction_log(),
            vec![Transaction::from_sql_and_values(
                db_backend.clone(),
                r#"SELECT `delivery_centers`.`id`, `delivery_centers`.`name`, `delivery_centers`.`country_id`, `delivery_centers`.`created_at` FROM `delivery_centers`"#,
                vec![]
            ),]
        );

        let db_backend = DatabaseBackend::Postgres;
        let db_conn = MockDatabase::new(db_backend.clone())
            .append_query_results(expected_result_vec.clone())
            .into_connection();

        assert_eq!(Ok(expected_result.clone()), super::find_all(&db_conn).await);

        assert_eq!(
            db_conn.into_transaction_log(),
            vec![Transaction::from_sql_and_values(
                db_backend.clone(),
                r#"SELECT "delivery_centers"."id", "delivery_centers"."name", "delivery_centers"."country_id", "delivery_centers"."created_at" FROM "delivery_centers""#,
                vec![]
            ),]
        );
    }
}
