use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::Rocket;
use std::env;
use crate::controllers::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn rocket() -> Rocket<rocket::Build> {
    let rocket = rocket::build()
        .mount("/api/vehicle", vehicle_controller::vehicle_routes());

    rocket
}
