use actix_web::web;

use super::game::{create_game, delete_game, get_game_by_id, get_games, updated_game};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_games)
        .service(get_game_by_id)
        .service(create_game)
        .service(updated_game)
        .service(delete_game);

    conf.service(scope);
}