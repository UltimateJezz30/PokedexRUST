table! {
    public.pokemon (id_p) {
        id_p -> Int4,
        nombre -> Varchar,
        peso -> Int4,
        altura -> Int4,
        tipo -> Varchar,
    }
}

table! {
    public.entrenador (id_entrenador) {
        id_entrenador -> Int4,
        nombre_e -> Varchar,
        nro_pokedex -> Int4,
    }
}

table! {
    public.habilidad (nombre_h) {
        nombre_h -> Varchar,
        daño -> Int4,
        c_lanzamientos -> Int4,
        elemento -> Varchar,
    }
}

table! {
    public.region (id_region) {
        id_region -> Int4,
        nombre_r -> Varchar,
        clima -> Varchar,
    }
}

table! {
    public.conexion (id_general) {
        id_general -> Int4,
        n_registro -> Int4,
        v_registro -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    pokemon,
    entrenador,
    habilidad,
    region,
    conexion,
);
