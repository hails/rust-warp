use serde_derive::{Deserialize, Serialize};
use warp::Filter;

mod pokemon;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let root = warp::path("pokemons");

    let get_all = warp::get().map(|| {
        let pokemons = pokemon::get_all();
        warp::reply::json(&pokemons)
    });

    let get_by_id = warp::get().and(warp::path!(u32).map(|id| {
        let pokemons = pokemon::get_all();
        if let Some(pokemon) = pokemons.iter().filter(|p| p.id == id).nth(0) {
            let response = warp::reply::json(pokemon);

            warp::reply::with_status(response, warp::http::StatusCode::OK)
        } else {
            #[derive(Deserialize, Serialize)]
            struct ErrorResponse {
                body: String,
            };
            let response = warp::reply::json(&ErrorResponse {
                body: "Pokemon not found".to_owned(),
            });
            warp::reply::with_status(response, warp::http::StatusCode::NOT_FOUND)
        }
    }));

    let create = warp::post()
        .and(warp::body::json())
        .map(|pokemon: pokemon::Pokemon| {
            pokemon::create(&pokemon);
            let response = warp::reply::json(&pokemon);
            warp::reply::with_status(response, warp::http::StatusCode::CREATED)
        });

    let routes = root.and(get_by_id.or(get_all).or(create));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
