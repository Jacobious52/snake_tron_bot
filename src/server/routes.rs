use super::model::Games;
use crate::game::model::*;
use axum::{extract, Json};
use http::StatusCode;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerQuery {
    pub player_number: usize,
}

pub async fn init(
    games: extract::Extension<Arc<Mutex<Games>>>,
    _query: extract::Query<PlayerQuery>,
    extract::Path(_modifier): extract::Path<String>,
    extract::Json(meta): extract::Json<Meta>,
) -> Result<Json<Move>, StatusCode> {
    let mut games = games.lock().unwrap();

    tracing::debug!("input = {:?}", meta);

    let game = games.new_game(
        &meta.game_id,
        GameState {
            meta: meta.clone(),
            ..Default::default()
        },
    );

    let next_move = game.init();

    tracing::info!("initial chose move {:?}", next_move);

    Ok(Json(next_move))
}

pub async fn update(
    games: extract::Extension<Arc<Mutex<Games>>>,
    extract::Path(modifier): extract::Path<String>,
    query: extract::Query<PlayerQuery>,
    extract::Json(state): extract::Json<GameState>,
) -> Result<Json<Move>, StatusCode> {
    let mut games = games.lock().unwrap();
    let game = games.game(&state.meta.game_id).ok_or(StatusCode::GONE)?;

    let tick = state.tick;

    let player_number = query.player_number;
    let next_move = game.update(player_number, &modifier, state);

    tracing::info!(
        "[{}] mod {} update chose move {:?}",
        tick,
        modifier,
        next_move
    );

    Ok(Json(next_move))
}

pub async fn end(
    games: extract::Extension<Arc<Mutex<Games>>>,
    _query: extract::Query<PlayerQuery>,
    extract::Path(_modifier): extract::Path<String>,
    extract::Json(state): extract::Json<GameState>,
) -> Result<Json<Move>, StatusCode> {
    let mut games = games.lock().unwrap();
    games.end_game(&state.meta.game_id);

    tracing::info!("died. removing game");

    Ok(Json(Move::E))
}

pub async fn debug(
    games: extract::Extension<Arc<Mutex<Games>>>,
) -> Result<Json<Games>, StatusCode> {
    let games = games.0.lock().unwrap();
    Ok(Json(games.clone()))
}
