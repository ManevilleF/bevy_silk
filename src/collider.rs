use bevy::ecs::component::Component;
use bevy_rapier3d::prelude::InteractionGroups;

/// Enables collisions on a cloth entity
///
/// The collisions will be detected by casting a cuboid shape using the cloth AABB bounding box
#[derive(Debug, Clone, Component)]
pub struct ClothCollider {
    /// Collision interaction groups, all by default
    pub interaction_groups: InteractionGroups,
    /// offset to apply on collision projected point to prevent clipping
    pub offset: f32,
    /// Coefficient of the received velocity to apply to cloth:
    /// - 0 meaning no velocity will be applied
    /// - 1 meaning velocity is fully applied
    /// - 2 meaning the double velocity is applied
    pub velocity_coefficient: f32,
    /// Defines the velocity reduction coefficient for dynamic rigibodies colliding with the cloth,
    /// improving the cloth effect.
    pub dampen_others: Option<f32>,
}

impl Default for ClothCollider {
    fn default() -> Self {
        Self {
            interaction_groups: InteractionGroups::all(),
            offset: 0.2,
            velocity_coefficient: 1.0,
            dampen_others: None,
        }
    }
}
