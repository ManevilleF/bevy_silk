use crate::components::cloth::StickId;
use crate::prelude::{StickMode, VertexAnchor};
use bevy::ecs::prelude::Component;
use bevy::math::Vec3;
use bevy::reflect::Reflect;

/// Cloth inflator center selection mode
#[derive(Debug, Default, Copy, Clone, Reflect)]
pub enum InflatorCenter {
    #[default]
    /// The center of the mesh bounding box will be selected
    Aabb,
    /// A custom local center
    Custom(Vec3),
}

/// Cloth Inflator component. Allows to simulate inflating a cloth mesh.
///
/// This component will add an extra *centroid* point to the cloth and as many spring
/// sticks as there are points (vertice) on the cloth.
/// Pleas not that this is not an actual soft body simulation, but a very approximated
/// version using the verlet integration engine
#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct ClothInflator {
    /// Target minimal inflating amount
    /// - `0.0` would mean no inflating
    /// - `1.0` would mean *full* inflating
    pub inflating_percent: f32,
    /// The inflating center. This will be applied only once
    pub(crate) center: InflatorCenter,
    /// Optional anchor for the added center point
    pub(crate) anchor: Option<VertexAnchor>,
    /// The cloth sticks behaviours to edit
    #[reflect(ignore)]
    pub(crate) sticks: Vec<StickId>,
}

impl ClothInflator {
    /// Instantiates a new inflator with the given `min_volume`
    #[inline]
    #[must_use]
    pub const fn new(inflating_percent: f32) -> Self {
        Self {
            inflating_percent,
            center: InflatorCenter::Aabb,
            anchor: None,
            sticks: vec![],
        }
    }

    /// Defines a custom local space center instead of the default Aabb center
    #[inline]
    #[must_use]
    pub const fn with_custom_center(mut self, center: Vec3) -> Self {
        self.center = InflatorCenter::Custom(center);
        self
    }

    /// Defines a vertex anchor for the new cloth point
    #[inline]
    #[must_use]
    pub const fn with_anchor(mut self, anchor: VertexAnchor) -> Self {
        self.anchor = Some(anchor);
        self
    }

    #[inline]
    #[must_use]
    /// Computes the stick behaviour mode of the inflator
    pub(crate) fn stick_mode(&self) -> StickMode {
        StickMode::Spring {
            min_percent: self.inflating_percent.max(0.1),
            max_percent: f32::MAX,
        }
    }
}
