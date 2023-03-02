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
        .add_plugin(ResourceInspectorPlugin::<ClothConfig>::new())
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(camera_plugin::CameraPlugin)
        .add_plugin(ClothPlugin)
        .add_startup_system(spawn_cloth)
        .add_startup_system(setup)
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
    let mesh_handle = meshes.add(shape::Cube::new(1.0).into());
    [
        (Color::BLUE, [-10.0, 0.0]),
        (Color::GREEN, [10.0, 0.0]),
        (Color::YELLOW, [0.0, -10.0]),
        (Color::RED, [0.0, 10.0]),
    ]
    .map(|(color, [x, z])| {
        commands.spawn(PbrBundle {
            mesh: mesh_handle.clone(),
            transform: Transform::from_xyz(x, 1.0, z),
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
    let (size_x, size_y) = (60, 40);

    let anchor_mesh = meshes.add(shape::Cube::new(1.0).into());
    let entity_a = commands
        .spawn((
            PbrBundle {
                mesh: anchor_mesh.clone(),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_xyz(15.0, 15.0, 15.0),
                ..Default::default()
            },
            Name::new("Anchor RED"),
        ))
        .id();
    let entity_b = commands
        .spawn((
            PbrBundle {
                mesh: anchor_mesh,
                material: materials.add(Color::GREEN.into()),
                transform: Transform::from_xyz(-15.0, 15.0, 15.0),
                ..Default::default()
            },
            Name::new("Anchor GREEN"),
        ))
        .id();

    let mesh = rectangle_mesh((size_x, size_y), (-Vec3::X * 0.5, -Vec3::Y * 0.5), Vec3::Z);
    let cloth = ClothBuilder::new()
        .with_anchored_vertex_ids(
            (0..size_y).map(|i| i * size_x),
            VertexAnchor {
                custom_target: Some(entity_a),
                ..Default::default()
            },
        )
        .with_anchored_vertex_ids(
            (0..size_y).map(|i| i * size_x + size_x - 1),
            VertexAnchor {
                custom_target: Some(entity_b),
                custom_offset: Some(Vec3::X * 30.0),
                ..Default::default()
            },
        );
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(flag_texture),
                cull_mode: None,    // Option required to render back faces correctly
                double_sided: true, // Option required to render back faces correctly
                ..Default::default()
            }),
            transform: Transform::from_xyz(15.0, 15.0, 15.0),
            ..Default::default()
        },
        cloth,
        Name::new("Cloth"),
    ));
}
