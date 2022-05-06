use crate::config::ClothConfig;
use crate::point::Point;
use crate::stick::Stick;
use bevy::ecs::component::Component;
use bevy::log;
use bevy::math::Vec3;
use bevy::prelude::GlobalTransform;

/// Cloth component
#[derive(Debug, Clone, Component)]
#[must_use]
pub struct Cloth {
    /// Cloth points
    pub points: Vec<Point>,
    /// Cloth sticks linking points
    pub sticks: Vec<Stick>,
    /// Optional maximum stick tension.
    ///
    /// If set, the sticks will break under too much tension with the value as threshold.
    pub max_tension: Option<f32>,
}

impl Cloth {
    #[allow(clippy::cast_precision_loss)]
    /// Creates a cloth component as a rectangle
    ///
    /// # Params
    ///
    /// * `size_x` - the size of the cloth in the X axis (should be above 1)
    /// * `size_z` - the size of the cloth in the Z axis (should be above 1)
    pub fn rectangle(size_x: usize, size_z: usize, step: f32) -> Self {
        let points = (0..size_z)
            .flat_map(|z| {
                (0..size_x).map(move |x| Point::Dynamic {
                    position: Vec3::new(x as f32 * step, 0.0, z as f32 * step),
                    old_position: None,
                })
            })
            .collect();
        let mut sticks = Vec::with_capacity(
            size_x * size_z.saturating_sub(1) + size_x.saturating_sub(1) * size_z,
        );
        for i in 1..(size_x * size_z) {
            if let Some(above) = i.checked_sub(size_x) {
                sticks.push(Stick {
                    point_a_index: above,
                    point_b_index: i,
                    length: step,
                });
            }
            if i % size_x != 0 {
                sticks.push(Stick {
                    point_a_index: i - 1,
                    point_b_index: i,
                    length: step,
                });
            }
        }
        Self {
            points,
            sticks,
            max_tension: None,
        }
    }

    pub(crate) fn update_points(&mut self, delta_time: f32, config: &ClothConfig) {
        let gravity = config.gravity * delta_time;
        let friction = config.friction_coefficient();

        for point in &mut self.points {
            if let Point::Dynamic {
                position,
                old_position,
            } = point
            {
                let velocity = old_position.map_or(Vec3::ZERO, |old| *position - old);
                *old_position = Some(*position);
                *position += velocity * friction * delta_time + gravity;
            }
        }
    }

    pub(crate) fn update_sticks(&mut self, config: &ClothConfig, transform: &GlobalTransform) {
        let matrix = transform.compute_matrix();
        for _depth in 0..config.sticks_computation_depth {
            for stick in &self.sticks {
                let (position_a, fixed_a) = match self.points.get(stick.point_a_index) {
                    None => {
                        log::warn!(
                            "Failed to retrieve a Cloth point at index {}",
                            stick.point_a_index
                        );
                        continue;
                    }
                    Some(p) => (p.position(&matrix), p.is_fixed()),
                };
                let (position_b, fixed_b) = match self.points.get(stick.point_b_index) {
                    None => {
                        log::warn!(
                            "Failed to retrieve a Cloth point at index {}",
                            stick.point_b_index
                        );
                        continue;
                    }
                    Some(p) => (p.position(&matrix), p.is_fixed()),
                };
                if let Some(max_tension) = self.max_tension {
                    let distance = position_a.distance(position_b);
                    if distance > stick.length * max_tension {
                        // TODO: destroy stick
                    }
                }
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
                if let Some(Point::Dynamic {
                    position,
                    old_position,
                }) = self.points.get_mut(stick.point_a_index)
                {
                    *old_position = Some(*position);
                    *position = center + direction;
                }
                if let Some(Point::Dynamic {
                    position,
                    old_position,
                }) = self.points.get_mut(stick.point_b_index)
                {
                    *old_position = Some(*position);
                    *position = center - direction;
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
