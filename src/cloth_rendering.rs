use crate::Error;
use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_math::Vec3;
use bevy_reflect::Reflect;
use bevy_render::mesh::{Indices, Mesh, VertexAttributeValues};

/// Cloth rendering component. It allows mesh data extraction, vertex duplication and normal computation
#[derive(Debug, Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct ClothRendering {
    /// Mesh vertex positions
    pub vertex_positions: Vec<Vec3>,
    /// Mesh vertex indices
    pub indices: Vec<u32>,
    /// If set to true, the vertices will be duplicated and normals computed before updating the mesh
    pub compute_normals: bool,
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
    pub fn init(mesh: &Mesh, compute_normals: bool) -> Result<Self, Error> {
        let vertex_positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .ok_or_else(|| Error::MissingMeshAttribute("ATTRIBUTE_POSITION".to_string()))?;
        // Original vertex positions in local space
        let vertex_positions = match vertex_positions {
            VertexAttributeValues::Float32x3(v) => v.iter().copied().map(Vec3::from).collect(),
            _ => return Err(Error::UnsupportedVertexPositionAttribute),
        };
        let indices = match mesh.indices() {
            None => return Err(Error::MissingIndices),
            Some(i) => match i {
                Indices::U16(v) => v.iter().copied().map(u32::from).collect(),
                Indices::U32(v) => v.clone(),
            },
        };
        Ok(Self {
            vertex_positions,
            indices,
            compute_normals,
        })
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
        let (vertex_positions, indices): (Vec<_>, Vec<_>) = self
            .indices
            .iter()
            .enumerate()
            .map(|(i, indice)| (self.vertex_positions[*indice as usize], i as u32))
            .unzip();
        Self {
            vertex_positions,
            indices,
            compute_normals: self.compute_normals,
        }
    }

    /// Computes vertex normals from indices, should be called on [`Self::duplicated_self`] as it requires
    /// no shared vertices
    pub(crate) fn compute_normals(&self) -> Vec<Vec3> {
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

    /// applies the rendering data to the mesh.
    /// If [`Self::compute_normals`] is set to `true`, the vertices will be duplicated and vertex
    /// normals will be computed and applied to the mesh.
    /// Otherwise, only the vertex positions are applied.
    pub fn apply(&self, mesh: &mut Mesh) {
        if self.compute_normals {
            let new_self = self.duplicated_self();
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                new_self
                    .vertex_positions
                    .iter()
                    .map(Vec3::to_array)
                    .collect::<Vec<[f32; 3]>>(),
            );
            let vertex_normals = new_self.compute_normals();
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                vertex_normals
                    .iter()
                    .map(Vec3::to_array)
                    .collect::<Vec<[f32; 3]>>(),
            );
            mesh.set_indices(Some(Indices::U32(new_self.indices)));
        } else {
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                self.vertex_positions
                    .iter()
                    .map(Vec3::to_array)
                    .collect::<Vec<[f32; 3]>>(),
            );
        }
    }
}
