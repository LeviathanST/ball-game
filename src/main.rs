mod player;
mod enemies;

use bevy::{app::App, DefaultPlugins};
use enemies::plugin::EnemiesPlugin;
use player::plugin::PlayerPlugin;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PlayerPlugin)
    .add_plugins(EnemiesPlugin)
    .run();
}
