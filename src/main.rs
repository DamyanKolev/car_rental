use std::process::Command;

mod config;
mod constants;
mod controllers;
mod models;
mod schema;
mod services;

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenvy;
extern crate serde_json;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // let _rocket = config::rocket().launch().await?;

    Command::new("cmd")
        .args(["/C", "cd", "C:\"Users\"User\"Documents\"Damyan\"Projects\"car_rental_backend\"car-rental-frontend"])
        .spawn()
        .expect("ls command failed to start");

    Ok(())
}
