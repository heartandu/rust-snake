use bevy::prelude::*;
use snake::snake::SnakeApp;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SnakeApp))
        .run();
}
