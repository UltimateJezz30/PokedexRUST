use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;

pub fn get_all_pokemon(conn: &PgConnection) -> QueryResult<Vec<Pokemon>> {
    pokemon::table.load::<Pokemon>(conn)
}

pub fn get_all_entrenadores(conn: &PgConnection) -> QueryResult<Vec<Entrenador>> {
    entrenador::table.load::<Entrenador>(conn)
}

pub fn get_all_habilidades(conn: &PgConnection) -> QueryResult<Vec<Habilidad>> {
    habilidad::table.load::<Habilidad>(conn)
}

pub fn get_all_regiones(conn: &PgConnection) -> QueryResult<Vec<Region>> {
    region::table.load::<Region>(conn)
}

pub fn get_all_conexiones(conn: &PgConnection) -> QueryResult<Vec<Conexion>> {
    conexion::table.load::<Conexion>(conn)
}
