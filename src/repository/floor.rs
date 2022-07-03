use entity::{delivery_center, delivery_center_floor, prelude::*};
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};

pub async fn get_floor_by_id_of_center_by_id(
    conn: &DatabaseConnection,
    delivery_center_id: i32,
    floor_id: i32,
) -> Result<Option<(delivery_center_floor::Model, Option<delivery_center::Model>)>, DbErr> {
    DeliveryCenterFloor::find()
        .find_also_related(DeliveryCenter)
        .filter(delivery_center::Column::Id.eq(delivery_center_id))
        .filter(delivery_center_floor::Column::Id.eq(floor_id))
        .one(conn)
        .await
}
