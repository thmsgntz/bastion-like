extern crate core;
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod chess_pieces;
mod direction;
mod map;
mod physics;
mod fox;
mod skeleton;
mod animations_handler;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::camera::Camera3d;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier3d::prelude::*;

use bevy::window::PresentMode;

mod settings {
    use bevy::window::WindowMode;

    pub static NAME: &str = "BastionLike!";
    pub const WINDOW_WIDTH: f32 = 800.;
    pub const WINDOW_HEIGHT: f32 = 600.;
    pub const WINDOW_POSITION_X: f32 = 50.;
    pub const WINDOW_POSITION_Y: f32 = 25.;
    pub const WINDOW_MODE: WindowMode = WindowMode::Windowed;
}

fn create_camera() -> OrthographicCameraBundle<Camera3d> {
    // should consider adding: https://github.com/BlackPhlox/bevy_config_cam

    let transform =
        Transform::from_xyz(-1.0, 5.0, -1.0).looking_at(Vec3::new(4.0, 0.0, 4.0), Vec3::Y);

    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection = OrthographicProjection {
        scale: 4.0, // the lower, the higher is the zoom
        scaling_mode: ScalingMode::FixedVertical,
        ..default()
    }
    .into();
    camera.transform = transform;
    camera
}

fn draw_repere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 1,
        })),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 1,
        })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 1,
        })),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 1,
        })),
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    });
}

fn setup_camera_and_light(mut commands: Commands) {
    // camera
    commands.spawn_bundle(create_camera());

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: settings::NAME.parse().unwrap(),
            width: settings::WINDOW_WIDTH,
            height: settings::WINDOW_HEIGHT,
            position: Vec2::new(settings::WINDOW_POSITION_X, settings::WINDOW_POSITION_Y).into(),
            mode: settings::WINDOW_MODE,
            present_mode: PresentMode::Mailbox,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(animations_handler::AnimationHandler)
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        //.add_startup_system(draw_repere)
        .add_plugin(map::MapPlugin)
        //.add_plugin(chess_pieces::PiecesPlugin)
        //.add_plugin(skeleton::SkeletonPlugin)
        //.add_plugin(physics::PhysicsPlugin)
        .add_plugin(fox::FoxPlugin)
        .add_startup_system(setup_camera_and_light)
        .run();
}
