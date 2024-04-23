use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// --- Start Constants ---
const BALL_SIZE: f32 = 5.;
const PADDLE_SPEED: f32 = 1.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;
// --- End Constants ---

// --- General Component ---
#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);
// --- End General Component ---

// --- Start BallBundle ---
#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    position: Position,
    velocity: Velocity,
}

impl BallBundle {
    fn new(p_x:f32, p_y:f32, v_x: f32, v_y: f32) -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(p_x, p_y)),
            velocity: Velocity(Vec2::new(v_x, v_y)),
        }
    }
}
// --- End BallBundle ---

// --- Start PaddleBundle ---
#[derive(Component)]
struct Paddle;

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    position: Position,
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            position: Position(Vec2::new(x, y)),
        }
    }
}
// --- End PaddleBundle ---

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_paddle, spawn_camera))
        .add_systems(Update, (move_ball, project_positions.after(move_ball)))
        .run();
}

// --- Start Systems ---
fn spawn_ball(
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
            BallBundle::new(-25., -25., 1., 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
            },
        ))
        .insert(Transform::default());
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

fn project_positions(mut ball: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut ball{
        transform.translation = position.0.extend(0.);
    }
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    println!("Moving ball ...");
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0;
    }
}

fn spawn_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning paddle ...");

    let mesh = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    let material = ColorMaterial::from(Color::rgb(0., 1., 0.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        PaddleBundle::new(0., 0.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}
// --- End Systems ---
