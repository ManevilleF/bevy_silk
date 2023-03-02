# Changelog

## Uneleased

* Bump `bevy_rapier3d` to `0.20.x`
* `ClothBuilder` is now `Reflect` and registered
* Added vertex position anchoring for `ClothBuilder`:
  * `ClothBuilder::with_pinned_vertex_positions`
  * `ClothBuilder::with_anchored_vertex_positions`
* Removed unused `VertexAnchor::world_space` attribute

### bevy_inspector_egui

With the new version (See [Migration guide](https://github.com/jakobhellermann/bevy-inspector-egui/blob/main/docs/MIGRATION_GUIDE_0.15_0.16.md)),
the following changes were applied:
* (**BREAKING**) Removed `debug` feature
* Removed `bevy_inspector_egui` dependency, kept only as `dev-dependency` for examples

## 0.4.0

* Bump `bevy` to `0.9.x`
* Bump `bevy_rapier3d` to `0.19.x`
* Bump `bevy_inspector_egui` to `0.14.x`
* Modules refactoring

## 0.3.0

* Bump `bevy` to `0.8.x`
* Bump `bevy_rapier3d` to `0.18.x`
* Bump `bevy_inspector_egui` to `0.13.x`
* Removed `smooth-bevy-cameras` dependency

## 0.2.0

### Added

* [bevy_rapier](https://github.com/dimforge/bevy_rapier) collision support:
  * Added `rapier_collisions` feature
  * Added `rapier_collision` example
  * Added `ClothCollider` component
* Added `AccelerationSmoothing` enum, defining gravity/winds acceleration smoothing
  * Added related `acceleration_smoothing` field to `ClothConfig`
* Added custom anchor support with `VertexAnchor`

### API changes

* (**BREAKING**) Renamed `ClothBuilder::fixed_points` to `anchored_vertex_ids`
  * Added `ClothBuilder::with_pinned_vertex_ids` method
  * Added `ClothBuilder::with_pinned_vertex_id` method
  * Added `ClothBuilder::with_anchored_vertex_ids` method
  * Added `ClothBuilder::with_anchored_vertex_id` method
  * Deprecated `ClothBuilder::with_fixed_points` in favor of `ClothBuilder::with_pinned_vertex_ids`
* Added `ClothBuilder::anchored_vertex_colors` field:
  * Added `ClothBuilder::with_pinned_vertex_colors` method
  * Added `ClothBuilder::with_pinned_vertex_color` method
  * Added `ClothBuilder::with_anchored_vertex_colors` method
  * Added `ClothBuilder::with_anchored_vertex_color` method
* Added `ClothBuilder::with_flat_normals` method
  * Deprecated `ClothBuilder::with_flat_normal_computation` in favor of `ClothBuilder::with_flat_normals`
* Added `ClothBuilder::with_smooth_normals` method
  * Deprecated `ClothBuilder::with_smooth_normal_computation` in favor of `ClothBuilder::with_smooth_normals`

### Examples

* Added `rapier_collisions` example
* Added `anchors` example

## 0.1.0

First version
