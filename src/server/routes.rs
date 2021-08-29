use super::model::{Games, Input, Output};
use crate::game::model::*;
use axum::{extract, Json};
use http::StatusCode;
use std::sync::{Arc, Mutex};

pub async fn tick(
    games: extract::Extension<Arc<Mutex<Games>>>,
    extract::Json(input): extract::Json<Input>,
) -> Result<Json<Output>, StatusCode> {
    let mut games = games.lock().unwrap();

    tracing::debug!("input = {:?}", input);

    let next_move = match input {
        Input::Init(meta) => {
            let game = games.new_game(
                &meta.game_id,
                GameState {
                    meta: meta.clone(),
                    ..Default::default()
                },
            );

            game.init()
        }
        Input::Update(state) => {
            let game = games.game(&state.meta.game_id).ok_or(StatusCode::GONE)?;
            game.update(state)
        }
    };

    tracing::info!("chose move {:?}", next_move);

    Ok(Json(Output::Move(next_move)))
}

pub async fn debug(
    games: extract::Extension<Arc<Mutex<Games>>>,
) -> Result<Json<Games>, StatusCode> {
    let games = games.0.lock().unwrap();
    Ok(Json(games.clone()))
}
