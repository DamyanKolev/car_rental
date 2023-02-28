// @generated automatically by Diesel CLI.

diesel::table! {
    vehicles (id) {
        id -> Int4,
        brand -> Varchar,
        model -> Varchar,
        vehicle_type -> Varchar,
        engine -> Varchar,
    }
}
