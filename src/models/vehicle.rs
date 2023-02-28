use diesel::prelude::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::schema::vehicles;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub vehicle_type: String,
    pub engine: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = vehicles)]
pub struct InsertVehicle {
    pub brand: String,
    pub model: String,
    pub vehicle_type: String,
    pub engine: String,
}
