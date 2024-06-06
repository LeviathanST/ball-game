mod player;

use bevy::{app::App, DefaultPlugins};
use player::plugin::PlayerPlugin;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PlayerPlugin)
    .run();
}
