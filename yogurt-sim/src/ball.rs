use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    primitive_components::{Acceleration, Position, PreviousPosition, Shape, Velocity}, score_board::{Scored, Scorer}, BALL_ACCEL, BALL_INITIAL_SPEED, BALL_RADIUS, DELTA_TIME, GUTTER_HEIGHT, VELOCITY_THRESHOLD
};

// --- Start BallBundle ---
#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    position: Position,
    previous_position: PreviousPosition,
    velocity: Velocity,
    shape: Shape,
    accel: Acceleration,
}

impl BallBundle {
    pub fn new(p_x: f32, p_y: f32) -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(p_x, p_y)),
            previous_position: PreviousPosition(Vec2::new(0., 0.)),
            velocity: Velocity(Vec2::new(BALL_INITIAL_SPEED, BALL_INITIAL_SPEED)),
            shape: Shape(Vec2::splat(BALL_RADIUS)),
            accel: Acceleration(BALL_ACCEL),
        }
    }
}

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball ...");
    let shape = Mesh::from(Circle::new(BALL_RADIUS));
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

pub fn move_ball(
    time: Res<Time>,
    mut ball: Query<(&mut Position, &mut PreviousPosition, &mut Velocity, &Acceleration), With<Ball>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2. - GUTTER_HEIGHT - BALL_RADIUS;
        if let Ok((mut position, mut previous_position, mut velocity, accel)) = ball.get_single_mut() {
            let delta_time = DELTA_TIME;
            let new_velocity = if velocity.0.x.abs() < VELOCITY_THRESHOLD {
                velocity.0 + accel.0 * delta_time } else {
                    velocity.0 
                };
            let new_position =
                position.0 + velocity.0 * delta_time;
            velocity.0 = new_velocity;
            previous_position.0 = position.0;
            if new_position.y <= max_y {
                position.0 = new_position;
            } else {
                position.0 = Vec2::new(new_position.x, max_y);
            }
            // println!(
            //     "position_y = {}, max_y = {}, velocity = {}",
            //     position.0.y, max_y, velocity.0
            // );
        }
    }
}

pub fn reset_ball(
    mut ball: Query<
        (
            &mut Position,
            &mut Velocity,
            &mut Acceleration,
        ),
        With<Ball>,
    >,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        if let Ok((mut position, mut velocity, mut accel)) = ball.get_single_mut() {
            match event.0 {
                Scorer::Player => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(-BALL_INITIAL_SPEED, BALL_INITIAL_SPEED);
                    accel.0 = -BALL_ACCEL;
                }
                Scorer::AI => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(BALL_INITIAL_SPEED, -BALL_INITIAL_SPEED);
                    accel.0 = BALL_ACCEL;
                }
            }
        }
    }
}
// --- End BallBundle ---
