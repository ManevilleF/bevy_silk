use crate::prelude::*;
use crate::vertex_anchor::VertexAnchor;
use bevy::ecs::prelude::Component;
use bevy::log::warn;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::prelude::{Color, Mesh};
use bevy::utils::HashMap;

/// Builder component for cloth behaviour, defines every available option for cloth generation and rendering.
///
/// Add this component to an entity with at least a `GlobalTransform` and a `Handle<Mesh>`
#[derive(Debug, Clone, Default, Component)]
#[must_use]
pub struct ClothBuilder {
    /// cloth vertex ids unaffected by physics and following the attached `GlobalTransform`.
    pub anchored_vertex_ids: HashMap<usize, VertexAnchor>,
    /// cloth vertex colors unaffected by physics and following the attached `GlobalTransform`.
    // TODO: convert to hashmap
    pub anchored_vertex_colors: Vec<(Color, VertexAnchor)>,
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

    /// Adds pinned points for the cloth
    ///
    /// # Arguments
    ///
    /// * `fixed_points` - Iterator on the vertex indexes that should be attached to the associated `GlobalTransform`
    #[inline]
    #[doc(hidden)]
    #[deprecated(note = "Use `with_pinned_vertex_ids` instead")]
    pub fn with_fixed_points(mut self, fixed_points: impl Iterator<Item = usize>) -> Self {
        self.anchored_vertex_ids
            .extend(fixed_points.map(|id| (id, VertexAnchor::default())));
        self
    }

    /// Adds pinned vertex ids for the cloth. The vertices will be pinned to the associated `GlobalTransform`
    ///
    /// # Arguments
    ///
    /// * `pinned_ids` - Iterator on the vertex indexes that should be pinned to the associated `GlobalTransform`
    #[inline]
    pub fn with_pinned_vertex_ids(mut self, pinned_ids: impl Iterator<Item = usize>) -> Self {
        self.anchored_vertex_ids
            .extend(pinned_ids.map(|id| (id, VertexAnchor::default())));
        self
    }

    /// Adds pinned a vertex id for the cloth. The vertex will be pinned to the associated `GlobalTransform`
    ///
    /// # Arguments
    ///
    /// * `pinned_id` - Vertex index that should be pinned to the associated `GlobalTransform`
    #[inline]
    pub fn with_pinned_vertex_id(mut self, pinned_id: usize) -> Self {
        self.anchored_vertex_ids
            .insert(pinned_id, VertexAnchor::default());
        self
    }

    /// Adds custom anchored vertex ids for the cloth
    ///
    /// # Arguments
    ///
    /// * `vertex_ids` - Iterator on the vertex indexes that should be anchored
    /// * `vertex_anchor` - Vertex anchor definition
    #[inline]
    pub fn with_anchored_vertex_ids(
        mut self,
        vertex_ids: impl Iterator<Item = usize>,
        vertex_anchor: VertexAnchor,
    ) -> Self {
        self.anchored_vertex_ids
            .extend(vertex_ids.map(|id| (id, vertex_anchor)));
        self
    }

    /// Adds a custom anchored vertex id for the cloth
    ///
    /// # Arguments
    ///
    /// * `vertex_id` - vertex index that should be anchored
    /// * `vertex_anchor` - Vertex anchor definition
    #[inline]
    pub fn with_anchored_vertex_id(
        mut self,
        vertex_id: usize,
        vertex_anchor: VertexAnchor,
    ) -> Self {
        self.anchored_vertex_ids.insert(vertex_id, vertex_anchor);
        self
    }

    /// Adds pinned vertex colors for the cloth
    ///
    /// # Arguments
    ///
    /// * `vertex_colors` - Iterator on the vertex colors that should be pinned to the associated `GlobalTransform`
    #[inline]
    pub fn with_pinned_vertex_colors(mut self, vertex_colors: impl Iterator<Item = Color>) -> Self {
        self.anchored_vertex_colors
            .extend(vertex_colors.map(|c| (c, VertexAnchor::default())));
        self
    }

    /// Adds a pinned vertex color for the cloth
    ///
    /// # Arguments
    ///
    /// * `vertex_color` - Vertex colors that should be pinned to the associated `GlobalTransform`
    #[inline]
    pub fn with_pinned_vertex_color(mut self, vertex_color: Color) -> Self {
        self.anchored_vertex_colors
            .push((vertex_color, VertexAnchor::default()));
        self
    }

    /// Adds custom anchored vertex colors the cloth
    ///
    /// # Arguments
    ///
    /// * `vertex_colors` - Iterator on the vertex colors that should be anchored
    /// * `vertex_anchor` - Vertex anchor definition
    #[inline]
    pub fn with_anchored_vertex_colors(
        mut self,
        vertex_colors: impl Iterator<Item = Color>,
        vertex_anchor: VertexAnchor,
    ) -> Self {
        self.anchored_vertex_colors
            .extend(vertex_colors.map(|c| (c, vertex_anchor)));
        self
    }

    /// Adds a custom anchored vertex color the cloth
    ///
    /// # Arguments
    ///
    /// * `vertex_color` - vertex color that should be anchored
    /// * `vertex_anchor` - Vertex anchor definition
    #[inline]
    pub fn with_anchored_vertex_color(
        mut self,
        vertex_color: Color,
        vertex_anchor: VertexAnchor,
    ) -> Self {
        self.anchored_vertex_colors
            .push((vertex_color, vertex_anchor));
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
    #[inline]
    pub fn without_normal_computation(mut self) -> Self {
        self.normals_computing = NormalComputing::None;
        self
    }

    /// The cloth will compute smooth vertex normals
    #[deprecated(note = "Use `with_smooth_normals` instead")]
    #[doc(hidden)]
    #[inline]
    pub fn with_smooth_normal_computation(mut self) -> Self {
        self.normals_computing = NormalComputing::SmoothNormals;
        self
    }

    /// The cloth will compute smooth vertex normals
    #[inline]
    pub fn with_smooth_normals(mut self) -> Self {
        self.normals_computing = NormalComputing::SmoothNormals;
        self
    }

    /// The cloth will compute flat vertex normals and duplicate shared vertices
    #[deprecated(note = "Use `with_flat_normals` instead")]
    #[doc(hidden)]
    #[inline]
    pub fn with_flat_normal_computation(mut self) -> Self {
        self.normals_computing = NormalComputing::FlatNormals;
        self
    }

    /// The cloth will compute flat vertex normals and duplicate shared vertices
    #[inline]
    pub fn with_flat_normals(mut self) -> Self {
        self.normals_computing = NormalComputing::FlatNormals;
        self
    }

    /// Retrieves all anchored vertex ids using:
    /// - [`Self::anchored_vertex_ids`] explicit ids
    /// - [`Self::anchored_vertex_colors`] to find every vertex id in `mesh` matching a pinned color
    ///
    /// Note: anchored vertex colors are ignored if the given `mesh` doesn't have vertex colors
    #[must_use]
    pub fn anchored_vertex_ids(&self, mesh: &Mesh) -> HashMap<usize, VertexAnchor> {
        let mut res = self.anchored_vertex_ids.clone();
        if !self.anchored_vertex_colors.is_empty() {
            let vertex_colors: Option<Vec<Color>> =
                mesh.attribute(Mesh::ATTRIBUTE_COLOR)
                    .and_then(|attr| match attr {
                        VertexAttributeValues::Float32x3(v) => {
                            Some(v.iter().copied().map(Color::from).collect())
                        }
                        VertexAttributeValues::Float32x4(v) => {
                            Some(v.iter().copied().map(Color::from).collect())
                        }
                        VertexAttributeValues::Uint8x4(v) => Some(
                            v.iter()
                                .map(|c| Color::rgba_u8(c[0], c[1], c[2], c[3]))
                                .collect(),
                        ),
                        _ => None,
                    });
            vertex_colors.map_or_else(|| {
                warn!("ClothBuilder has anchored vertex colors but the associated mesh doesn't have a valid Vertex_Color attribute");
            }, |colors| {
                res.extend(colors.into_iter().enumerate().filter_map(|(i, color)| {
                    self.anchored_vertex_colors
                        .iter()
                        .find(|(c, _)| *c == color)
                        .map(|(_, anchor)| (i, *anchor))
                }));
            });
        }
        res
    }
}
