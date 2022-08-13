//extern crate core;
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod direction;
mod map;
mod animations_handler;
mod creatures;
mod inputs;
mod camera;


use bevy::prelude::*;
use bevy::log::LogSettings;

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

fn setup_light(mut commands: Commands) {

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
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,bastion_like=debug".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(inputs::InputsPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(animations_handler::AnimationHandler)
        .add_plugin(map::MapPlugin)
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(creatures::CreaturePlugin)
        .add_startup_system(setup_light)

        .run();
}
