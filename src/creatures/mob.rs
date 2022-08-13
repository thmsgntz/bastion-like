use crate::creatures::{Creature, CreatureTrait};
use bevy::prelude::*;
use crate::animations_handler::{AddAnimation, SceneHandle};
use crate::direction::Direction;

pub(crate) struct Gollum;

impl CreatureTrait for Gollum {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, mut event_writer: EventWriter<AddAnimation>) {
        let mut scene_handler = setup_gollum(&asset_server, "models/golem/scene.gltf#Scene0");

        let id_creature = commands
            .spawn()
            .insert_bundle(PbrBundle {
                transform: Transform {
                    translation: Vec3::new(7.0, 0.0, 7.0),
                    rotation: Quat::from_rotation_y(Direction::Down.get_angle()),
                    scale: Vec3::ONE * 1.5,
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_scene(scene_handler.handle.clone());
            })
            .insert(Creature("Gollum".into()))
            .id()
            ;

        scene_handler.creature_entity_id = Some(id_creature.id());

        event_writer.send(
            AddAnimation {
                scene_handler,
            }
        );
    }
}

fn setup_gollum(asset_server: &Res<AssetServer>, scene_path: &str) -> SceneHandle {
    let asset_scene_handle = asset_server.load(scene_path);

    SceneHandle {
        handle: asset_scene_handle,
        vec_animations: vec![
            asset_server.load("models/golem/scene.gltf#Animation0"),
            asset_server.load("models/golem/scene.gltf#Animation1"),
            asset_server.load("models/golem/scene.gltf#Animation2"),
            asset_server.load("models/golem/scene.gltf#Animation3"),
            asset_server.load("models/golem/scene.gltf#Animation4"),
            asset_server.load("models/golem/scene.gltf#Animation5"),
        ],
        creature_entity_id: None,
    }
}