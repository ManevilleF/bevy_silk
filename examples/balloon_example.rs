use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_silk::prelude::*;

mod camera_plugin;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(ResourceInspectorPlugin::<ClothConfig>::new())
        .add_plugins(camera_plugin::CameraPlugin)
        .add_plugins(ClothPlugin)
        .insert_resource(ClothConfig {
            friction: 0.1,
            ..Default::default()
        })
        .add_systems(Startup, (spawn_cloth, setup))
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(DirectionalLightBundle::default());
    let mesh_handle = meshes.add(Cuboid::default());
    [
        (Color::BLUE, [-10.0, 0.0]),
        (Color::GREEN, [10.0, 0.0]),
        (Color::YELLOW, [0.0, -10.0]),
        (Color::RED, [0.0, 10.0]),
    ]
    .map(|(color, [x, z])| {
        commands.spawn(PbrBundle {
            mesh: mesh_handle.clone(),
            transform: Transform::from_xyz(x, 0.0, z),
            material: materials.add(StandardMaterial {
                base_color: color,
                double_sided: true,
                ..Default::default()
            }),
            ..Default::default()
        });
    });
}

fn spawn_cloth(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(5.).mesh().ico(10).unwrap()),
            material: materials.add(Color::YELLOW),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..Default::default()
        },
        ClothBuilder::new().with_pinned_vertex_ids(0..=0),
        Name::new("Balloon"),
    ));
}
