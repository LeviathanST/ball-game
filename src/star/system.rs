use bevy::{
    asset::AssetServer,
    audio::AudioBundle,
    ecs::system::{Commands, Res},
    prelude::{DetectChanges, Entity, Query, ResMut, With},
    sprite::SpriteBundle,
    transform::components::Transform,
    utils::default,
    window::{PrimaryWindow, Window},
};
use rand::random;

use crate::{
    player::{component::Player, system::PLAYER_SIZE},
    score::resource::Score,
};

use super::component::Star;

/*---------Constant--------*/
const STAR_SIZE: f32 = 30.0;
const NUM_OF_STARS: u32 = 5;

/*---------Function--------*/
pub fn spawn_star(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUM_OF_STARS {
        let x_random: f32 = random::<f32>() * window.width();
        let y_random: f32 = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_random, y_random, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn player_hit_start(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    let window = window_query.get_single().unwrap();
    let half_star_size = STAR_SIZE / 2.0;
    let half_player_size = PLAYER_SIZE / 2.0;
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance <= half_player_size + half_star_size {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/laserLarge_000.ogg"),
                    ..default()
                });

                score.value += 1;

                let x_random: f32 = random::<f32>() * window.width();
                let y_random: f32 = random::<f32>() * window.height();

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x_random, y_random, 0.0),
                        texture: asset_server.load("sprites/star.png"),
                        ..default()
                    },
                    Star {},
                ));

                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score (score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}