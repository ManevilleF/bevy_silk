use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Mesh associated to cloth doesn't have `{0}` attribute set")]
    MissingMeshAttribute(String),
    #[error("Unsupported vertex position attribute, only `Float32x3` is supported")]
    UnsupportedVertexPositionAttribute,
    #[error("Cloth requires meshes with indexed geometry")]
    MissingIndices,
}
