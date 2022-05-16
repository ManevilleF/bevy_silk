use crate::Error;
use bevy::ecs::prelude::Component;
use bevy::math::{const_vec3, Vec3};
use bevy::reflect::Reflect;
use bevy::render::color::Color;
use bevy::render::mesh::{Indices, Mesh, VertexAttributeValues};
use bevy::utils::HashMap;

/// Defines the cloth computation mode of vertex normals
#[derive(Debug, Copy, Clone, Reflect)]
pub enum NormalComputing {
    /// The cloth won't compute any vertex normals, leaving the original ones
    None,
    ///
    SmoothNormals,
    /// The cloth will duplicate the vertex positions, avoiding shared vertices, and compute
    /// flat vertex normals
    FlatNormals,
}

impl Default for NormalComputing {
    fn default() -> Self {
        Self::SmoothNormals
    }
}

/// Cloth rendering component. It allows mesh data extraction, vertex duplication and normal computation
#[derive(Debug, Clone, Component, Default)]
pub struct ClothRendering {
    /// Mesh vertex positions
    pub vertex_positions: Vec<Vec3>,
    /// Mesh vertex UV positions
    pub vertex_uvs: Option<Vec<[f32; 2]>>,
    /// Mesh vertex colors
    pub vertex_colors: Option<Vec<[f32; 4]>>,
    /// Mesh vertex indices
    pub indices: Vec<u32>,
    /// If set to true, the vertices will be duplicated and normals computed before updating the mesh
    pub normal_computing: NormalComputing,
}

impl ClothRendering {
    fn face_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        (b - a).cross(c - a).normalize() // TODO: enable default value
    }

    /// Initializes from mesh data.
    ///
    /// # Arguments
    ///
    /// * `mesh` - the mesh containing the desired data
    ///
    /// # Errors
    ///
    /// The function fails in the event of the mesh `ATTRIBUTE_POSITION` attribute is missing or invalid.
    /// It may also fail if the mesh doesn't have indices.
    pub fn init(mesh: &Mesh, normal_computing: NormalComputing) -> Result<Self, Error> {
        let vertex_positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .ok_or_else(|| Error::MissingMeshAttribute("Vertex_Position".to_string()))?;
        // Vertex positions
        let vertex_positions: Vec<Vec3> = match vertex_positions {
            VertexAttributeValues::Float32x3(v) => v.iter().copied().map(Vec3::from).collect(),
            _ => return Err(Error::UnsupportedVertexPositionAttribute),
        };
        let vertex_count = vertex_positions.len();
        // UVs
        let vertex_uvs = mesh
            .attribute(Mesh::ATTRIBUTE_UV_0)
            .and_then(|attr| match attr {
                VertexAttributeValues::Float32x2(v) => Some(v.clone()),
                _ => None,
            });
        // Assertion
        let attr_count = vertex_uvs.as_ref().map_or(vertex_count, Vec::len);
        if attr_count != vertex_count {
            return Err(Error::InvalidMeshAttribute {
                attribute: "Vertex_Uv".to_string(),
                message: format!("Expected {vertex_count} values, got {attr_count}"),
            });
        }
        // Colors
        let vertex_colors = mesh
            .attribute(Mesh::ATTRIBUTE_COLOR)
            .and_then(|attr| match attr {
                VertexAttributeValues::Float32x4(v) => Some(v.clone()),
                VertexAttributeValues::Float32x3(v) => {
                    Some(v.iter().copied().map(|[r, g, b]| [r, g, b, 1.0]).collect())
                }
                VertexAttributeValues::Uint8x4(v) => Some(
                    v.iter()
                        .copied()
                        .map(|[r, g, b, a]| Color::rgba_u8(r, g, b, a).as_rgba_f32())
                        .collect(),
                ),
                _ => None,
            });
        // Assertion
        let attr_count = vertex_colors.as_ref().map_or(vertex_count, Vec::len);
        if attr_count != vertex_count {
            return Err(Error::InvalidMeshAttribute {
                attribute: "Vertex_Color".to_string(),
                message: format!("Expected {vertex_count} values, got {attr_count}"),
            });
        }

        let indices = match mesh.indices() {
            None => return Err(Error::MissingIndices),
            Some(i) => match i {
                Indices::U16(v) => v.iter().copied().map(u32::from).collect(),
                Indices::U32(v) => v.clone(),
            },
        };
        Ok(Self {
            vertex_positions,
            vertex_uvs,
            vertex_colors,
            indices,
            normal_computing,
        })
    }

    ///  Compute the Axis-Aligned Bounding Box of the mesh vertices in model space
    ///
    /// # Returns
    ///
    /// A tuple value with:
    /// - 0: The center of the bounding box
    /// - 1: The half extents of the bounding box
    #[must_use]
    pub fn compute_aabb(&self, offset: Option<f32>) -> (Vec3, Vec3) {
        const VEC3_MIN: Vec3 = const_vec3!([std::f32::MIN, std::f32::MIN, std::f32::MIN]);
        const VEC3_MAX: Vec3 = const_vec3!([std::f32::MAX, std::f32::MAX, std::f32::MAX]);

        let mut minimum = VEC3_MAX;
        let mut maximum = VEC3_MIN;
        for p in &self.vertex_positions {
            minimum = minimum.min(*p);
            maximum = maximum.max(*p);
        }
        let center = 0.5 * (maximum + minimum);
        let half_extents = 0.5 * (maximum - minimum) + offset.unwrap_or(0.0);
        (center, half_extents)
    }

    /// Updates the vertex positions from the cloth point values
    ///
    /// # Panics
    ///
    /// Panics if the new `vertex_positions` doesn't have the same length as the previous vertices
    pub fn update_positions(&mut self, vertex_positions: Vec<Vec3>) {
        assert_eq!(vertex_positions.len(), self.vertex_positions.len());
        self.vertex_positions = vertex_positions;
    }

    /// Duplicates `self` by computing one vertex position per indice.
    /// This allows to remove shared vertices and compute normals.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn duplicated_self(&self) -> Self {
        let ((vertex_positions, indices), (vertex_uvs, vertex_colors)): ((_, _), (Vec<_>, Vec<_>)) =
            self.indices
                .iter()
                .enumerate()
                .map(|(i, indice)| {
                    (
                        (self.vertex_positions[*indice as usize], i as u32),
                        (
                            self.vertex_uvs.as_ref().map(|v| v[*indice as usize]),
                            self.vertex_colors.as_ref().map(|v| v[*indice as usize]),
                        ),
                    )
                })
                .unzip();
        Self {
            vertex_positions,
            indices,
            normal_computing: self.normal_computing,
            vertex_uvs: vertex_uvs.into_iter().collect(),
            vertex_colors: vertex_colors.into_iter().collect(),
        }
    }

    /// Computes vertex normals from indices, should be called on [`Self::duplicated_self`] as it requires
    /// no shared vertices
    pub(crate) fn compute_flat_normals(&self) -> Vec<Vec3> {
        self.indices
            .chunks_exact(3)
            .flat_map(|chunk| {
                let [a, b, c] =
                    [chunk[0], chunk[1], chunk[2]].map(|i| self.vertex_positions[i as usize]);
                let normal = Self::face_normal(a, b, c);
                [normal; 3]
            })
            .collect()
    }

    /// Computes averaged vertex normals from indices, should be called without duplication as it requires shared vertices
    #[allow(clippy::cast_precision_loss)]
    pub(crate) fn compute_smooth_normals(&self) -> Vec<Vec3> {
        let mut map = HashMap::with_capacity(self.vertex_positions.len());
        for chunk in self.indices.chunks_exact(3) {
            let [a, b, c] = [chunk[0] as usize, chunk[1] as usize, chunk[2] as usize];
            let flat_normal = Self::face_normal(
                self.vertex_positions[a],
                self.vertex_positions[b],
                self.vertex_positions[c],
            );
            map.entry(a).or_insert(vec![]).push(flat_normal);
            map.entry(b).or_insert(vec![]).push(flat_normal);
            map.entry(c).or_insert(vec![]).push(flat_normal);
        }
        (0..self.vertex_positions.len())
            .map(|i| {
                let sum = map[&i].iter().fold(Vec3::ZERO, |res, v| res + *v);
                sum / map[&i].len() as f32
            })
            .collect()
    }

    fn vec3_vertex_attr(attr: &[Vec3]) -> Vec<[f32; 3]> {
        attr.iter().map(Vec3::to_array).collect()
    }

    /// applies the rendering data to the mesh.
    ///
    /// If [`Self::normal_computing`] is set to [`NormalComputing::FlatNormals`], the vertices will
    /// be first be duplicated before the normals are computed
    pub fn apply(&self, mesh: &mut Mesh) {
        match self.normal_computing {
            NormalComputing::None => mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                Self::vec3_vertex_attr(&self.vertex_positions),
            ),
            NormalComputing::SmoothNormals => {
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    Self::vec3_vertex_attr(&self.vertex_positions),
                );
                let vertex_normals = self.compute_smooth_normals();
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_NORMAL,
                    Self::vec3_vertex_attr(&vertex_normals),
                );
            }
            NormalComputing::FlatNormals => {
                let new_self = self.duplicated_self();
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    Self::vec3_vertex_attr(&new_self.vertex_positions),
                );
                if let Some(ref attr) = new_self.vertex_uvs {
                    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, attr.clone());
                }
                if let Some(ref attr) = new_self.vertex_colors {
                    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, attr.clone());
                }
                let vertex_normals = new_self.compute_flat_normals();
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_NORMAL,
                    Self::vec3_vertex_attr(&vertex_normals),
                );
                mesh.set_indices(Some(Indices::U32(new_self.indices)));
            }
        }
    }
}
