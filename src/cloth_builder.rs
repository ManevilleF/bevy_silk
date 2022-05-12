use crate::prelude::*;
use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::Reflect;
use bevy_utils::HashSet;

/// Builder component for cloth behaviour, defines every available option for cloth generation and rendering.
///
/// Add this component to an entity with at least a `GlobalTransform` and a `Handle<Mesh>`
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
#[must_use]
pub struct ClothBuilder {
    /// cloth points unaffected by physics and following the attached `GlobalTransform`.
    pub fixed_points: Option<HashSet<usize>>,
    /// How cloth sticks get generated
    pub stick_generation: Option<StickGeneration>,
    /// Define cloth sticks target length
    pub stick_length: Option<StickLen>,
    /// Should the cloth compute normal data.
    /// If set to true the lighting will be correct, but the rendering might be slower
    pub compute_normals: bool,
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

    /// Sets the sticks target length option for the cloth
    ///
    /// # Arguments
    ///
    /// * `stick_len` - Cloth sticks target length option
    #[inline]
    pub fn with_stick_length(mut self, stick_len: StickLen) -> Self {
        self.stick_length = Some(stick_len);
        self
    }

    /// The cloth won't re-compute the mesh normals. It's the fastest option but lighting will
    /// become inconsistent
    pub fn without_normal_computation(mut self) -> Self {
        self.compute_normals = false;
        self
    }
    /// The cloth will re-compute the mesh normals
    pub fn with_normal_computation(mut self) -> Self {
        self.compute_normals = true;
        self
    }

    /// Retrieves the cloth fixed vertex indices
    pub fn fixed_points(&self) -> HashSet<usize> {
        self.fixed_points.unwrap_or_default()
    }

    /// Retrieves the cloth stick generation mode
    pub fn stick_generation(&self) -> StickGeneration {
        self.stick_generation.unwrap_or_default()
    }

    /// Retrieves the cloth stick length option
    pub fn stick_length(&self) -> StickLen {
        self.stick_length.unwrap_or_default()
    }

    /// Should the cloth compute the normals
    pub fn compute_normals(&self) -> bool {
        self.compute_normals
    }
}

impl Default for ClothBuilder {
    fn default() -> Self {
        Self {
            fixed_points: None,
            stick_generation: None,
            stick_length: None,
            compute_normals: false,
        }
    }
}
