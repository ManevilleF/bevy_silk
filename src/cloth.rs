use crate::point::Point;
use crate::stick::Stick;
use bevy::ecs::component::Component;
use bevy::math::Vec3;

#[derive(Debug, Clone, Component)]
pub struct Cloth {
    pub points: Vec<Point>,
    pub sticks: Vec<Stick>,
}
