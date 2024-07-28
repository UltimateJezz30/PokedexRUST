#[macro_use] extern crate rocket;

mod models;
mod repositories;
mod services;
mod controllers;
mod schema;

use controllers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/api", routes![get_pokemons, get_entrenadores, get_habilidades, get_regiones, get_conexiones])
}
