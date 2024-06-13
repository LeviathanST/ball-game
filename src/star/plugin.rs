use bevy::app::{Plugin, Startup, Update};

use super::system::{player_hit_start, spawn_star, update_score};

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_star)
            .add_systems(Update, (player_hit_start, update_score));
    }
}
