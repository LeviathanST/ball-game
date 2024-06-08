use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Enemies {
    pub direction: Vec2,
}
