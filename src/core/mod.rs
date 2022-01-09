use sea_orm::DatabaseConnection;

pub mod authorized_user;

pub use authorized_user::AuthorizedUser;
pub use authorized_user::BadAuthHeaderError;

#[derive(Debug)]
pub struct AppState {
    conn: DatabaseConnection,
}

impl AppState {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub fn get_db_conn(&self) -> &DatabaseConnection {
        &self.conn
    }
}
