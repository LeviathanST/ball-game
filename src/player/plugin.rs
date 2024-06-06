use bevy::app::{App, Plugin, Startup, Update};

use super::system::{confine_player_movement, player_movement, spawn_camera, spawn_player};

pub struct PlayerPlugin; 

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, confine_player_movement);
    }
}
