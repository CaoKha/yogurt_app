use bevy::prelude::*;


// --- General Component ---
#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Shape(pub Vec2);


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AI;
// --- End General Component ---
