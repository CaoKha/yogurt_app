use bevy::prelude::*;

use crate::{ball::Ball, primitive_components::Position};

// --- Start Enum ---
pub enum Scorer {
    AI,
    Player,
}
// --- End Enum ---

// --- Start Resource ---
#[derive(Resource, Default)]
pub struct Score {
    pub player: u32,
    pub ai: u32,
}

// --- End Resource ---

// --- Start Event ---
#[derive(Event)]
pub struct Scored(pub Scorer);
// --- End Event ---

#[derive(Component)]
pub struct PlayerScore;

#[derive(Component)]
pub struct AiScore;

pub fn update_scoreboard(
    mut player_score: Query<&mut Text, With<PlayerScore>>,
    mut ai_score: Query<&mut Text, (With<AiScore>, Without<PlayerScore>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut player_score) = player_score.get_single_mut() {
            player_score.sections[0].value = score.player.to_string();
        }
        if let Ok(mut ai_score) = ai_score.get_single_mut() {
            ai_score.sections[0].value = score.ai.to_string();
        }
    }
}

pub fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<Scored>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        if let Ok(ball) = ball.get_single_mut() {
            if ball.0.x > window_width / 2. {
                events.send(Scored(Scorer::Player));
            } else if ball.0.x < -window_width / 2. {
                events.send(Scored(Scorer::AI));
            }
        }
    }
}

pub fn update_score(mut score: ResMut<Score>, mut events: EventReader<Scored>) {
    for event in events.read() {
        match event.0 {
            Scorer::Player => score.player += 1,
            Scorer::AI => score.ai += 1,
        }
    }

    println!("Score: {} - {}", score.player, score.ai);
}

pub fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        PlayerScore,
    ));

    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        AiScore,
    ));
}

// pub fn spawn_winner_board(mut score: Res<Score>, mut commands: Commands) {
//     if score.player == 10 {
//         commands.spawn((TextBundle::from_section("Winner: ")));
//     }
// }
