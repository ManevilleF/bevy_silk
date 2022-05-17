use bevy::math::Vec3;
use bevy::prelude::{Entity, GlobalTransform};

/// Defines a cloth vertex anchor through a `target` and `offset`
#[derive(Debug, Copy, Clone, Default)]
pub struct VertexAnchor {
    /// Vertex anchor target
    pub target: VertexAnchorTarget,
    /// Vertex anchor optional offset. If not set the base vertex position will be used
    pub custom_offset: Option<VertexAnchorOffset>,
}

/// Vertex anchor offset options
#[derive(Debug, Copy, Clone)]
pub enum VertexAnchorOffset {
    /// Local space offset
    Local {
        /// offset value
        offset: Vec3,
    },
    /// World space offset
    World {
        /// offset value
        offset: Vec3,
    },
}

/// Vertex anchor target options
#[derive(Debug, Copy, Clone)]
pub enum VertexAnchorTarget {
    /// `GlobalTransform` attached to the cloth entity
    SelfTransform,
    /// `GlobalTransform` attached to a custom entity.
    /// For example a bone entity for a skeletal/skinned mesh
    CustomTransform(Entity),
}

impl VertexAnchorOffset {
    #[inline]
    #[must_use]
    /// Computes the offset to retrieve the anchor world space position according to the given `transform`
    pub fn get_position(&self, transform: &GlobalTransform) -> Vec3 {
        let matrix = transform.compute_matrix();
        match self {
            VertexAnchorOffset::Local { offset } => {
                transform.translation + matrix.transform_point3(*offset)
            }
            VertexAnchorOffset::World { offset } => transform.translation + *offset,
        }
    }
}

impl VertexAnchor {
    /// Tries to get the anchor world space position.
    ///
    /// # Arguments
    ///
    /// * `self_transform` - the `GlobalTransform` associated with the cloth entity use in case of [`VertexAnchorTarget::SelfTransform`]
    /// * `transform_query` - ECS query used in case of [`VertexAnchorTarget::CustomTransform`]
    ///
    /// Can return `None` if the target transform cannot be found
    #[inline]
    #[must_use]
    pub fn get_position(
        &self,
        self_transform: &GlobalTransform,
        query: impl Fn(Entity) -> Option<GlobalTransform>,
    ) -> Option<Vec3> {
        let transform = match self.target {
            VertexAnchorTarget::SelfTransform => Some(*self_transform),
            VertexAnchorTarget::CustomTransform(entity) => query(entity),
        }?;
        Some(self.custom_offset.map_or(transform.translation, |offset| {
            offset.get_position(&transform)
        }))
    }
}

impl Default for VertexAnchorTarget {
    fn default() -> Self {
        Self::SelfTransform
    }
}
