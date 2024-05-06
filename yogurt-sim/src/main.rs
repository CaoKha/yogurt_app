use bevy::prelude::*;

mod ball;
mod collisions;
mod constants;
mod gutters;
mod paddles;
mod primitive_components;
mod score_board;

use ball::{move_ball, project_positions, reset_ball, spawn_ball};
use collisions::handle_collisions;
use constants::*;
use gutters::spawn_gutters;
use paddles::{handle_player_input, move_ai, move_paddles, move_player_ai, player_not_playing, spawn_paddles};
use score_board::{
    detect_scoring, spawn_scoreboard, update_score, update_scoreboard, Score, Scored,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                resolution: (480., 360.).into(),
                // prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Score>()
        .add_event::<Scored>()
        .add_systems(
            Startup,
            (
                spawn_ball,
                spawn_paddles,
                spawn_gutters,
                spawn_camera,
                spawn_scoreboard,
            ),
        )
        .add_systems(
            Update,
            (
                move_ball,
                // handle_player_input,
                move_ai,
                // move_player_ai.run_if(player_not_playing),
                move_paddles,
                move_player_ai,
                detect_scoring,
                reset_ball.after(detect_scoring),
                update_score.after(detect_scoring),
                update_scoreboard.after(detect_scoring),
                // move_paddles.after(handle_player_input),
                project_positions.after(move_ball),
                handle_collisions.after(move_ball),
            ),
        )
        .run();
}

// --- Start Systems ---
fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

// --- End Systems ---
