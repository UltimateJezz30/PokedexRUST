use rocket::serde::json::Json;
use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::database;

use crate::models::*;
use crate::services::*;

#[database("postgres")]
pub struct DbConn(diesel::PgConnection);

#[get("/pokemons")]
pub async fn get_pokemons(conn: DbConn) -> Json<Vec<Pokemon>> {
    conn.run(|c| Json(fetch_all_pokemon(c))).await
}

#[get("/entrenadores")]
pub async fn get_entrenadores(conn: DbConn) -> Json<Vec<Entrenador>> {
    conn.run(|c| Json(fetch_all_entrenadores(c))).await
}

#[get("/habilidades")]
pub async fn get_habilidades(conn: DbConn) -> Json<Vec<Habilidad>> {
    conn.run(|c| Json(fetch_all_habilidades(c))).await
}

#[get("/regiones")]
pub async fn get_regiones(conn: DbConn) -> Json<Vec<Region>> {
    conn.run(|c| Json(fetch_all_regiones(c))).await
}

#[get("/conexiones")]
pub async fn get_conexiones(conn: DbConn) -> Json<Vec<Conexion>> {
    conn.run(|c| Json(fetch_all_conexiones(c))).await
}
