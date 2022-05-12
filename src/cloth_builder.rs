use crate::prelude::*;
use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::Reflect;
use bevy_utils::HashSet;

/// Builder component for cloth behaviour, defines every available option for cloth generation and rendering.
///
/// Add this component to an entity with at least a `GlobalTransform` and a `Handle<Mesh>`
#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect(Component)]
#[must_use]
pub struct ClothBuilder {
    /// cloth points unaffected by physics and following the attached `GlobalTransform`.
    pub fixed_points: HashSet<usize>,
    /// How cloth sticks get generated
    pub stick_generation: StickGeneration,
    /// Define cloth sticks target length
    pub stick_length: StickLen,
    /// Defines the cloth computation mode of vertex normals
    pub normals_computing: NormalComputing,
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
        self.stick_generation = stick_generation;
        self
    }

    /// Sets the sticks target length option for the cloth
    ///
    /// # Arguments
    ///
    /// * `stick_len` - Cloth sticks target length option
    #[inline]
    pub fn with_stick_length(mut self, stick_len: StickLen) -> Self {
        self.stick_length = stick_len;
        self
    }

    /// The cloth won't re-compute the mesh normals. It's the fastest option but lighting will
    /// become inconsistent
    pub fn without_normal_computation(mut self) -> Self {
        self.normals_computing = NormalComputing::None;
        self
    }

    /// The cloth will compute smooth vertex normals
    pub fn with_smooth_normal_computation(mut self) -> Self {
        self.normals_computing = NormalComputing::SmoothNormals;
        self
    }

    /// The cloth will compute flat vertex normals and duplicate shared vertices
    pub fn with_flat_normal_computation(mut self) -> Self {
        self.normals_computing = NormalComputing::FlatNormals;
        self
    }
}
