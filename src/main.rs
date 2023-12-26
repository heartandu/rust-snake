use bevy::prelude::*;
use snake::snake::SnakeApp;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SnakeApp))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
