use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Route;

use crate::models::response::Response;
use crate::models::vehicle::*;
use crate::services::vehicle_service::*;

#[get("/all")]
pub async fn get_all_vehicles() -> status::Custom<Json<Response>> {
    let response = select_all();

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[post("/new-vehicle", format = "json", data = "<insert_request>")]
pub async fn create_new_vehicle(
    insert_request: Json<InsertVehicle>,
) -> status::Custom<Json<Response>> {
    let response = create_new(insert_request.0);

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

pub fn vehicle_routes() -> Vec<Route> {
    routes![get_all_vehicles, create_new_vehicle,]
}
