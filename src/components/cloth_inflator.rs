use bevy::ecs::prelude::Component;
use bevy::math::Vec3;
use bevy::reflect::Reflect;

#[derive(Debug, Copy, Clone, Reflect, Default)]
pub enum PivotPosition {
    #[default]
    Center,
    AabbCenter,
    Custom(Vec3),
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct ClothInflator {
    pub pivot: PivotPosition,
    pub inflated_amount: f32,
}

impl Default for ClothInflator {
    fn default() -> Self {
        Self {
            pivot: Default::default(),
            inflated_amount: 1.0,
        }
    }
}
