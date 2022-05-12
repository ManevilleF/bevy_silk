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

For a mesh to be used as cloth, add the `ClothBuilder` component to any entity with a `Handle<Mesh>` component.

> Note: `Transform` and `GlobalTransform` are also required

cloth data which will be populated automatically from the associated `Handle<Mesh>`.

```rust
use bevy::prelude::*;
use bevy_cloth::prelude::*;

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(PbrBundle {
        // Add your mesh, material and your custom PBR data   
        ..Default::default()
    }).insert(ClothBuilder::new()
        // Define fixed vertices using an Iterator
        .with_fixed_points(0..9)
        // Define the stick generation mode
        .with_stick_generation(StickGeneration::Quads)
        // Defines the sticks target length option
        .with_stick_length(StickLen::Auto)
        // Defines that the cloth will compute mesh normals
        .with_normal_computation()
    );
}
```

### Configuration

You can customize the global cloth physics by inserting the `ClothConfig` resource to your app:

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

`ClothConfig` can also be used as a *component* to override the global configuration.

## Wind

You may add wind forces to the simulation for a more dynamic clothing effect, for each force you may choose from:
- `Wind::Constant` for constant wind force
- `Wind::SinWave` for a sinwave following wind intensity with custom force and frequency.

`Wind` forces can be added as a resource to your app through the `Winds` container:

```rust no_run
use bevy::prelude::*;
use bevy_cloth::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Winds {
        wind_forces: vec![Wind::SinWave {
            max_velocity: Vec3::new(10.0, 15.0, -5.0),
            frequency: 3.0,
            normalize: false,
            abs: false
        }]
    })
    .add_plugin(ClothPlugin)
    // ... Add your resources and systems
    .run();
}
```

> Check the flag example for simple wind effect.

## Mesh utils

`bevy_cloth` provides a plane mesh generation function `rectangle_mesh` useful for classic cloth uses like flags or capes

## Q&A

- `My mesh falls immediately and infinitely when I add a Cloth component, how to fix it?`

You probably didn't specify any *fixed points*, meaning there are no vertices anchored to your entity's `GlobalTransform`.


<!-- cargo-sync-readme end -->

## TODO list

- [x] Wind
- [x] Custom stick target length options
- [ ] Different stick behaviors (spring, stick, etc)
- [ ] [heron](https://github.com/jcornaz/heron) integration to support collisions
- [ ] dynamic normal mapping
- [ ] Cloth cutting maybe?

## Examples

1. Flag example

run `cargo run --example flag_example --features debug`

2. Balloon example

run `cargo run --example balloon_example --features debug`

3. Moving example

run `cargo run --example moving_example --features debug`