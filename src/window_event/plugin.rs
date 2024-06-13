use bevy::app::{Plugin, Update};

use super::system::exit_game;

pub struct WindowEventPlugin;

impl Plugin for WindowEventPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, exit_game);
    }
}
