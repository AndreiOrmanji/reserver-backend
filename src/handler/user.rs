use crate::dto::user::UserWithCountry;
use crate::repository;
use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use log::{error, info};

#[get("/user/{user_id}")]
pub async fn get_user_by_id(
    _req: HttpRequest,
    data: web::Data<AppState>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let conn = (&data).get_db_conn();

    match repository::user::find_user_with_country_by_id(conn, user_id.into_inner()).await {
        Err(e) => internal_server_error_with_log(e),

        Ok(option_user_with_country) => match option_user_with_country {
            None => HttpResponse::NotFound().finish(),

            Some(user_with_country) => {
                let dto = UserWithCountry::from(&user_with_country);
                info!("{:?}", &dto);

                HttpResponse::Ok().json(dto)
            }
        },
    }
}

fn internal_server_error_with_log(e: impl std::fmt::Debug) -> HttpResponse {
    error!("{:?}", e);
    HttpResponse::InternalServerError().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web::Bytes, App};
    use entity::{country, user};
    use sea_orm::{entity::prelude::TimeDateTimeWithTimeZone, DatabaseBackend, MockDatabase};
    use time::format_description::well_known::Rfc3339;

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
            created_at: Some(
                TimeDateTimeWithTimeZone::parse("2022-01-01T16:46:28Z", &Rfc3339).unwrap(),
            ),
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

        assert_eq!(result, Bytes::from_static(b"{\"id\":1,\"email\":\"a@n.com\",\"firstName\":\"mF_9.B5f-4.1JqM\",\"lastName\":\"TestNameLast\",\"country\":{\"id\":1,\"name\":\"Moldova\"},\"createdAt\":\"2022-01-01T16:46:28Z\"}"));
    }
}
