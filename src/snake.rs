/*
TODO
    1. Implement walls and collision with walls and snake itself - DONE
    2. Mouse spawning and snake growing - DONE
    3. Scoring system
*/

use std::ops::Mul;
use bevy::{
    prelude::*,
    sprite::collide_aabb::collide,
};
use rand::Rng;

const BLOCK_SIZE: Vec3 = Vec3::new(20.0, 20.0, 1.0);
const SCREEN_HEIGHT: f32 = 22.0;
const SCREEN_WIDTH: f32 = 40.0;

const SNAKE_STARTING_LENGTH: i32 = 4;
const SNAKE_STARTING_POSITION: Position = Position::new(0.0, 0.0);
const SNAKE_STARTING_DIRECTION: Direction = Direction::Right;

const WALL_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const MOUSE_COLOR: Color = Color::rgb(1.0, 0.65, 0.34);
const SNAKE_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

pub struct SnakeApp;

impl Plugin for SnakeApp {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(MoveTimer(Timer::from_seconds(0.15, TimerMode::Repeating)))
            .add_systems(Startup, setup)
            .add_systems(Update, (
                move_snake,
                check_collisions,
                bevy::window::close_on_esc,
            ));
    }
}

#[derive(Component, PartialEq)]
enum GameState {
    Running,
    Paused,
}

#[derive(Resource, Deref, DerefMut)]
struct MoveTimer(Timer);

#[derive(Component)]
struct Snake(u32);

#[derive(Bundle)]
struct SnakeBundle {
    block_bundle: BlockBundle,
    snake: Snake,
    direction: Direction,
    collider: Collider,
}

impl SnakeBundle {
    fn new(id: u32, block_bundle: BlockBundle, direction: Direction) -> SnakeBundle {
        SnakeBundle {
            block_bundle,
            snake: Snake(id),
            direction,
            collider: Collider,
        }
    }
}

#[derive(Component)]
struct Mouse;

#[derive(Bundle)]
struct MouseBundle {
    block_bundle: BlockBundle,
    mouse: Mouse,
    collider: Collider,
}

impl MouseBundle {
    fn new(block_size: Vec3) -> MouseBundle {
        let x_pos = SCREEN_WIDTH / 2.0 - 1.0;
        let y_pos = SCREEN_HEIGHT / 2.0 - 1.0;

        let mut rng = rand::thread_rng();

        MouseBundle {
            block_bundle: BlockBundle::new(
                MOUSE_COLOR,
                Position(Vec2::new(
                    rng.gen_range(-x_pos..=x_pos).round(),
                    rng.gen_range(-y_pos..=y_pos).round(),
                )),
                block_size,
            ),
            mouse: Mouse,
            collider: Collider,
        }
    }
}

#[derive(Bundle)]
struct BlockBundle {
    sprite_bundle: SpriteBundle,
    position: Position,
}

impl BlockBundle {
    fn new(color: Color, position: Position, block_size: Vec3) -> BlockBundle {
        BlockBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        position.x * block_size.x,
                        position.y * block_size.y,
                        0.0,
                    ),
                    scale: block_size,
                    ..default()
                },
                sprite: Sprite {
                    color,
                    ..default()
                },
                ..default()
            },
            position,
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

#[derive(Component, Copy, Clone, PartialEq, Debug)]
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

    fn reverse(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    fn new(location: WallLocation, block_size: Vec3) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.translation(block_size),
                    scale: location.scale(block_size),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

#[derive(Component)]
struct Collider;

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn translation(&self, block_size: Vec3) -> Vec3 {
        let (start, end) = self.points();

        let x_pos = (start.x + end.x) / 2.0;
        let y_pos = (start.y + end.y) / 2.0;

        Vec3::new(x_pos, y_pos, 0.0).mul(block_size)
    }

    fn scale(&self, block_size: Vec3) -> Vec3 {
        let (start, end) = self.points();

        let dx = (start.x - end.x).abs() + 1.0;
        let dy = (start.y - end.y).abs() + 1.0;

        Vec3::new(dx, dy, 1.0).mul(block_size)
    }

    fn points(&self) -> (Vec2, Vec2) {
        let x_pos = SCREEN_WIDTH / 2.0;
        let y_pos = SCREEN_HEIGHT / 2.0;

        match self {
            WallLocation::Left => (Vec2::new(-x_pos, -y_pos), Vec2::new(-x_pos, y_pos)),
            WallLocation::Right => (Vec2::new(x_pos, y_pos), Vec2::new(x_pos, -y_pos)),
            WallLocation::Bottom => (Vec2::new(x_pos, -y_pos), Vec2::new(-x_pos, -y_pos)),
            WallLocation::Top => (Vec2::new(-x_pos, y_pos), Vec2::new(x_pos, y_pos)),
        }
    }
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Game state
    commands.spawn(GameState::Running);

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left, BLOCK_SIZE));
    commands.spawn(WallBundle::new(WallLocation::Top, BLOCK_SIZE));
    commands.spawn(WallBundle::new(WallLocation::Right, BLOCK_SIZE));
    commands.spawn(WallBundle::new(WallLocation::Bottom, BLOCK_SIZE));

    // Mouse
    commands.spawn(MouseBundle::new(BLOCK_SIZE));

    // Snake
    let delta = 1.0 / SNAKE_STARTING_LENGTH as f32;
    let blocks_offset = SNAKE_STARTING_DIRECTION.reverse().velocity();
    let mut color = SNAKE_COLOR;
    for i in 0..SNAKE_STARTING_LENGTH {
        color.set_r(delta * i as f32);

        commands.spawn(SnakeBundle::new(
            i as u32,
            BlockBundle::new(
                color,
                Position::new(
                    SNAKE_STARTING_POSITION.x + i as f32 * blocks_offset.x,
                    SNAKE_STARTING_POSITION.y + i as f32 * blocks_offset.y,
                ),
                BLOCK_SIZE,
            ),
            SNAKE_STARTING_DIRECTION,
        ));
    }
}

fn move_snake(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Position, &mut Direction), With<Snake>>,
    state_query: Query<&GameState>,
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
) {
    if *state_query.single() != GameState::Running {
        return;
    }

    {
        // Handle keyboard controls
        let (_, _, mut head_dir) = query.iter_mut().next().unwrap();

        let directions: Vec<Direction> = keys.get_pressed().filter_map(|k| match k {
            KeyCode::Left | KeyCode::A if *head_dir != Direction::Right => Some(Direction::Left),
            KeyCode::Right | KeyCode::D if *head_dir != Direction::Left => Some(Direction::Right),
            KeyCode::Up | KeyCode::W if *head_dir != Direction::Down => Some(Direction::Up),
            KeyCode::Down | KeyCode::S if *head_dir != Direction::Up => Some(Direction::Down),
            _ => None,
        }).collect();

        if let Some(d) = directions.first() {
            *head_dir = d.clone();
        }
    }

    // Move the snake
    if timer.tick(time.delta()).just_finished() {
        let mut prev_dir = None;
        for (mut transform, mut pos, mut dir) in query.iter_mut() {
            pos.apply_vel(&dir.velocity());
            transform.translation = pos.translation();

            if let Some(d) = prev_dir {
                prev_dir = Some(dir.clone());
                *dir = d.clone();
            } else {
                prev_dir = Some(dir.clone());
            }
        }
    }
}

fn check_collisions(
    mut commands: Commands,
    mut state_query: Query<&mut GameState>,
    snake_query: Query<(&Snake, &Transform, &Position, &Direction), With<Snake>>,
    collider_query: Query<(Entity, &Transform, Option<&Snake>, Option<&Mouse>), With<Collider>>,
) {
    let snake: Vec<(&Snake, &Transform, &Position, &Direction)> = snake_query.iter().collect();

    let (head, head_transform, _, _) = snake.first().unwrap();

    for (entity, transform, maybe_snake, maybe_mouse) in collider_query.iter() {
        // Do not collide snake head with itself
        if let Some(snake) = maybe_snake {
            if snake.0 == head.0 {
                continue;
            }
        }

        let collision = collide(
            head_transform.translation,
            head_transform.scale.truncate(),
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(_) = collision {
            // If collided with mouse, spawn a new one
            if maybe_mouse.is_some() {
                commands.entity(entity).despawn();

                let mut mouse_bundle = MouseBundle::new(BLOCK_SIZE);
                // Check if we are trying to spawn a mouse inside the snake
                while snake.iter().find(|(_, _, position, _)| {
                    position.x == mouse_bundle.block_bundle.position.x
                        && position.y == mouse_bundle.block_bundle.position.y
                }).is_some() {
                    mouse_bundle = MouseBundle::new(BLOCK_SIZE);
                }

                commands.spawn(mouse_bundle);

                // Spawn a new snake block behind the current tail block
                let (tail, _, tail_position, &tail_direction) = snake.last().unwrap();
                let pos_offset = tail_direction.reverse().velocity();
                commands.spawn(SnakeBundle::new(
                    tail.0 + 1,
                    BlockBundle::new(
                        SNAKE_COLOR,
                        Position::new(
                            tail_position.x + pos_offset.x,
                            tail_position.y + pos_offset.y,
                        ),
                        BLOCK_SIZE,
                    ),
                    tail_direction,
                ));

                return;
            }

            // If collided with wall or snake itself, stop the game
            let mut state = state_query.single_mut();
            *state = GameState::Paused;
        }
    }
}
