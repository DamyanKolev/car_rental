use diesel::prelude::*;
use rocket::http::Status;

use crate::config::establish_connection;
use crate::constants::message_constants;
use crate::models::response::*;
use crate::models::vehicle::*;
use crate::schema::vehicles;
use crate::schema::vehicles::dsl::*;

pub fn select_all() -> ResponseWithStatus{
    let connection = &mut establish_connection();
    let result = vehicles.load::<Vehicle>(connection).is_ok();

    if result {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: serde_json::to_value("").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_CAN_NOT_SELECT_DATA),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn create_new(data: InsertVehicle) -> ResponseWithStatus {
    let connection = &mut establish_connection();
    let result = diesel::insert_into(vehicles::table)
        .values(&data)
        .execute(connection)
        .is_ok();

    if result {
        ResponseWithStatus {
            status_code: Status::Created.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: serde_json::to_value("").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::InternalServerError.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_CAN_NOT_INSERT_DATA),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}
