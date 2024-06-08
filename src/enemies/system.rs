use bevy::{
    asset::AssetServer,
    ecs::{
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

use super::component::Enemies;

/*-------------Constant-------------*/
const NUMBER_OF_ENEMIES: usize = 20;
const ENEMY_SIZE: f32 = 64.0;
const ENEMY_SPEED: f32 = 200.0;

/*-----------Enemies-------------*/
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemies {
                direction: Vec2::new(random_x, random_y).normalize(),
            },
        ));
    }
}

pub fn enemies_movement(mut enemies_query: Query<(&mut Transform, &Enemies)>, time: Res<Time>) {
    for (mut transform, enemy) in enemies_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemies_movement (
    mut enemies_query: Query<(&Transform, &mut Enemies)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemies_query.iter_mut() {
        let translation = transform.translation;

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
        }
    }
}
