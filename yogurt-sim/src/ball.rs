use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{primitive_components::*, score_board::{Scored, Scorer}, BALL_SIZE, BALL_SPEED};

// --- Start BallBundle ---
#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    position: Position,
    velocity: Velocity,
    shape: Shape,
}

impl BallBundle {
    pub fn new(p_x: f32, p_y: f32) -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(p_x, p_y)),
            velocity: Velocity(Vec2::new(BALL_SPEED, BALL_SPEED)),
            shape: Shape(Vec2::splat(BALL_SIZE)),
        }
    }
}

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball ...");
    let shape = Mesh::from(Circle::new(BALL_SIZE));
    let color = ColorMaterial::from(Color::rgb(0., 1., 1.));
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    commands
        .spawn((
            BallBundle::new(0., 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
            },
        ))
        .insert(Transform::default());
}

pub fn project_positions(mut ball: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut ball {
        transform.translation = position.0.extend(0.);
    }
}

pub fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0;
    }
}

pub fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        if let Ok((mut position, mut velocity)) = ball.get_single_mut() {
            match event.0 {
                Scorer::Player => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(-BALL_SPEED, BALL_SPEED);
                }
                Scorer::AI => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(BALL_SPEED, -BALL_SPEED);
                }
            }
        }
    }
}
// --- End BallBundle ---
