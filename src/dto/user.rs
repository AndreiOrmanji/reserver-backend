use crate::entity::{country::Model as Country, user::Model as User};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserWithCountry {
    pub id: i32,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub age: Option<i8>,
    pub country: Option<Country>,
    pub created_at: Option<NaiveDateTime>,
}

impl From<&(User, Option<Country>)> for UserWithCountry {
    fn from(user_with_country: &(User, Option<Country>)) -> UserWithCountry {
        let (user, country) = user_with_country;
        Self {
            country: country.clone(),
            id: user.id,
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            age: user.age,
            created_at: user.created_at,
        }
    }
}
