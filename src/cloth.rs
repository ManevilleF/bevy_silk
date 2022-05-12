use crate::config::ClothConfig;
use crate::stick::{StickGeneration, StickLen};
use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_log::warn;
use bevy_math::{Mat4, Vec3};
use bevy_reflect::Reflect;
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
    /// Computes the new vertex positions of the cloth mesh
    ///
    /// # Arguments
    ///
    /// * `transform_matrix` - the transform matrix of the associated `GlobalTransform`
    #[must_use]
    pub fn compute_vertex_positions(&self, transform_matrix: &Mat4) -> Vec<Vec3> {
        let matrix = transform_matrix.inverse();

        self.current_point_positions
            .iter()
            .enumerate()
            .map(|(i, p)| {
                if self.fixed_points.contains(&i) {
                    *p
                } else {
                    matrix.transform_point3(*p)
                }
            })
            .collect()
    }

    /// Creates a new cloth from a mesh. Points positions will be directly extracted from the given vertex positions
    /// and the sticks will be extracted from the given `indices` (triangles) according to
    /// the associated [`StickGeneration`] and [`StickLen`].
    ///
    /// # Arguments
    ///
    /// * `vertex_positions` - the mesh vertex positions
    /// * `indices` - the mesh indices
    /// * `fixed_points` - the fixed vertex position indices
    /// * `stick_generation` - The stick generation mode
    /// * `stick_len` - The stick length option
    /// * `transform_matrix` - the transform matrix of the associated `GlobalTransform`
    pub fn new(
        vertex_positions: &[Vec3],
        indices: &[u32],
        fixed_points: HashSet<usize>,
        stick_generation: StickGeneration,
        stick_len: StickLen,
        transform_matrix: &Mat4,
    ) -> Self {
        // World space positions used to compute stick lengths
        let world_positions: Vec<Vec3> = vertex_positions
            .iter()
            .map(|p| transform_matrix.transform_point3(*p))
            .collect();
        // World and local positions to store in cloth
        let positions: Vec<Vec3> = vertex_positions
            .iter()
            .enumerate()
            .map(|(i, p)| {
                if fixed_points.contains(&i) {
                    *p
                } else {
                    transform_matrix.transform_point3(*p)
                }
            })
            .collect();
        let indices: Vec<usize> = indices.iter().map(|i| *i as usize).collect();
        let mut sticks = HashMap::new();

        for truple in indices.chunks_exact(3) {
            let [a, b, c] = [truple[0], truple[1], truple[2]];
            let (p_a, p_b, p_c) = (world_positions[a], world_positions[b], world_positions[c]);
            if !sticks.contains_key(&(b, a)) {
                sticks.insert((a, b), stick_len.get_stick_len(p_a, p_b));
            }
            if !sticks.contains_key(&(c, b)) {
                sticks.insert((b, c), stick_len.get_stick_len(p_b, p_c));
            }
            if let StickGeneration::Triangles = stick_generation {
                if !sticks.contains_key(&(a, c)) {
                    sticks.insert((c, a), stick_len.get_stick_len(p_c, p_a));
                }
            }
        }
        Self {
            fixed_points,
            current_point_positions: positions.clone(),
            previous_point_positions: positions,
            sticks,
        }
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
        let acceleration = (config.gravity + wind_force) * delta_time * delta_time;
        let friction = config.friction_coefficient();

        for (i, point) in self.current_point_positions.iter_mut().enumerate() {
            if !self.fixed_points.contains(&i) {
                let velocity = self
                    .previous_point_positions
                    .get(i)
                    .map_or(Vec3::ZERO, |prev| *point - *prev);
                *point += velocity * friction + acceleration;
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
            if fixed_a && fixed_b {
                continue;
            }
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
        use crate::ClothRendering;
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
            let cloth_rendering = ClothRendering::init(&mesh, false).unwrap();
            let cloth = Cloth::new(
                &cloth_rendering.vertex_positions,
                &cloth_rendering.indices,
                Default::default(),
                StickGeneration::Quads,
                StickLen::Auto,
                &matrix,
            );
            assert_eq!(cloth.current_point_positions.len(), 100 * 100);
            assert_eq!(cloth.previous_point_positions.len(), 100 * 100);
            expected_stick_len(cloth.sticks.len(), StickGeneration::Quads, (100, 100));
        }

        #[test]
        fn works_with_quads_2() {
            let mesh = rectangle_mesh((66, 42), (Vec3::X, -Vec3::Y), Vec3::Z);
            let matrix = Transform::default().compute_matrix();
            let cloth_rendering = ClothRendering::init(&mesh, false).unwrap();
            let cloth = Cloth::new(
                &cloth_rendering.vertex_positions,
                &cloth_rendering.indices,
                Default::default(),
                StickGeneration::Quads,
                StickLen::Auto,
                &matrix,
            );
            assert_eq!(cloth.current_point_positions.len(), 66 * 42);
            assert_eq!(cloth.previous_point_positions.len(), 66 * 42);
            expected_stick_len(cloth.sticks.len(), StickGeneration::Quads, (66, 42));
        }

        #[test]
        fn works_with_triangles() {
            let mesh = rectangle_mesh((100, 100), (Vec3::X, -Vec3::Y), Vec3::Z);
            let matrix = Transform::default().compute_matrix();
            let cloth_rendering = ClothRendering::init(&mesh, false).unwrap();
            let cloth = Cloth::new(
                &cloth_rendering.vertex_positions,
                &cloth_rendering.indices,
                Default::default(),
                StickGeneration::Triangles,
                StickLen::Auto,
                &matrix,
            );
            assert_eq!(cloth.current_point_positions.len(), 100 * 100);
            assert_eq!(cloth.previous_point_positions.len(), 100 * 100);
            expected_stick_len(cloth.sticks.len(), StickGeneration::Triangles, (100, 100));
        }

        #[test]
        fn works_with_triangles_2() {
            let mesh = rectangle_mesh((66, 42), (Vec3::X, -Vec3::Y), Vec3::Z);
            let matrix = Transform::default().compute_matrix();
            let cloth_rendering = ClothRendering::init(&mesh, false).unwrap();
            let cloth = Cloth::new(
                &cloth_rendering.vertex_positions,
                &cloth_rendering.indices,
                Default::default(),
                StickGeneration::Triangles,
                StickLen::Auto,
                &matrix,
            );
            assert_eq!(cloth.current_point_positions.len(), 66 * 42);
            assert_eq!(cloth.previous_point_positions.len(), 66 * 42);
            expected_stick_len(cloth.sticks.len(), StickGeneration::Triangles, (66, 42));
        }
    }
}
