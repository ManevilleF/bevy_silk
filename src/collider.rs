use bevy::ecs::component::Component;
use bevy_rapier3d::prelude::InteractionGroups;

/// Enables collisions on a cloth entity
///
/// The collisions will be detected by casting a cuboid shape using the cloth AABB bounding box
#[derive(Debug, Clone, Component)]
pub struct ClothCollider {
    /// Collision interaction groups, all by default
    pub interaction_groups: InteractionGroups,
}

impl Default for ClothCollider {
    fn default() -> Self {
        Self {
            interaction_groups: InteractionGroups::all(),
        }
    }
}
