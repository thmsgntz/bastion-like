use bevy::{prelude::*, render::camera::ScalingMode};

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_camera);
    }
}

fn create_camera(
    mut command: Commands,
) {
    // should consider adding: https://github.com/BlackPhlox/bevy_config_cam

    let transform =
        Transform::from_xyz(-1.0, 5.0, -1.0).looking_at(Vec3::new(4.0, 0.0, 4.0), Vec3::Y);

    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection = OrthographicProjection {
        scale: 4.0, // the lower, the higher is the zoom
        scaling_mode: ScalingMode::FixedVertical,
        ..default()
    };
    camera.transform = transform;

    command.spawn_bundle(camera);
}