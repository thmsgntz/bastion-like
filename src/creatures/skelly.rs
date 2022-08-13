use crate::creatures::{Creature, CreatureTrait, Player};
use bevy::prelude::*;
use crate::animations_handler::{AddAnimation, SceneHandle};

pub(crate) struct Skelly;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum SkellyAnimationId {
    Idle,          // duration: 1.5800002
    LookingAround, // duration: 3.1800003
    Attack,        // duration: 2.3200002
    Yell,          // duration: 1.5800002
    Walk,          // duration: 0.9800001
    Run,           // duration: 0.78000003
    Fall,          // ?
    Hit,           // ?
    Die,           // ?
    Spawn,         // ?
    Hanged,        // ?
    None,          // ?
}

const SKELLY_ANIM_DURATION_IDLE: f32 = 1.58;
const SKELLY_ANIM_DURATION_YELL: f32 = 1.58;
const SKELLY_ANIM_DURATION_LOOKING_AROUND: f32 = 3.18;
const SKELLY_ANIM_DURATION_ATTACK: f32 = 2.32;
const SKELLY_ANIM_DURATION_WALK: f32 = 0.98;
const SKELLY_ANIM_DURATION_RUN: f32 = 0.78;
const SKELLY_ANIM_DURATION_FALL: f32 = 1.0; // TO CHECK
const SKELLY_ANIM_DURATION_HIT: f32 = 0.6; // TO CHECK
const SKELLY_ANIM_DURATION_DIE: f32 = 1.0; // TO CHECK
const SKELLY_ANIM_DURATION_SPAWN: f32 = 1.58; // TO CHECK
const SKELLY_ANIM_DURATION_HANGED: f32 = 1.58; // TO CHECK

impl CreatureTrait for Skelly {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, mut event_writer: EventWriter<AddAnimation>) {
        let mut skelly_scene_handle = setup_skelly(&asset_server, "models/skeleton/scene.gltf");

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
    let asset_scene_handle = asset_server.load(format!("{}{}", scene_path, "#Scene0").as_str());

    SceneHandle {
        handle: asset_scene_handle,
        vec_animations: vec![
            asset_server.load(format!("{}#Animation{}", scene_path, SkellyAnimationId::Idle as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, SkellyAnimationId::LookingAround as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, SkellyAnimationId::Attack as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, SkellyAnimationId::Yell as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, SkellyAnimationId::Walk as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, SkellyAnimationId::Run as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, SkellyAnimationId::Fall as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, SkellyAnimationId::Hit as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, SkellyAnimationId::Die as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, SkellyAnimationId::Spawn as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, SkellyAnimationId::Hanged as usize).as_str()),
        ],
        creature_entity_id: None,
    }
}
