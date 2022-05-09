use crate::config::ClothConfig;
use crate::stick::StickGeneration;
use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_log::{error, warn};
use bevy_math::{Mat4, Vec3};
use bevy_reflect::Reflect;
use bevy_render::mesh::{Indices, Mesh, VertexAttributeValues};
use bevy_utils::{HashMap, HashSet};

macro_rules! get_point {
    ($id:expr, $points:expr, $fixed_points:expr, $matrix:expr) => {
        match $points.get($id) {
            None => {
                warn!("Failed to retrieve a Cloth point at index {}", $id);
                continue;
            }
            Some(p) => {
                if $fixed_points.contains(&$id) {
                    ($matrix.transform_point3(*p), true)
                } else {
                    (*p, false)
                }
            }
        }
    };
}

/// Cloth component
#[derive(Debug, Clone, Component, Default, Reflect)]
#[reflect(Component)]
#[must_use]
pub struct Cloth {
    /// cloth points unaffected by physics and following the attached `GlobalTransform`.
    pub fixed_points: HashSet<usize>,
    /// How cloth sticks get generated
    pub stick_generation: StickGeneration,
    /// Current Cloth points 3D positions in world space
    ///
    /// Note: this field will be automatically populated from mesh data
    pub current_point_positions: Vec<Vec3>,
    /// Old Cloth points 3D positions in world space
    ///
    /// Note: this field will be automatically populated from mesh data
    pub previous_point_positions: Vec<Vec3>,
    /// Cloth sticks linking points
    ///
    /// * key: tuple with the connected points indexes
    /// * value: the target distance between the points
    ///
    /// Note: this field will be automatically populated from mesh data
    pub sticks: HashMap<(usize, usize), f32>,
}

impl Cloth {
    /// Checks if the cloth initialized from mesh data
    #[inline]
    #[must_use]
    pub fn is_setup(&self) -> bool {
        !self.current_point_positions.is_empty()
    }

    /// Applies the cloth data to a mesh
    ///
    /// # Arguments
    ///
    /// * `mesh` - the mesh to edit
    /// * `transform_matrix` - the transform matrix of the associated `GlobalTransform`
    pub fn apply_to_mesh(&self, mesh: &mut Mesh, transform_matrix: &Mat4) {
        let matrix = transform_matrix.inverse();

        let positions: Vec<[f32; 3]> = self
            .current_point_positions
            .iter()
            .enumerate()
            .map(|(i, p)| {
                if self.fixed_points.contains(&i) {
                    p.to_array()
                } else {
                    matrix.transform_point3(*p).to_array()
                }
            })
            .collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    }

    /// Initializes the cloth from a mesh. Points positions will be extracted from the mesh vertex positions
    /// (`ATTRIBUTE_POSITION`) and the sticks will be extracted from the `indices` (triangles) according to
    /// the associated [`StickGeneration`] mode.
    ///
    /// # Arguments
    ///
    /// * `mesh` - the mesh containing the desired data
    /// * `transform_matrix` - the transform matrix of the associated `GlobalTransform`
    ///
    /// # Panics
    ///
    /// The function may panic in the event of the mesh `ATTRIBUTE_POSITION` attribute being invalid
    pub fn init_from_mesh(&mut self, mesh: &Mesh, transform_matrix: &Mat4) {
        let vertex_positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Mesh associated to cloth doesn't have `ATTRIBUTE_POSITION` set");
        let positions: Vec<Vec3> = match vertex_positions {
            VertexAttributeValues::Float32x3(v) => v
                .iter()
                .map(|p| transform_matrix.transform_point3(Vec3::from(*p)))
                .collect(),
            _ => {
                panic!("Unsupported vertex position attribute, only `Float32x3` is supported");
            }
        };
        let indices: Vec<usize> = match mesh.indices() {
            None => {
                error!("Mesh associated to cloth doesn't have indices set");
                return;
            }
            Some(i) => match i {
                Indices::U16(v) => v.iter().map(|i| *i as usize).collect(),
                Indices::U32(v) => v.iter().map(|i| *i as usize).collect(),
            },
        };
        let mut sticks = HashMap::new();

        for truple in indices.chunks_exact(3) {
            let [a, b, c] = [truple[0], truple[1], truple[2]];
            let (p_a, p_b, p_c) = (positions[a], positions[b], positions[c]);
            if !sticks.contains_key(&(b, a)) {
                sticks.insert((a, b), p_a.distance(p_b));
            }
            if !sticks.contains_key(&(c, b)) {
                sticks.insert((b, c), p_b.distance(p_c));
            }
            if let StickGeneration::Triangles = self.stick_generation {
                if !sticks.contains_key(&(a, c)) {
                    sticks.insert((c, a), p_c.distance(p_a));
                }
            }
        }
        self.sticks = sticks;
        self.previous_point_positions = positions.clone();
        self.current_point_positions = positions;
    }

    /// Updates the cloth
    ///
    /// # Arguments
    ///
    /// * `config` - the current configuration for the cloth physics
    /// * `delta_time` - the time since last update
    /// * `transform_matrix` - the transform matrix of the associated `GlobalTransform`
    pub fn update(
        &mut self,
        config: &ClothConfig,
        delta_time: f32,
        transform_matrix: &Mat4,
        wind_force: Vec3,
    ) {
        let position_cache = self.current_point_positions.clone();
        self.update_points(delta_time, config, wind_force);
        for _depth in 0..config.sticks_computation_depth {
            self.update_sticks(transform_matrix);
        }
        self.previous_point_positions = position_cache;
    }

    fn update_points(&mut self, delta_time: f32, config: &ClothConfig, wind_force: Vec3) {
        let gravity = config.gravity * delta_time;
        let friction = config.friction_coefficient();

        for (i, point) in self.current_point_positions.iter_mut().enumerate() {
            if !self.fixed_points.contains(&i) {
                let velocity = self
                    .previous_point_positions
                    .get(i)
                    .map_or(Vec3::ZERO, |prev| *point - *prev)
                    + wind_force;
                *point += velocity * friction * delta_time + gravity;
            }
        }
    }

    fn update_sticks(&mut self, matrix: &Mat4) {
        for ((id_a, id_b), target_len) in &self.sticks {
            let (position_a, fixed_a) = get_point!(
                *id_a,
                self.current_point_positions,
                self.fixed_points,
                matrix
            );
            let (position_b, fixed_b) = get_point!(
                *id_b,
                self.current_point_positions,
                self.fixed_points,
                matrix
            );
            let center = (position_b + position_a) / 2.0;
            let direction = match (position_b - position_a).try_normalize() {
                None => {
                    warn!("Failed handle stick between points {} and {} which are too close to each other", *id_a, *id_b);
                    continue;
                }
                Some(dir) => dir * *target_len / 2.0,
            };
            if !fixed_a {
                self.current_point_positions[*id_a] = if fixed_b {
                    position_b - direction * 2.0
                } else {
                    center - direction
                };
            }
            if !fixed_b {
                self.current_point_positions[*id_b] = if fixed_a {
                    position_a + direction * 2.0
                } else {
                    center + direction
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::rectangle_mesh;

    mod init_from_mesh {
        use super::*;
        use crate::cloth_builder::ClothBuilder;
        use bevy_transform::prelude::Transform;

        fn expected_stick_len(
            len: usize,
            generation: StickGeneration,
            (size_x, size_y): (usize, usize),
        ) {
            match generation {
                StickGeneration::Quads => {
                    assert_eq!(len, (size_x - 1) * size_y + (size_y - 1) * size_x);
                }
                StickGeneration::Triangles => {
                    assert_eq!(
                        len,
                        (size_x - 1) * size_y + (size_y - 1) * size_x + (size_x - 1) * (size_y - 1)
                    );
                }
            }
        }

        #[test]
        fn works_with_quads() {
            let mesh = rectangle_mesh((100, 100), (Vec3::X, -Vec3::Y), Vec3::Z);
            let matrix = Transform::default().compute_matrix();
            let mut cloth = ClothBuilder::new()
                .with_stick_generation(StickGeneration::Quads)
                .build(); // QUADS
            cloth.init_from_mesh(&mesh, &matrix);
            assert_eq!(cloth.current_point_positions.len(), 100 * 100);
            assert_eq!(cloth.previous_point_positions.len(), 100 * 100);
            expected_stick_len(cloth.sticks.len(), cloth.stick_generation, (100, 100));
        }

        #[test]
        fn works_with_quads_2() {
            let mesh = rectangle_mesh((66, 42), (Vec3::X, -Vec3::Y), Vec3::Z);
            let matrix = Transform::default().compute_matrix();
            let mut cloth = ClothBuilder::new()
                .with_stick_generation(StickGeneration::Quads)
                .build(); // QUADS
            cloth.init_from_mesh(&mesh, &matrix);
            assert_eq!(cloth.current_point_positions.len(), 66 * 42);
            assert_eq!(cloth.previous_point_positions.len(), 66 * 42);
            expected_stick_len(cloth.sticks.len(), cloth.stick_generation, (66, 42));
        }

        #[test]
        fn works_with_triangles() {
            let mesh = rectangle_mesh((100, 100), (Vec3::X, -Vec3::Y), Vec3::Z);
            let matrix = Transform::default().compute_matrix();
            let mut cloth = ClothBuilder::new()
                .with_stick_generation(StickGeneration::Triangles)
                .build(); // TRIANGLES
            cloth.init_from_mesh(&mesh, &matrix);
            assert_eq!(cloth.current_point_positions.len(), 100 * 100);
            assert_eq!(cloth.previous_point_positions.len(), 100 * 100);
            expected_stick_len(cloth.sticks.len(), cloth.stick_generation, (100, 100));
        }

        #[test]
        fn works_with_triangles_2() {
            let mesh = rectangle_mesh((66, 42), (Vec3::X, -Vec3::Y), Vec3::Z);
            let matrix = Transform::default().compute_matrix();
            let mut cloth = ClothBuilder::new()
                .with_stick_generation(StickGeneration::Triangles)
                .build(); // TRIANGLES
            cloth.init_from_mesh(&mesh, &matrix);
            assert_eq!(cloth.current_point_positions.len(), 66 * 42);
            assert_eq!(cloth.previous_point_positions.len(), 66 * 42);
            expected_stick_len(cloth.sticks.len(), cloth.stick_generation, (66, 42));
        }
    }
}
