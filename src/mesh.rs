// TODO: mesh generation utils

// #[allow(clippy::cast_precision_loss)]
// /// Creates a cloth component as a rectangle
// ///
// /// # Params
// ///
// /// * `size_x` - the size of the cloth in the X axis (should be above 1)
// /// * `size_z` - the size of the cloth in the Z axis (should be above 1)
// pub fn rectangle(size_x: usize, size_z: usize, step: f32) -> Self {
//     let points = (0..size_z)
//         .flat_map(|z| {
//             (0..size_x).map(move |x| Point::Dynamic {
//                 position: Vec3::new(x as f32 * step, 0.0, z as f32 * step),
//                 old_position: None,
//             })
//         })
//         .collect();
//     let mut sticks = Vec::with_capacity(
//         size_x * size_z.saturating_sub(1) + size_x.saturating_sub(1) * size_z,
//     );
//     for i in 1..(size_x * size_z) {
//         if let Some(above) = i.checked_sub(size_x) {
//             sticks.push(Stick {
//                 point_a_index: above,
//                 point_b_index: i,
//                 length: step,
//             });
//         }
//         if i % size_x != 0 {
//             sticks.push(Stick {
//                 point_a_index: i - 1,
//                 point_b_index: i,
//                 length: step,
//             });
//         }
//     }
//     Self {
//         points,
//         sticks,
//         max_tension: None,
//     }
// }
