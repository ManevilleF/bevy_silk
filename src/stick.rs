/// A single cloth "stick" connecting two points
#[derive(Debug, Copy, Clone, Default)]
pub struct Stick {
    /// Index of a first [`Point`] in a [`Cloth`]
    ///
    /// [`Point`]: crate::point::Point
    /// [`Cloth`]: crate::cloth::Cloth
    pub point_a_index: usize,
    /// Index of a second [`Point`] in a [`Cloth`]
    ///
    /// [`Point`]: crate::point::Point
    /// [`Cloth`]: crate::cloth::Cloth
    pub point_b_index: usize,
    /// Target stick length
    pub length: f32,
}
