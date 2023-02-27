use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub vehicle_type: String,
    pub engine: String,
}
