use rusqlite::{params, Connection};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub image: String,
}

pub fn get_all() -> Vec<Pokemon> {
    let mut pokemons = Vec::new();

    let conn = Connection::open(&"pokemon.db").unwrap();

    let mut stmt = conn.prepare("SELECT * FROM pokemons").unwrap();

    let results = stmt
        .query_map(params![], |row| {
            Ok(Pokemon {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                image: row.get(2).unwrap(),
            })
        })
        .unwrap();

    for pokemon in results {
        pokemons.push(pokemon.unwrap())
    }

    pokemons
}

pub fn create(pokemon: &Pokemon) {
    let conn = Connection::open(&"pokemon.db").unwrap();

    conn.execute(
        "INSERT INTO pokemons (id, name, image) VALUES (?1, ?2, ?3)",
        params![pokemon.id, pokemon.name, pokemon.image],
    )
    .unwrap();
}
