use crate::prelude::*;
use bevy_utils::HashSet;

/// Builder for [`Cloth`]
#[derive(Debug, Clone, Default)]
#[must_use]
pub struct ClothBuilder {
    /// cloth points unaffected by physics and following the attached `GlobalTransform`.
    pub fixed_points: HashSet<usize>,
    /// How cloth sticks get generated
    pub stick_generation: Option<StickGeneration>,
}

#[allow(clippy::missing_const_for_fn)]
impl ClothBuilder {
    /// Instantiates a new `ClothBuilder`
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets fixed points for the cloth
    ///
    /// # Arguments
    ///
    /// * `fixed_points` - Iterator on the vertex indexes that should be attached to the associated `GlobalTransform`
    #[inline]
    pub fn with_fixed_points(mut self, fixed_points: impl Iterator<Item = usize>) -> Self {
        self.fixed_points = fixed_points.collect();
        self
    }

    /// Sets the stick generation option for the cloth
    ///
    /// # Arguments
    ///
    /// * `stick_generation` - Cloth sticks generation mode
    #[inline]
    pub fn with_stick_generation(mut self, stick_generation: StickGeneration) -> Self {
        self.stick_generation = Some(stick_generation);
        self
    }

    /// Consumes the builder and builds a new [`Cloth`] components
    #[inline]
    pub fn build(self) -> Cloth {
        Cloth {
            fixed_points: self.fixed_points,
            stick_generation: self.stick_generation.unwrap_or_default(),
            ..Default::default()
        }
    }
}
