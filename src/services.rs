use crate::models::*;
use crate::repositories::*;
use diesel::pg::PgConnection;

pub fn fetch_all_pokemon(conn: &PgConnection) -> Vec<Pokemon> {
    get_all_pokemon(conn).unwrap()
}

pub fn fetch_all_entrenadores(conn: &PgConnection) -> Vec<Entrenador> {
    get_all_entrenadores(conn).unwrap()
}

pub fn fetch_all_habilidades(conn: &PgConnection) -> Vec<Habilidad> {
    get_all_habilidades(conn).unwrap()
}

pub fn fetch_all_regiones(conn: &PgConnection) -> Vec<Region> {
    get_all_regiones(conn).unwrap()
}

pub fn fetch_all_conexiones(conn: &PgConnection) -> Vec<Conexion> {
    get_all_conexiones(conn).unwrap()
}
