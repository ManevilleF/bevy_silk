use bevy::math::Vec3;

/// A single cloth point
#[derive(Debug, Clone)]
pub struct Point {
    /// Custom 3D position of the point
    pub position: Vec3,
    /// Previous 3D position of the point, used to compute its velocity
    pub old_position: Option<Vec3>,
}

impl Point {
    /// Retrieves the previous 3D position of the point
    #[inline]
    #[must_use]
    pub fn old_position(&self) -> Vec3 {
        self.old_position.unwrap_or(self.position)
    }

    /// Computes the current point velocity
    #[inline]
    #[must_use]
    pub fn velocity(&self) -> Vec3 {
        self.position - self.old_position()
    }
}
