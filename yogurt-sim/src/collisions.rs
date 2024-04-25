use bevy::{math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume}, prelude::*};

use crate::{ball::Ball, primitive_components::{Position, Shape, Velocity}};

// --- Start Enum ---
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}
// --- End Enum ---

pub fn handle_collisions(
    mut ball: Query<(&Position, &mut Velocity, &Shape), With<Ball>>,
    other_thing: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((ball_position, mut ball_velocity, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &other_thing {
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(position.0, shape.0 / 2.),
            ) {
                match collision {
                    Collision::Left => ball_velocity.0.x *= -1.,
                    Collision::Right => ball_velocity.0.x *= -1.,
                    Collision::Top => ball_velocity.0.y *= -1.,
                    Collision::Bottom => ball_velocity.0.y *= -1.,
                }
            }
        }
    }
}

// --- Start Utils ---
fn collide_with_side(bound_circle: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !bound_circle.intersects(&wall) {
        return None;
    }
    let closest_point_of_wall = wall.closest_point(bound_circle.center());
    let offset = closest_point_of_wall - bound_circle.center();

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left // (wall) | <--- . (point)
        } else {
            Collision::Right // (point) . ---> | (wall)
        }
    } else if offset.y > 0. {
        // --
        // ^
        // |
        // .
        Collision::Top
    } else {
        // .
        // |
        // v
        // --
        Collision::Bottom
    };
    Some(side)
}

// --- End Utils ---
