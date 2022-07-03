use entity::{prelude::*, work_desk};
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};

pub async fn get_work_desks_by_floor_id(
    conn: &DatabaseConnection,
    floor_id: i32,
) -> Result<Vec<work_desk::Model>, DbErr> {
    WorkDesk::find()
        .filter(work_desk::Column::FloorId.eq(floor_id))
        .all(conn)
        .await
}
