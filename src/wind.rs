use bevy_math::Vec3;
use bevy_reflect::Reflect;

/// Wind definition for cloth physics
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Reflect)]
pub enum Wind {
    /// Constant Wind force
    ConstantWind {
        /// Wind velocity
        velocity: Vec3,
    },
    /// Wind force following a sin wave
    SinWave {
        /// Wind velocity at the top of the sin wave
        max_velocity: Vec3,
        /// sin wave frequency
        frequency: f32,
    },
}

impl Default for Wind {
    fn default() -> Self {
        Self::SinWave {
            max_velocity: Vec3::X,
            frequency: 0.5,
        }
    }
}

impl Wind {
    /// Retrieves the current wind velocity according to the elapsed time since startup
    #[must_use]
    pub fn current_velocity(&self, elapsed_time: f32) -> Vec3 {
        match self {
            Wind::ConstantWind { velocity } => *velocity,
            Wind::SinWave {
                max_velocity,
                frequency,
            } => ((elapsed_time * frequency).sin() + 1.0) / 2.0 * *max_velocity,
        }
    }
}
