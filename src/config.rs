use bevy::math::Vec3;

/// Cloth physics configuration
#[derive(Debug, Clone)]
pub struct ClothConfig {
    /// Custom gravity, classic (0, -9.81, 0) is used by default
    pub gravity: Vec3,
    /// Custom friction to apply to velocity, 0.01 by default
    pub friction: f32,
    /// Sets the number of sticks computation iteration.
    /// The higher the value, the more precision and less elasticity for the sticks but the cost is increased
    pub sticks_computation_depth: u8,
}

impl ClothConfig {
    /// Default Y value for gravity
    pub const DEFAULT_GRAVITY: f32 = -9.81;
}

impl Default for ClothConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::Y * Self::DEFAULT_GRAVITY,
            friction: 0.01,
            sticks_computation_depth: 2,
        }
    }
}
