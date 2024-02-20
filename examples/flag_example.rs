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
        .add_plugins(ResourceInspectorPlugin::<Winds>::new())
        .add_plugins(ResourceInspectorPlugin::<ClothConfig>::new())
        .add_plugins(camera_plugin::CameraPlugin)
        .insert_resource(Winds::from(vec![
            Wind::SinWave {
                max_velocity: Vec3::new(20.0, 15.0, 0.0),
                frequency: 3.0,
                normalize: true,
                abs: false,
            },
            Wind::SinWave {
                max_velocity: Vec3::new(0.0, 0.0, 15.0),
                frequency: 2.0,
                normalize: false,
                abs: false,
            },
        ]))
        .add_plugins(ClothPlugin)
        .add_systems(Startup, (spawn_cloth, setup))
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_rotation_y(5.0)),
        ..Default::default()
    });
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
    asset_server: Res<AssetServer>,
) {
    let flag_texture = asset_server.load("Bevy.png");
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(flag_texture),
        cull_mode: None,    // Option required to render back faces correctly
        double_sided: true, // Option required to render back faces correctly
        ..Default::default()
    });
    let (size_x, size_y) = (30, 15);
    let mesh = rectangle_mesh((size_x, size_y), (Vec3::X * 0.5, -Vec3::Y * 0.5), Vec3::Z);

    // Regular Smooth Flag
    let cloth = ClothBuilder::new()
        .with_pinned_vertex_ids((0..size_y).map(|i| i * size_x))
        .with_stick_generation(StickGeneration::Triangles);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh.clone()),
            material: material.clone(),
            transform: Transform::from_xyz(0.0, 8.0, 10.0),
            ..Default::default()
        },
        cloth,
        Name::new("Regular Smooth Flag"),
    ));

    // Regular Flat Flag
    let cloth = ClothBuilder::new()
        .with_pinned_vertex_ids((0..size_y).map(|i| i * size_x))
        .with_flat_normals();
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh.clone()),
            material: material.clone(),
            transform: Transform::from_xyz(0.0, 8.0, 0.0),
            ..Default::default()
        },
        cloth,
        Name::new("Regular Flat Flag"),
    ));

    // Color flag
    let mut mesh = mesh;
    let colors: Vec<[f32; 4]> = (0..size_y)
        .flat_map(|_| {
            (0..size_x).map(|v| {
                let v = v as f32 / size_x as f32;
                [1.0, v, v, 1.0]
            })
        })
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    let cloth = ClothBuilder::new().with_pinned_vertex_color(Color::RED);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material,
            transform: Transform::from_xyz(0.0, 8.0, -10.0),
            ..Default::default()
        },
        cloth,
        Name::new("Colored smooth Flag"),
    ));
}
