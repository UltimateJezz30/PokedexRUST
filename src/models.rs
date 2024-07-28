use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Pokemon {
    pub id_p: i32,
    pub nombre: String,
    pub peso: i32,
    pub altura: i32,
    pub tipo: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Entrenador {
    pub id_entrenador: i32,
    pub nombre_e: String,
    pub nro_pokedex: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Habilidad {
    pub nombre_h: String,
    pub daño: i32,
    pub c_lanzamientos: i32,
    pub elemento: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Region {
    pub id_region: i32,
    pub nombre_r: String,
    pub clima: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Conexion {
    pub id_general: i32,
    pub n_registro: i32,
    pub v_registro: i32,
}
