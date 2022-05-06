use bevy::math::{Mat4, Vec3};

/// A single cloth point
#[derive(Debug, Clone)]
pub enum Point {
    /// Fixed point attached to a `GlobalTransform`
    Fixed {
        /// offset from the attached `lobalTTransform`
        offset: Vec3,
    },
    /// Regular dynamic cloth point
    Dynamic {
        /// Custom 3D position of the point
        position: Vec3,
        /// Previous 3D position of the point, used to compute its velocity
        old_position: Option<Vec3>,
    },
}

impl Point {
    /// Retrieves the position of the point
    ///
    /// The `transform_matrix` is used for `Point::Fixed` variants.
    ///
    /// You can retrieve the matrix with the `GlobalTransform::compute_matrix` method.
    #[inline]
    #[must_use]
    pub fn position(&self, transform_matrix: &Mat4) -> Vec3 {
        match self {
            Point::Fixed { offset } => transform_matrix.transform_point3(*offset),
            Point::Dynamic { position, .. } => *position,
        }
    }

    #[inline]
    #[must_use]
    /// Checks if `self` is a [`Point::Fixed`]
    pub const fn is_fixed(&self) -> bool {
        matches!(self, Point::Fixed { .. })
    }
}
