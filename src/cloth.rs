use crate::config::ClothConfig;
use crate::stick::Stick;
use bevy::ecs::component::Component;
use bevy::log;
use bevy::math::Vec3;
use bevy::prelude::{GlobalTransform, Mesh};
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy::utils::HashMap;

/// Cloth component
#[derive(Debug, Clone, Component, Default)]
#[must_use]
pub struct Cloth {
    /// cloth points unaffected by physics and following the attached `GlobalTransform`.
    ///
    /// The key is the index of the vertex position, the value is the translation relative to the `GlobalTransform`
    pub fixed_points: HashMap<usize, Vec3>,
    /// Old Cloth points 3D positions
    previous_point_positions: Vec<Vec3>,
    /// Cloth sticks linking points
    sticks: Vec<Stick>,
    // TODO: enable max tension
    // /// Optional maximum stick tension.
    // ///
    // /// If set, the sticks will break under too much tension with the value as threshold.
    // pub max_tension: Option<f32>,
}

impl Cloth {
    #[inline]
    #[must_use]
    pub fn new(fixed_points: impl Iterator<Item = (usize, Vec3)>) -> Self {
        Self {
            fixed_points: fixed_points.collect(),
            previous_point_positions: vec![],
            sticks: vec![],
        }
    }

    #[inline]
    #[must_use]
    pub fn is_setup(&self) -> bool {
        !self.sticks.is_empty()
    }

    pub fn compute_mesh(
        &mut self,
        mesh: &mut Mesh,
        transform: &GlobalTransform,
        config: &ClothConfig,
        delta_time: f32,
    ) {
        let vertex_positions = mesh
            .attribute_mut(Mesh::ATTRIBUTE_POSITION)
            .expect("Mesh associated to cloth doesn't have `ATTRIBUTE_POSITION` set");
        let positions = match vertex_positions {
            VertexAttributeValues::Float32x3(v) => v,
            _ => {
                panic!("Unsupported vertex position attribute, only `Float32x3` is supported");
            }
        };
        // TODO: benchmark this and test with a cloned array and attribute insertion
        self.update_points(positions, delta_time, config, transform);
        self.update_sticks(positions, config);
    }

    pub fn setup_sticks(&mut self, mesh: &Mesh) {
        let indices: Vec<usize> = match mesh.indices() {
            None => {
                log::error!("Mesh associated to cloth doesn't have indices set");
                return;
            }
            Some(i) => match i {
                Indices::U16(v) => v.into_iter().map(|i| *i as usize).collect(),
                Indices::U32(v) => v.into_iter().map(|i| *i as usize).collect(),
            },
        };
        let vertex_positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Mesh associated to cloth doesn't have `ATTRIBUTE_POSITION` set");
        let positions: Vec<Vec3> = match vertex_positions {
            VertexAttributeValues::Float32x3(v) => v.iter().copied().map(Vec3::from).collect(),
            _ => {
                panic!("Unsupported vertex position attribute, only `Float32x3` is supported");
            }
        };
        let sticks = indices
            .chunks_exact(3)
            .flat_map(|truple| {
                let [a, b, c] = [truple[0], truple[1], truple[2]];
                let (p_a, p_b, p_c) = (positions[a], positions[b], positions[c]);
                vec![
                    Stick {
                        point_a_index: a,
                        point_b_index: b,
                        length: p_a.distance(p_b),
                    },
                    Stick {
                        point_a_index: b,
                        point_b_index: c,
                        length: p_b.distance(p_c),
                    },
                    Stick {
                        point_a_index: c,
                        point_b_index: a,
                        length: p_c.distance(p_a),
                    },
                ]
            })
            .collect();
        self.sticks = sticks;
    }

    fn update_points(
        &mut self,
        positions: &mut Vec<[f32; 3]>,
        delta_time: f32,
        config: &ClothConfig,
        transform: &GlobalTransform,
    ) {
        let gravity = config.gravity * delta_time * delta_time;
        let friction = config.friction_coefficient();
        let matrix = transform.compute_matrix();

        for (i, point) in positions.iter_mut().enumerate() {
            let mut point_vec3 = Vec3::from(*point);
            let old_position = match self.previous_point_positions.get(i).copied() {
                None => {
                    self.previous_point_positions.push(point_vec3);
                    point_vec3
                }
                Some(p) => p,
            };
            if let Some(translation) = self.fixed_points.get(&i) {
                *point = matrix.transform_point3(*translation).to_array();
            } else {
                let velocity = point_vec3 - old_position;
                self.previous_point_positions[i] = point_vec3;
                point_vec3 += velocity * friction * delta_time + gravity;
                *point = point_vec3.to_array();
            }
        }
    }

    fn update_sticks(&mut self, positions: &mut Vec<[f32; 3]>, config: &ClothConfig) {
        for _depth in 0..config.sticks_computation_depth {
            for stick in &self.sticks {
                let (position_a, fixed_a) = match positions.get(stick.point_a_index) {
                    None => {
                        log::warn!(
                            "Failed to retrieve a Cloth point at index {}",
                            stick.point_a_index
                        );
                        continue;
                    }
                    Some(p) => (
                        Vec3::from(*p),
                        self.fixed_points.contains_key(&stick.point_a_index),
                    ),
                };
                let (position_b, fixed_b) = match positions.get(stick.point_b_index) {
                    None => {
                        log::warn!(
                            "Failed to retrieve a Cloth point at index {}",
                            stick.point_b_index
                        );
                        continue;
                    }
                    Some(p) => (
                        Vec3::from(*p),
                        self.fixed_points.contains_key(&stick.point_b_index),
                    ),
                };
                let target_len = if fixed_a == fixed_b {
                    stick.length / 2.0
                } else {
                    stick.length
                };
                let center = (position_b + position_a) / 2.0;
                let direction = match (position_b - position_a).try_normalize() {
                    None => {
                        log::warn!("Failed handle stick between points {} and {} which are too close to each other", stick.point_a_index, stick.point_b_index);
                        continue;
                    }
                    Some(dir) => dir * target_len,
                };
                if !fixed_a {
                    let position = positions.get_mut(stick.point_a_index).unwrap();
                    *position = (center + direction).to_array();
                }
                if !fixed_b {
                    let position = positions.get_mut(stick.point_b_index).unwrap();
                    *position = (center - direction).to_array();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod rectangle {
        use super::*;

        #[test]
        fn correct_rectangle() {
            let cloth = Cloth::rectangle(10, 10, 10.0);
            assert_eq!(cloth.points.len(), 100);
            assert_eq!(cloth.sticks.len(), 180);
        }
    }
}
