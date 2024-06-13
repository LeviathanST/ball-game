mod enemies;
mod player;
mod score;
mod star;
mod window_event;

use bevy::{app::App, DefaultPlugins};
use enemies::plugin::EnemiesPlugin;
use player::plugin::PlayerPlugin;
use score::resource::Score;
use star::plugin::StarPlugin;
use window_event::plugin::WindowEventPlugin;
fn main() {
    App::new()
        .insert_resource::<Score>(Score { value: 0 })
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            EnemiesPlugin,
            StarPlugin,
            WindowEventPlugin,
        ))
        .run();
}
