use sea_orm::{entity::prelude::*, prelude::DateTimeWithTimeZone};
use entity::{country::Model as Country, user::Model as User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWithCountry {
    pub id: i32,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<Country>,
    pub created_at: Option<DateTimeWithTimeZone>,
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
            created_at: user.created_at,
        }
    }
}
