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
    None,
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
            GollumAnimationId::None => {14}
        }
    }
}

impl From<usize> for GollumAnimationId {
    fn from(i: usize) -> Self {
        match i {
            0 => {  GollumAnimationId::Idle }
            1 => {  GollumAnimationId::IdleAction }
            2 => {  GollumAnimationId::SleepStart }
            3 => {  GollumAnimationId::Sleep }
            4 => {  GollumAnimationId::SleepEnd }
            5 => {  GollumAnimationId::Damage }
            6 => {  GollumAnimationId::Hit }
            7 => {  GollumAnimationId::Die }
            8 => {  GollumAnimationId::Walk }
            9 => {  GollumAnimationId::Hit2 }
            10 => { GollumAnimationId::Rage }
            11 => { GollumAnimationId::Jump }
            12 => { GollumAnimationId::Fly }
            13 => { GollumAnimationId::Land }
            _ => {GollumAnimationId::None}
        }
    }
}

impl GollumAnimationId {
    fn get_duration (&self) -> f32 {
        match self {
            GollumAnimationId::Idle => {ANIMATION_DURATION_IDLE }
            GollumAnimationId::IdleAction => {ANIMATION_DURATION_IDLE_ACTION}
            GollumAnimationId::SleepStart => {ANIMATION_DURATION_SLEEP_START }
            GollumAnimationId::Sleep => {ANIMATION_DURATION_SLEEP }
            GollumAnimationId::SleepEnd => {ANIMATION_DURATION_SLEEP_END }
            GollumAnimationId::Damage => {ANIMATION_DURATION_DAMAGE }
            GollumAnimationId::Hit => {ANIMATION_DURATION_HIT }
            GollumAnimationId::Die => {ANIMATION_DURATION_DIE }
            GollumAnimationId::Walk => {ANIMATION_DURATION_WALK }
            GollumAnimationId::Hit2 => {ANIMATION_DURATION_HIT2 }
            GollumAnimationId::Rage => {ANIMATION_DURATION_RAGE }
            GollumAnimationId::Jump => {ANIMATION_DURATION_JUMP}
            GollumAnimationId::Fly => {ANIMATION_DURATION_FLY}
            GollumAnimationId::Land => {ANIMATION_DURATION_LAND}
            GollumAnimationId::None => {ANIMATION_DURATION_IDLE}
        }
    }
}

/*
 TODO:
    Les animations ont une durée chelou, croissante.
    On dirait que IDLE_ACTION a 2 sec inutile, avant de jouer son animation de 2.1 secondes.
    Comme si elle devait commencer après les 2sec de la n-1.
    Chiant.
 */
const ANIMATION_DURATION_IDLE : f32 = 2.00;
const ANIMATION_DURATION_IDLE_ACTION: f32 = 4.1;
const ANIMATION_DURATION_SLEEP_START : f32 = 4.3333335;
const ANIMATION_DURATION_SLEEP : f32 = 6.3333335;
const ANIMATION_DURATION_SLEEP_END : f32 = 7.6666665;
const ANIMATION_DURATION_DAMAGE : f32 = 9.0;
const ANIMATION_DURATION_HIT : f32 = 9.933333;
const ANIMATION_DURATION_DIE : f32 = 11.066667;
const ANIMATION_DURATION_WALK : f32 = 12.133333;
const ANIMATION_DURATION_HIT2 : f32 = 12.933333;
const ANIMATION_DURATION_RAGE : f32 = 14.533334;
const ANIMATION_DURATION_JUMP : f32 = 15.5;
const ANIMATION_DURATION_FLY : f32 = 16.166666;
const ANIMATION_DURATION_LAND : f32 = 16.666666;

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
                type_creature: TypeCreature::Gollum,
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

    fn update_animation(target: u32, index_animation: usize, event_writer: &mut EventWriter<ChangeAnimation>) {
        info!("calling with {:#?} {}", GollumAnimationId::from(index_animation), index_animation);
        let mut new_animation = GollumAnimationId::Idle;
        let mut repeat = false;

        match GollumAnimationId::from(index_animation) {
            GollumAnimationId::Idle => {
                new_animation = GollumAnimationId::IdleAction;
            }
            GollumAnimationId::IdleAction => {
                new_animation = GollumAnimationId::Idle;
            }
            GollumAnimationId::SleepStart => {}
            GollumAnimationId::Sleep => {}
            GollumAnimationId::SleepEnd => {}
            GollumAnimationId::Damage => {}
            GollumAnimationId::Hit => {}
            GollumAnimationId::Die => {}
            GollumAnimationId::Walk => {}
            GollumAnimationId::Hit2 => {}
            GollumAnimationId::Rage => {}
            GollumAnimationId::Jump => {}
            GollumAnimationId::Fly => {}
            GollumAnimationId::Land => {}
            GollumAnimationId::None => {}
        }


        event_writer.send(
            ChangeAnimation {
                target,
                index: new_animation.into(),
                repeat
            }
        );
    }
}

fn setup_gollum(asset_server: &Res<AssetServer>, scene_path: &str) -> SceneHandle {

    let asset_scene_handle = asset_server.load(format!("{}{}", scene_path, "#Scene0").as_str());

    let mut hm_animations = HashMapAnimationClip::new();

    for i in 0..GollumAnimationId::None.into() {
        let id = GollumAnimationId::from(i as usize);
        let handle = asset_server.load(format!("{}#Animation{}", scene_path, id as usize).as_str());
        hm_animations.insert(id.into(), id.get_duration(), handle);
    }

    SceneHandle {
        handle: asset_scene_handle,
        vec_animations: hm_animations,
        creature_entity_id: None,
    }
}