use bevy::app::{Plugin, Startup, Update};

use super::system::{check_collision_with_player, confine_enemy_movement, enemies_movement, spawn_enemies, update_enemies_movement};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_enemies)
        .add_systems(Update, update_enemies_movement)
        .add_systems(Update, enemies_movement)
        // .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, check_collision_with_player);
    }
}