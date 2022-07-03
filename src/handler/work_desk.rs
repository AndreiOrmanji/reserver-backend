use crate::dto::{floor_with_work_desks::FloorWithWorkDesks};
use crate::repository;
use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use log::error;

#[get("/delivery-center/{center_id}/floor/{floor_id}/work_desks")]
pub async fn get_floor_by_id_of_center_by_id(
    _req: HttpRequest,
    data: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> impl Responder {
    let (center_id, floor_id) = params.into_inner();
    let conn = (&data).get_db_conn();

    match repository::floor::get_floor_by_id_of_center_by_id(conn, center_id, floor_id).await {
        Err(e) => internal_server_error_with_log(e),

        Ok(option_floor_with_center) => match option_floor_with_center {
            None => HttpResponse::NotFound().finish(),

            Some((floor, Some(center))) => {
                match repository::work_desk::get_work_desks_by_floor_id(conn, floor.id).await {
                    Err(e) => internal_server_error_with_log(e),

                    Ok(work_desks) => HttpResponse::Ok()
                        .json(FloorWithWorkDesks::from(&(floor, center, work_desks))),
                }
            }
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

fn internal_server_error_with_log(e: impl std::fmt::Debug) -> HttpResponse {
    error!("{:?}", e);
    HttpResponse::InternalServerError().finish()
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use entity::{country, user};
    use actix_web::{test, web::Bytes, App};
    use chrono::{DateTime, NaiveDateTime, Utc};
    use sea_orm::{DatabaseBackend, MockDatabase};
    use std::str::FromStr;

    #[actix_web::test]
    async fn test_handler_get_user_by_id() {
        let c = country::Model {
            id: 1,
            name: "Moldova".into(),
        };
        let u = user::Model {
            id: 1,
            email: Some("a@n.com".into()),
            first_name: Some("mF_9.B5f-4.1JqM".into()),
            last_name: Some("TestNameLast".into()),
            country_id: Some(c.id),
            created_at: Some(DateTime::from_utc(NaiveDateTime::from_str("2022-01-01T16:46:28").unwrap(), Utc)),
        };

        // Create MockDatabase with mock query results
        let db_conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![(u.clone(), c.clone())]])
            .into_connection();

        let d = web::Data::new(AppState::new(db_conn));

        let app = test::init_service(App::new().app_data(d.clone()).service(get_user_by_id)).await;

        let resp = test::TestRequest::get()
            .uri("/user/2")
            .send_request(&app)
            .await;

        let result = test::read_body(resp).await;

        assert_eq!(result, Bytes::from_static(b"{\"id\":1,\"email\":\"a@n.com\",\"first_name\":\"mF_9.B5f-4.1JqM\",\"last_name\":\"TestNameLast\",\"country\":{\"id\":1,\"name\":\"Moldova\"},\"created_at\":\"2022-01-01T16:46:28Z\"}"));
    }
}
 */
