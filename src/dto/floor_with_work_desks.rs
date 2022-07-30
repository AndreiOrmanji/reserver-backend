use super::work_desk_default::WorkDeskDefault;
use entity::{
    delivery_center::Model as DeliveryCenter, delivery_center_floor::Model as DeliveryCenterFloor,
    work_desk::Model as WorkDesk,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FloorWithWorkDesks {
    pub id: i32,
    pub name: String,
    pub delivery_center_id: i32,
    pub delivery_center_name: String,
    pub desks: Vec<WorkDeskDefault>,
}

impl From<&(DeliveryCenterFloor, DeliveryCenter)> for FloorWithWorkDesks {
    fn from(source: &(DeliveryCenterFloor, DeliveryCenter)) -> Self {
        let (floor, center) = source;
        let work_desks: Vec<WorkDeskDefault> = Vec::with_capacity(0);

        Self {
            id: floor.id,
            name: floor.name.clone(),
            delivery_center_id: center.id,
            delivery_center_name: center.name.clone(),
            desks: work_desks,
        }
    }
}

impl From<&(DeliveryCenterFloor, DeliveryCenter, Vec<WorkDesk>)> for FloorWithWorkDesks {
    fn from(source: &(DeliveryCenterFloor, DeliveryCenter, Vec<WorkDesk>)) -> Self {
        let (floor, center, work_desks) = source;

        Self {
            id: floor.id,
            name: floor.name.clone(),
            delivery_center_id: center.id,
            delivery_center_name: center.name.clone(),
            desks: work_desks.iter().map(WorkDeskDefault::from).collect(),
        }
    }
}
