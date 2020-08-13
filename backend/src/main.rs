#![feature(proc_macro_hygiene, decl_macro)]

use rand::Rng;
use rocket::response::NamedFile;
use rocket::{get, post, routes, State};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use skat::card::Card;
use skat::game::Round;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

struct Game {
    rounds: std::vec::Vec<Round>,
    player_ids: (u32, u32, u32),
}

fn rotate_players(game: &mut Game) {
    game.player_ids = (game.player_ids.1, game.player_ids.2, game.player_ids.0)
}

#[post("/game")]
fn create_game(games_map: State<Mutex<HashMap<u32, Game>>>) -> Json<u32> {
    let game = Game {
        rounds: std::vec::Vec::new(),
        player_ids: (0, 0, 0),
    };
    let mut rng = rand::thread_rng();
    let mut game_id;
    loop {
        game_id = rng.gen::<u32>();
        if game_id != 0 && !games_map.lock().unwrap().contains_key(&game_id) {
            break;
        }
    }
    games_map.lock().unwrap().insert(game_id, game);
    return Json(game_id);
}

#[post("/game/<game_id>/join")]
fn join_game(games_map: State<Mutex<HashMap<u32, Game>>>, game_id: u32) -> Option<Json<u32>> {
    match games_map.lock().unwrap().get_mut(&game_id) {
        None => None,
        Some(game) => {
            let mut rng = rand::thread_rng();
            let mut player_id;
            loop {
                player_id = rng.gen::<u32>();
                if player_id != 0 {
                    break;
                }
            }

            match game.player_ids {
                (0, 0, 0) => game.player_ids.0 = player_id,
                (_, 0, 0) => game.player_ids.1 = player_id,
                (_, _, 0) => game.player_ids.2 = player_id,
                (_, _, _) => panic!(), // TODO game full, dealer/spectators?
            }
            Some(Json(player_id))
        }
    }
}

#[post("/game/<game_id>/round")]
fn new_round(games_map: State<Mutex<HashMap<u32, Game>>>, game_id: u32) {
    match games_map.lock().unwrap().get_mut(&game_id) {
        None => panic!(), // TODO
        Some(game) => {
            let round = skat::game::new_round();
            game.rounds.push(round);
            rotate_players(game);
        }
    }
}

#[get("/game/<game_id>/round?<player_id>")]
fn get_limited_round(
    games_map: State<Mutex<HashMap<u32, Game>>>,
    game_id: u32,
    player_id: u32,
) -> Option<Json<[Option<Card>; 10]>> {
    match games_map.lock().unwrap().get(&game_id) {
        None => None,
        Some(game) => {
            let (f, m, r) = game.player_ids;
            let round = &game.rounds[game.rounds.len() - 1];
            if player_id == f {
                return Some(Json(round.forehand));
            } else if player_id == m {
                return Some(Json(round.middlehand));
            } else if player_id == r {
                return Some(Json(round.rearhand));
            } else {
                None
            }
        }
    }
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/index.html")).ok()
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/favicon.ico")).ok()
}

fn main() {
    let games_map: Mutex<HashMap<u32, Game>> = Mutex::new(HashMap::new());
    rocket::ignite()
        .manage(games_map)
        .mount("/static", StaticFiles::from("./static"))
        .mount(
            "/api",
            routes![create_game, join_game, get_limited_round, new_round],
        )
        .mount("/", routes![index, favicon])
        .launch();
}
