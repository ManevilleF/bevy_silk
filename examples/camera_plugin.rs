use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    log,
    prelude::*,
};

pub struct CameraPlugin;

#[derive(Debug, Component)]
pub struct CameraController;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PostUpdate, handle_camera);
        log::info!("Camera Plugin loaded");
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-30.0, 30.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController,
    ));
}

pub fn handle_camera(
    mut cam_controls: Query<&mut Transform, With<CameraController>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    let mut transform = cam_controls.single_mut();
    let forward = transform.local_z();
    let right = transform.local_x();
    let up = transform.local_y();
    // Rotate
    if buttons.pressed(MouseButton::Left) {
        for ev in motion_evr.iter() {
            let delta = -ev.delta * delta_time * 0.1;
            transform.rotate_y(delta.x);
            transform.rotate_local_x(delta.y);
        }
    }
    // Pan
    if buttons.pressed(MouseButton::Right) {
        for ev in motion_evr.iter() {
            let delta = ev.delta * delta_time;
            transform.translation += -right * delta.x;
            transform.translation += up * delta.y;
        }
    }
    // Zoom
    for ev in scroll_evr.iter() {
        transform.translation += ev.y * delta_time * forward * 10.0;
    }
}
