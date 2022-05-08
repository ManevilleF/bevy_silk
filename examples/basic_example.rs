use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;
use bevy_cloth::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor::default())
        .insert_resource(AmbientLight {
            color: Color::YELLOW,
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(WorldInspectorPlugin::default())
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
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..PerspectiveCameraBundle::new_3d()
    });
    let mesh_handle = meshes.add(shape::Cube::new(1.0).into());
    [
        (Color::BLUE, [-10.0, 0.0]),
        (Color::GREEN, [10.0, 0.0]),
        (Color::YELLOW, [0.0, -10.0]),
        (Color::RED, [0.0, 10.0]),
    ]
    .map(|(color, [x, z])| {
        commands.spawn_bundle(PbrBundle {
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
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 5.0,
                    subdivisions: 10,
                }
                .into(),
            ),
            material: materials.add(Color::WHITE.into()),
            ..Default::default()
        })
        .insert(Cloth::new(vec![0].into_iter()))
        .insert(Wireframe)
        .insert(Name::new("Ico Cloth"));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(rectangle_mesh(10, 10, Vec3::X, Vec3::Z)),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            }),
            // transform: Transform::from_xyz(-5.0, 5.0, 0.0),
            ..Default::default()
        })
        .insert(Cloth::new((0..10).into_iter()))
        .insert(Wireframe)
        .insert(Name::new("Plane Cloth"));
}
