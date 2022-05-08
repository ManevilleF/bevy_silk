use bevy::math::Vec3;

/// Cloth physics configuration resource
#[derive(Debug, Clone)]
pub struct ClothConfig {
    /// Custom gravity, classic (0, -9.81, 0) is used by default
    pub gravity: Vec3,
    /// Custom friction to apply to velocity.
    ///
    /// Note: will be clamped between 0.0 and 1.0
    pub friction: f32,
    /// Sets the number of sticks computation iteration.
    /// The higher the value, the more precision and less elasticity for the sticks but the cost is increased
    pub sticks_computation_depth: u8,
}

impl ClothConfig {
    /// Default Y value for gravity
    pub const DEFAULT_GRAVITY: f32 = -9.81;

    #[must_use]
    #[inline]
    pub(crate) fn friction_coefficient(&self) -> f32 {
        1.0 - self.friction.clamp(0.0, 1.0)
    }
}

impl Default for ClothConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::Y * Self::DEFAULT_GRAVITY,
            friction: 0.02,
            sticks_computation_depth: 4,
        }
    }
}
