use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::{IntoSystemConfigs, IntoSystemSetConfigs},
};

use super::{
    system::{confine_player_movement, player_movement, spawn_camera, spawn_player},
    system_set::PlayerSystemSet,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            PlayerSystemSet::Confinement.before(PlayerSystemSet::Movement),
        )
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, confine_player_movement.in_set(PlayerSystemSet::Movement))
        .add_systems(Update, player_movement.in_set(PlayerSystemSet::Confinement));
    }
}
