use bevy::math::Vec3;
use bevy::prelude::{Entity, GlobalTransform};

/// Defines a cloth vertex anchor through a `target` and `offset`
#[derive(Debug, Copy, Clone, Default)]
#[must_use]
pub struct VertexAnchor {
    /// Optional custom anchor target entity. If not set, the cloth entity will be used
    pub custom_target: Option<Entity>,
    /// optional custom offset. If not set the base vertex position will be used
    pub custom_offset: Option<Vec3>,
    /// If set to true, the base vertex position will be ignored.
    /// If [`custom_offset`] is defined, it will then override the vertex position
    pub ignore_vertex_position: bool,
    /// Is the anchoring in world space
    pub world_space: bool,
}

impl VertexAnchor {
    /// Tries to get the anchor world space position.
    ///
    /// Will return `None`:
    /// - if the anchor doesn't have an [`VertexAnchorOffset`] set, which is an expected behaviour
    /// defaulting to the base vertex position `original_pos`.
    ///- if the query can't find the target's global transform which is unexpected.
    ///
    /// # Arguments
    ///
    /// * `original_pos` - the original local space vertex position
    /// * `self_transform` - the `GlobalTransform` associated with the cloth entity use in case of [`VertexAnchorTarget::SelfTransform`]
    /// * `transform_query` - ECS query used in case of [`VertexAnchorTarget::CustomTransform`]
    #[inline]
    #[must_use]
    pub fn get_position(
        &self,
        original_pos: Vec3,
        self_transform: &GlobalTransform,
        query: impl Fn(Entity) -> Option<GlobalTransform>,
    ) -> Vec3 {
        let transform = self
            .custom_target
            .and_then(query)
            .unwrap_or(*self_transform);
        let local_pos = if self.ignore_vertex_position {
            Vec3::ZERO
        } else {
            original_pos
        } + self.custom_offset.unwrap_or(Vec3::ZERO);
        let matrix = transform.compute_matrix();
        matrix.transform_point3(local_pos)
    }
}
