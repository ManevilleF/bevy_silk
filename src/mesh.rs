// TODO: mesh generation utils

use bevy_math::Vec3;
use bevy_render::mesh::{Indices, Mesh, PrimitiveTopology};

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
#[must_use]
/// Creates a cloth ready mesh in a triangle shape
///
/// # Params
///
/// * `size_x` - the size of the cloth in the X axis (should be above 1)
/// * `size_z` - the size of the cloth in the Z axis (should be above 1)
/// * `step` - the direction of the cloth propagation
/// * `normal` - the normal vector to apply to each vertex
pub fn rectangle_mesh(size_x: usize, size_y: usize, step: Vec3, normal: Vec3) -> Mesh {
    let points: Vec<[f32; 3]> = (0..size_y)
        .flat_map(|y| {
            (0..size_x).map(move |x| ((x % size_x) as f32 * step - Vec3::Y * y as f32).to_array())
        })
        .collect();
    let normal = normal.to_array();
    let normals: Vec<[f32; 3]> = (0..points.len()).map(|_| normal).collect();
    let uvs: Vec<[f32; 2]> = (0..size_y)
        .flat_map(|y| {
            (0..size_x).map(move |x| [x as f32 / size_x as f32, y as f32 / size_y as f32])
        })
        .collect();
    let indices: Vec<u32> = (0..points.len())
        .flat_map(|i| {
            let mut res = vec![];
            let i_32 = i as u32;
            if (i + 1) % size_x != 0 && points.get(i + size_x).is_some() {
                res.extend(vec![i_32 + 1, i_32, i_32 + size_x as u32]);
            }
            if i % size_x != 0
                && i.checked_sub(size_x)
                    .and_then(|i2| points.get(i2))
                    .is_some()
            {
                res.extend(vec![i_32 - 1, i_32, i_32 - size_x as u32]);
            }
            res
        })
        .collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_rectangle_mesh() {
        let mesh = rectangle_mesh(100, 100, Vec3::X, Vec3::Z);
        assert_eq!(mesh.count_vertices(), 100 * 100);
    }
}
