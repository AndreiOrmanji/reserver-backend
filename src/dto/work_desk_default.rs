use entity::work_desk::Model as WorkDesk;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDeskDefault {
    pub id: i32,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub is_bookable: bool,
    pub created_at: Option<TimeDateTimeWithTimeZone>,
}

impl From<&WorkDesk> for WorkDeskDefault {
    fn from(work_desk: &WorkDesk) -> WorkDeskDefault {
        Self {
            id: work_desk.id,
            x: work_desk.location_x,
            y: work_desk.location_y,
            is_bookable: work_desk.is_available,
            created_at: work_desk.created_at,
        }
    }
}
