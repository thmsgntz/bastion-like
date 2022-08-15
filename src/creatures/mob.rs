use crate::creatures::{Creature, CreatureTrait, CurrentAnimationIndex, TypeCreature};
use bevy::prelude::*;
use crate::animations_handler::{AddAnimation, ChangeAnimation, HashMapAnimationClip, SceneHandle};
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

impl Into<usize> for GollumAnimationId {
    fn into(self) -> usize {
        match self {
            GollumAnimationId::Idle => {0}
            GollumAnimationId::IdleAction => {1}
            GollumAnimationId::SleepStart => {2}
            GollumAnimationId::Sleep => {3}
            GollumAnimationId::SleepEnd => {4}
            GollumAnimationId::Damage => {5}
            GollumAnimationId::Hit => {6}
            GollumAnimationId::Die => {7}
            GollumAnimationId::Walk => {8}
            GollumAnimationId::Hit2 => {9}
            GollumAnimationId::Rage => {10}
            GollumAnimationId::Jump => {11}
            GollumAnimationId::Fly => {12}
            GollumAnimationId::Land => {13}
        }
    }
}

impl Into<CurrentAnimationIndex> for GollumAnimationId {
    fn into(self) -> CurrentAnimationIndex {
        CurrentAnimationIndex(self.into())
    }
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
            .insert(Creature {
                type_creature: TypeCreature::gollum,
                current_animation_index: GollumAnimationId::Idle.into(),
            })
            .id()
            ;

        scene_handler.creature_entity_id = Some(id_creature.id());

        event_writer.send(
            AddAnimation {
                scene_handler,
            }
        );
    }

    fn update_animation(_target: u32, _index_animation: usize, _event_writer: &mut EventWriter<ChangeAnimation>) {
        todo!()
    }
}

fn setup_gollum(asset_server: &Res<AssetServer>, scene_path: &str) -> SceneHandle {
    let asset_scene_handle = asset_server.load(format!("{}{}", scene_path, "#Scene0").as_str());

    let hm_animations = HashMapAnimationClip::new();

    // let id = SkellyAnimationId::Idle;
    // hm_animations.insert(id.into(), id.get_duration(),  asset_server.load(format!("{}#Animation{}", scene_path, id as usize).as_str()));

    SceneHandle {
        handle: asset_scene_handle,
        vec_animations: hm_animations,
/*        vec_animations: vec![
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Idle as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::IdleAction as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::SleepStart as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Sleep as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::SleepEnd as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Damage as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Hit as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Die as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Walk as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Hit2 as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Rage as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Jump as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Fly as usize).as_str()),
            asset_server.load(format!("{}#Animation{}", scene_path, GollumAnimationId::Land as usize).as_str()),
        ],*/
        creature_entity_id: None,
    }
}