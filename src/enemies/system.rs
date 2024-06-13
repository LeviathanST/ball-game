use bevy::{
    asset::{AssetServer, Handle},
    audio::{AudioBundle, AudioSource, PlaybackSettings},
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    math::{Vec2, Vec3},
    prelude::default,
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    window::{PrimaryWindow, Window},
};
use rand::random;

use super::component::Enemy;
use crate::player::{component::Player, system::PLAYER_SIZE};

/*-------------Constant-------------*/
const NUMBER_OF_ENEMIES: usize = 5;
const ENEMY_SIZE: f32 = 64.0;
const ENEMY_SPEED: f32 = 200.0;

/*-----------Function-------------*/
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * (window.width() - ENEMY_SIZE) + ENEMY_SIZE / 2.0;
        let random_y = random::<f32>() * (window.height() - ENEMY_SIZE) + ENEMY_SIZE / 2.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random_x, random_y).normalize(),
            },
        ));
    }
}

pub fn enemies_movement(mut enemies_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemies_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemies_movement(
    mut commands: Commands,
    mut enemies_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemies_query.iter_mut() {
        let translation = transform.translation;
        let mut direction_changed = false;

        if translation.x <= x_min || translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y <= y_min || translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // SFX
        if direction_changed {
            let effect_sound_1: Handle<AudioSource> = asset_server.load("audio/pluck_001.ogg");
            let effect_sound_2: Handle<AudioSource> = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<bool>() {
                effect_sound_1
            } else {
                effect_sound_2
            };

            commands.spawn(AudioBundle {
                source: sound_effect,
                settings: PlaybackSettings::ONCE,
                ..default()
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemies_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + window.width();
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + window.height();
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemies_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        };

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        };

        transform.translation = translation;
    }

}

pub fn check_collision_with_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemies_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemies_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance <= player_radius + enemy_radius {
                println!("GAME OVER!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");

                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings::ONCE,
                    ..default()
                });

                commands.entity(player_entity).despawn();
            }
        }
    }
}
