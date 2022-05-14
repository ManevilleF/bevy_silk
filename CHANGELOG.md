# Changelog

# Unreleased

* Added `AccelerationSmoothing` enum, defining gravity/winds acceleration smoothing
* Added related `acceleration_smoothing` field to `ClothConfig` 
* (**BREAKING**) Renamed `ClothBuilder::fixed_points` to `pinned_vertex_ids`
* Added `ClothBuilder::with_pinned_vertex_ids` method
* Deprecated `ClothBuilder::with_fixed_points` in favor of `ClothBuilder::with_pinned_vertex_ids`
* Added `ClothBuilder::pinned_vertex_colors` field
* Added `ClothBuilder::with_pinned_vertex_colors` method
* Added `ClothBuilder::with_flat_normals` method
* Deprecated `ClothBuilder::with_flat_normal_computation` in favor of `ClothBuilder::with_flat_normals`
* Added `ClothBuilder::with_smooth_normals` method
* Deprecated `ClothBuilder::with_smooth_normal_computation` in favor of `ClothBuilder::with_smooth_normals`

# 0.1.0

First version