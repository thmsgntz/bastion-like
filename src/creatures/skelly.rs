use crate::creatures::{Creature, CreatureTrait, Player};
use bevy::prelude::*;
use crate::animations_handler::{AddAnimation, SceneHandle};

pub(crate) struct Skelly;

impl CreatureTrait for Skelly {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, mut event_writer: EventWriter<AddAnimation>) {
        let mut skelly_scene_handle = setup_skelly(&asset_server, "models/skeleton/scene.gltf#Scene0");

        // Skeleton
        let skelly_id = commands
            .spawn()
            .insert_bundle(PbrBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::ONE * 0.6,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_scene(skelly_scene_handle.handle.clone());
            })
            .insert(Creature(String::from("Skelly")))
            .insert(Player)
            .id()
            ;

        skelly_scene_handle.creature_entity_id = Some(skelly_id.id());

        event_writer.send(AddAnimation{
            scene_handler: skelly_scene_handle
        });
    }
}

fn setup_skelly(asset_server: &Res<AssetServer>, scene_path: &str) -> SceneHandle {
    let asset_scene_handle = asset_server.load(scene_path);

    SceneHandle {
        handle: asset_scene_handle,
        vec_animations: vec![
            asset_server.load("models/skeleton/scene.gltf#Animation0"),
            asset_server.load("models/skeleton/scene.gltf#Animation1"),
            asset_server.load("models/skeleton/scene.gltf#Animation2"),
            asset_server.load("models/skeleton/scene.gltf#Animation3"),
            asset_server.load("models/skeleton/scene.gltf#Animation4"),
            asset_server.load("models/skeleton/scene.gltf#Animation5"),
            asset_server.load("models/skeleton/scene.gltf#Animation6"),
        ],
        creature_entity_id: None,
    }
}