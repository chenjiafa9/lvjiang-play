use bevy::prelude::*;

mod animation;
mod audio;
mod camera;
mod general_system;
mod input;
mod model;
mod scene;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(setup::SetupPlugin)
        .add_plugins(input::KeyboardInputPlugin)
        .run();
}
