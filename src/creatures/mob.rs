use crate::creatures;
use crate::creatures::Creature;
use bevy::prelude::*;
use crate::direction::Direction;
use crate::skelly::Skelly;

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
            // .add_system(setup_scene_once_loaded);
    }
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn()
        .insert_bundle(Creature {
            transform: PbrBundle {
                transform: Transform {
                    translation: Vec3::new(7.0, 0.0, 7.0),
                    rotation: Quat::from_rotation_y(Direction::Down.get_angle()),
                    scale: Vec3::ONE * 1.5,
                },
                ..Default::default()
            }})
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/golem/scene.gltf#Scene0"));
        });
}
