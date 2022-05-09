<!-- cargo-sync-readme start -->

# Bevy Cloth

[![workflow](https://github.com/ManevilleF/bevy_cloth/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_cloth/actions/workflows/rust.yml)

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

CPU driven Cloth engine for Bevy using Verlet integration.

by [Félix Lescaudey de Maneville](https://linktree.com/ManevilleF)

## Get started

### Dependency

Add `bevy_cloth` as a dependency in the `Cargo.toml`

`bevy_cloth = { git = "https://github.com/ManevilleF/bevy_cloth" }`

### Plugin

Add the `ClothPlugin` to your bevy app

```rust no_run
use bevy::prelude::*;
use bevy_cloth::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(ClothPlugin)
    // ... Add your resources and systems
    .run();
}
```

### Add cloth to a mesh

For a mesh to be used as cloth, add the `Cloth` component to any entity with a `Handle<Mesh>` component.

> Note: `Transform` and `GlobalTransform` are also required

```rust
use bevy::prelude::*;
use bevy_cloth::prelude::*;

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(PbrBundle {
        // Add your mesh, material and your custom PBR dat    
        ..Default::default()
    }).insert(Cloth::new());
}
```

The entity's mesh will now behave as cloth and will fall downwards.
To avoid this, you need to specify **fixed points** which will keep the cloth attached to the entity.
To do this you need to specify the vertex indexes to keep fixed by:
- Using `Cloth::with_fixed_points` instead of `Cloth::new`
- Editing the `Cloth::fixed_points` field

### Configuration

You can customize the cloth physics by inserting the `ClothConfig` resource to your app:

```rust no_run
use bevy::prelude::*;
use bevy_cloth::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(ClothConfig {
        gravity: Vec3::new(0.0, -9.81, 0.0),
        friction: 0.02,
        sticks_computation_depth: 5
    })
    .add_plugin(ClothPlugin)
    // ... Add your resources and systems
    .run();
}
```

## Wind

You may add wind to the simulation for a more dynamic clothing effect, you may choose from:
- `Wind::Constant` for constant wind force
- `Wind::SinWave` for a sinwave following wind intensity with custom force and frequency.

The `Wind` can be added as a resource to your app:

```rust no_run
use bevy::prelude::*;
use bevy_cloth::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Wind::SinWave {
        max_velocity: Vec3::new(10.0, 15.0, -5.0),
         frequency: 3.0,
    })
    .add_plugin(ClothPlugin)
    // ... Add your resources and systems
    .run();
}
```

> Check the flag example for simple wind effect.

## Mesh utils

`bevy_cloth` provides a plane mesh generation function `rectangle_mesh` useful for classic cloth uses like flags or capes


<!-- cargo-sync-readme end -->

## TODO list

- [x] Wind
- [ ] Different stick behaviors (spring, stick, etc)
- [ ] [heron](https://github.com/jcornaz/heron) integration to support collisions
- [ ] dynamic normal mapping

## Examples

1. Flag example

run `cargo run --example flag_example`

![GIF](docs/flag_example.gif)*Flag example*

2. Balloon example

run `cargo run --example balloon_example`

![GIF](docs/balloon_example.gif)*Balloon example*
