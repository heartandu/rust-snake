/*
TODO
    1. Implement walls and collision with walls and snake itself
    2. Mouse spawning and snake growing
    3. Scoring system
*/

use bevy::prelude::*;

const BLOCK_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);

const SNAKE_STARTING_LENGTH: i32 = 4;
const SNAKE_STARTING_POSITION: Position = Position::new(0.0, 0.0);
const SNAKE_STARTING_DIRECTION: Direction = Direction::Right;
const SNAKE_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

pub struct SnakeApp;

impl Plugin for SnakeApp {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(MoveTimer(Timer::from_seconds(0.15, TimerMode::Repeating)))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, move_snake)
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

#[derive(Resource, Deref, DerefMut)]
struct MoveTimer(Timer);

#[derive(Component)]
struct Snake;

#[derive(Bundle)]
struct BlockBundle {
    id: Id,
    sprite_bundle: SpriteBundle,
    position: Position,
    direction: Direction,
}

impl BlockBundle {
    fn new(id: i32, color: Color, position: Position, direction: Direction) -> BlockBundle {
        let translation = Vec3::new(
            position.x * BLOCK_SIZE.x,
            position.y * BLOCK_SIZE.y,
            0.0,
        );

        BlockBundle {
            id: Id(id),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation,
                    scale: BLOCK_SIZE,
                    ..default()
                },
                sprite: Sprite {
                    color,
                    ..default()
                },
                ..default()
            },
            position,
            direction,
        }
    }
}

#[derive(Component, Clone)]
struct Id(i32);

#[derive(Component, Deref, DerefMut)]
struct Position(Vec2);

impl Position {
    const fn new(x: f32, y: f32) -> Position {
        Position(Vec2::new(x, y))
    }

    fn apply_vel(&mut self, velocity: &Velocity) {
        self.x += velocity.x;
        self.y += velocity.y;
    }

    fn translation(&self) -> Vec3 {
        Vec3::new(
            self.x * BLOCK_SIZE.x,
            self.y * BLOCK_SIZE.y,
            0.0,
        )
    }
}

#[derive(Component, Clone, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Direction {
    fn velocity(&self) -> Velocity {
        match self {
            Direction::Left => Velocity(Vec2::new(-1.0, 0.0)),
            Direction::Right => Velocity(Vec2::new(1.0, 0.0)),
            Direction::Down => Velocity(Vec2::new(0.0, -1.0)),
            Direction::Up => Velocity(Vec2::new(0.0, 1.0)),
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Snake
    let delta = 1.0 / SNAKE_STARTING_LENGTH as f32;
    for i in 0..SNAKE_STARTING_LENGTH {
        let blocks_offset = Direction::Left.velocity();
        let mut color = SNAKE_COLOR;

        color.set_r(delta * i as f32);

        commands.spawn((
            BlockBundle::new(
                i,
                color,
                Position::new(
                    SNAKE_STARTING_POSITION.x + i as f32 * blocks_offset.x,
                    SNAKE_STARTING_POSITION.y + i as f32 * blocks_offset.y,
                ),
                SNAKE_STARTING_DIRECTION,
            ),
            Snake,
        ));
    }
}

fn move_snake(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Position, &mut Direction), With<Snake>>,
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
) {
    let (_, _, head_dir) = query.iter().next().unwrap();

    let directions: Vec<Direction> = keys.get_pressed().filter_map(|k| match k {
        KeyCode::A if *head_dir != Direction::Right => Some(Direction::Left),
        KeyCode::D if *head_dir != Direction::Left => Some(Direction::Right),
        KeyCode::W if *head_dir != Direction::Down => Some(Direction::Up),
        KeyCode::S if *head_dir != Direction::Up => Some(Direction::Down),
        _ => None,
    }).collect();

    timer.tick(time.delta());

    let mut i = 0;
    let mut prev_dir = None;

    for (mut transform, mut pos, mut dir) in query.iter_mut() {
        if timer.just_finished() {
            pos.apply_vel(&dir.velocity());
            transform.translation = pos.translation();

            if let Some(d) = prev_dir {
                prev_dir = Some(dir.clone());
                *dir = d.clone();
            } else {
                prev_dir = Some(dir.clone());
            }
        }

        if i == 0 {
            if let Some(d) = directions.first() {
                *dir = d.clone();
            }
        }

        i += 1;
    }
}
