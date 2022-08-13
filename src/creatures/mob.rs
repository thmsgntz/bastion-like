use crate::creatures::{Creature, CreatureTrait};
use bevy::prelude::*;
use crate::animations_handler::{AddAnimation, SceneHandle};
use crate::direction::Direction;

pub(crate) struct Gollum;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum GollumAnimationId {
    Idle,
    IdleAction,
    SleepStart,
    Sleep,
    SleepEnd,
    Damage,
    Hit,
    Die,
    Walk,
    Hit2,
    Rage,
    Jump,
    Fly,
    Land,
}

impl CreatureTrait for Gollum {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, mut event_writer: EventWriter<AddAnimation>) {
        let mut scene_handler = setup_gollum(&asset_server, "models/golem/scene.gltf");

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
    let asset_scene_handle = asset_server.load(format!("{}{}", scene_path, "#Scene0").as_str());

    SceneHandle {
        handle: asset_scene_handle,
        vec_animations: vec![
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Idle as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Idle_action as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::sleep_start as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::sleep as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::sleep_end as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::damage as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::hit as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::die as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::walk as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::hit2 as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::rage as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::jump as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::fly as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::land as usize).as_str()),
        ],
        creature_entity_id: None,
    }
}