use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::direction;
use direction::ENTITY_LOOKING_UP;

// https://github.com/bevyengine/bevy/blob/main/examples/animation/animated_fox.rs

/*

1. AnimationPlayer is a component attached to the root node of the model loaded from a GLTF file.
This component is inserted by the GLTF importer.
AnimationPlayer just knows how far into an animation it is, the speed, if it is paused or stopped.
AnimationClip is the actual animation.
This stores information about the keyframes the the animation is composed of.
When you start an animation you just tell the AnimationPlayer which AnimationClip asset it should use.

2. If you add a second Fox it will be given its own AnimationPlayer component, which can run another animation.

So if you want seperate animations for each Animated Fox you can iterate through the AnimationPlayer
 components with a query and tell them to use different AnimationClip assets.

 */

pub struct SkeletonPlugin;
impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(setup_scene_once_loaded)
            //.add_system(inspect_animation_clip.after(setup_scene_once_loaded))
            .add_system(keyboard_animation_control);
    }
}

pub const ENTITY_SPEED: f32 = 0.04;

#[derive(Component)]
struct Skelly {
    current_animation_id: SkellyAnimationId,
    hash_animations: HashMap<SkellyAnimationId, Animation>,
    //player: Option<&mut AnimationPlayer>,
}

impl Default for Skelly {
    fn default() -> Self {
        Skelly {
            //facing: direction::Direction::default(),
            current_animation_id: SkellyAnimationId::Idle,
            hash_animations: HashMap::new(),
            //player: None
        }
    }
}

impl Skelly {

    fn is_ready(&self, player:&mut AnimationPlayer) -> bool {
        let current_animation = self.current_animation_id;

        if current_animation == SkellyAnimationId::Idle {
            return true
        }

        if let Some(animation) = self.hash_animations.get(&current_animation) {
            return if !animation.is_repeatable {
                if player.elapsed() >= animation.duration {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        }

        false
    }

    fn can_move(&self) -> bool {
        match self.current_animation_id {
            SkellyAnimationId::Idle | SkellyAnimationId::Walk | SkellyAnimationId::Run => true,
            _ => false
        }
    }

    /// play_animation_action is meant to be used for Yell, Attack
    fn play_animation_action(&mut self, player:&mut AnimationPlayer, action_id: SkellyAnimationId) {
        if self.current_animation_id != action_id
        {
            if let Some(animation) = self.hash_animations.get(&action_id) {
                self.current_animation_id = action_id;
                player.play(animation.handle.clone_weak());
            }
        }
    }

    fn play_animation_move(
        &mut self,
        player:&mut AnimationPlayer,
        action_id: SkellyAnimationId,
    ) {
        if self.current_animation_id == SkellyAnimationId::Idle
            || self.current_animation_id != action_id
        {
            if let Some(animation) = self.hash_animations.get(&action_id) {
                self.current_animation_id = action_id;
                player.play(animation.handle.clone_weak());
            }
        }
    }

    fn play_animation_idle(&mut self, player:&mut AnimationPlayer) {
        if self.current_animation_id != SkellyAnimationId::Idle
        {
            if let Some(animation) = self.hash_animations.get(&SkellyAnimationId::Idle) {
                self.current_animation_id = SkellyAnimationId::Idle;
                player.play(animation.handle.clone_weak()).repeat();
            }
        }
    }

    fn play_animation_fall(&mut self, player:&mut AnimationPlayer) {
        if self.current_animation_id != SkellyAnimationId::Fall
        {
            if let Some(animation) = self.hash_animations.get(&SkellyAnimationId::Fall) {
                self.current_animation_id = SkellyAnimationId::Fall;
                player.play(animation.handle.clone_weak());
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum SkellyAnimationId {
    Idle, // duration: 1.5800002
    LookingAround,  // duration: 3.1800003
    Attack,  // duration: 2.3200002
    Yell,  // duration: 1.5800002
    Walk,  // duration: 0.9800001
    Run,  // duration: 0.78000003
    Fall,  // ?
    Hit,  // ?
    Die,
    Spawn,
    Hanged,
    None,
}

const SKELLY_ANIM_DURATION_IDLE : f32 = 1.58;
const SKELLY_ANIM_DURATION_YELL : f32 = 1.58;
const SKELLY_ANIM_DURATION_LOOKING_AROUND : f32 = 3.18;
const SKELLY_ANIM_DURATION_ATTACK : f32 = 2.32;
const SKELLY_ANIM_DURATION_WALK : f32 = 0.98;
const SKELLY_ANIM_DURATION_RUN : f32 = 0.78;
const SKELLY_ANIM_DURATION_FALL : f32 = 1.0; // TO CHECK
const SKELLY_ANIM_DURATION_HIT: f32 = 0.6; // TO CHECK
const SKELLY_ANIM_DURATION_DIE: f32 = 1.0; // TO CHECK
const SKELLY_ANIM_DURATION_SPAWN: f32 = 1.58; // TO CHECK
const SKELLY_ANIM_DURATION_HANGED: f32 = 1.58; // TO CHECK

struct Animation {
    id: SkellyAnimationId,
    handle: Handle<AnimationClip>,
    duration: f32,
    is_repeatable: bool,
}

struct VecAnimations(Vec<Handle<AnimationClip>>);

impl VecAnimations {
    pub fn play (&self, player : &mut AnimationPlayer, id_animation: SkellyAnimationId, do_repeat:bool) {

        let anim = self.0[id_animation as usize].clone_weak();
        // info!("Animation {:#?} duration {:#?}: ", id_animation, anim);

        if do_repeat {
            player.play(anim).repeat();
        } else {
            player.play(anim);
        }
    }
}

fn inspect_animation_clip(
    assets_handle: Res<Assets<AnimationClip>>,
    animations: Res<VecAnimations>,
    mut inspect_done: Local<bool>,  // play it once
) {
    if !*inspect_done {
        info!("Inspection!");
        if let Some(anim) = assets_handle.get(&animations.0[SkellyAnimationId::Run  as usize]) {
            info!("Inspection! 3");
            info!("Anim {:#?}", anim);
            *inspect_done = true;
        }
    }
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<VecAnimations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        // if let Some(anim) = assets_handle.get(&animations.0[0]) {
        //     info!("Inspection! 3");
        //     info!("Anim {:#?}", anim); // duration: 1.5800002
        // }

        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    let skelly_idle_animation = Animation {
        id: SkellyAnimationId::Idle,
        handle: asset_server.load("models/skeleton/scene.gltf#Animation1"),
        duration: SKELLY_ANIM_DURATION_IDLE,
        is_repeatable: true,
    };

    let skelly_looking_around_animation = Animation {
        id: SkellyAnimationId::LookingAround,
        handle: asset_server.load("models/skeleton/scene.gltf#Animation2"),
        duration: SKELLY_ANIM_DURATION_LOOKING_AROUND,
        is_repeatable: false,
    };

    let skelly_attack_animation = Animation {
        id: SkellyAnimationId::Attack,
        handle: asset_server.load("models/skeleton/scene.gltf#Animation3"),
        duration: SKELLY_ANIM_DURATION_ATTACK,
        is_repeatable: false,
    };

    let skelly_yell_animation = Animation {
        id: SkellyAnimationId::Yell,
        handle: asset_server.load("models/skeleton/scene.gltf#Animation4"),
        duration: SKELLY_ANIM_DURATION_YELL,
        is_repeatable: false,
    };

    let skelly_walk_animation = Animation {
        id: SkellyAnimationId::Walk,
        handle: asset_server.load("models/skeleton/scene.gltf#Animation5"),
        duration: SKELLY_ANIM_DURATION_WALK,
        is_repeatable: true,
    };

    let skelly_run_animation = Animation {
        id: SkellyAnimationId::Run,
        handle: asset_server.load("models/skeleton/scene.gltf#Animation6"),
        duration: SKELLY_ANIM_DURATION_RUN,
        is_repeatable: true,
    };

    let skelly_fall_animation = Animation {
        id: SkellyAnimationId::Fall,
        handle: asset_server.load("models/skeleton/scene.gltf#Animation7"),
        duration: SKELLY_ANIM_DURATION_FALL,
        is_repeatable: false,
    };

    let skelly_hit_animation = Animation {
        id: SkellyAnimationId::Hit,
        handle: asset_server.load("models/skeleton/scene.gltf#Animation8"),
        duration: SKELLY_ANIM_DURATION_HIT,
        is_repeatable: false,
    };

    let skelly_die_animation = Animation {
        id: SkellyAnimationId::Die,
        handle: asset_server.load("models/skeleton/scene.gltf#Animation9"),
        duration: SKELLY_ANIM_DURATION_DIE,
        is_repeatable: false,
    };

    let skelly_spawn_animation = Animation {
        id: SkellyAnimationId::Spawn,
        handle: asset_server.load("models/skeleton/scene.gltf#Animation0"),
        duration: SKELLY_ANIM_DURATION_SPAWN,
        is_repeatable: false,
    };

    let skelly_hanged_animation = Animation {
        id: SkellyAnimationId::Hanged,
        handle: asset_server.load("models/skeleton/scene.gltf#Animation10"),
        duration: SKELLY_ANIM_DURATION_HANGED,
        is_repeatable: false,
    };


    /*
    5 Walk,  // duration: 0.9800001
    6 Run,  // duration: 0.78000003
    7 None,
     */

    // Insert a resource with the current scene information
    commands.insert_resource(VecAnimations(vec![
        skelly_idle_animation.handle.clone(),
        skelly_looking_around_animation.handle.clone(),
        skelly_attack_animation.handle.clone(),
        skelly_yell_animation.handle.clone(),
        skelly_walk_animation.handle.clone(),
        skelly_run_animation.handle.clone(),
        skelly_fall_animation.handle.clone(),
    ]));

    let mut skelly_entity = Skelly {
        current_animation_id: SkellyAnimationId::Idle,
        hash_animations: Default::default(),
        //player: None
    };

    skelly_entity.hash_animations.insert(SkellyAnimationId::Idle, skelly_idle_animation);
    skelly_entity.hash_animations.insert(SkellyAnimationId::Yell, skelly_yell_animation);
    skelly_entity.hash_animations.insert(SkellyAnimationId::LookingAround, skelly_looking_around_animation);
    skelly_entity.hash_animations.insert(SkellyAnimationId::Attack, skelly_attack_animation);
    skelly_entity.hash_animations.insert(SkellyAnimationId::Walk, skelly_walk_animation);
    skelly_entity.hash_animations.insert(SkellyAnimationId::Run, skelly_run_animation);
    skelly_entity.hash_animations.insert(SkellyAnimationId::Fall, skelly_fall_animation);
    skelly_entity.hash_animations.insert(SkellyAnimationId::Hit, skelly_hit_animation);
    skelly_entity.hash_animations.insert(SkellyAnimationId::Die, skelly_die_animation);
    skelly_entity.hash_animations.insert(SkellyAnimationId::Spawn, skelly_spawn_animation);
    skelly_entity.hash_animations.insert(SkellyAnimationId::Hanged, skelly_hanged_animation);

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform {
                translation: Vec3::new(4.0, 0.0, 4.0),
                rotation: Quat::from_rotation_y(ENTITY_LOOKING_UP),
                scale: Vec3::ONE * 0.6,
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/skeleton/scene.gltf#Scene0"));
        })
        .insert(skelly_entity);



}




fn keyboard_animation_control(
    keyboard_input_res: Res<Input<KeyCode>>,
    animations: Res<VecAnimations>,
    mut query_animation: Query<&mut AnimationPlayer>,
    mut query_skelly: Query<(&mut Transform, &mut Skelly)>

) {
    if let Ok(mut player) = query_animation.get_single_mut() {
        let keyboard_input = &mut keyboard_input_res.into_inner();
        let (mut skelly_transform, mut skelly) = query_skelly.get_single_mut().unwrap();

        let pressed_keys = direction::get_pressed_keys_of_interest(keyboard_input);
        let mut vector_direction = Vec3::ZERO;
        let mut is_action = SkellyAnimationId::None;
        let mut is_shift = 0.0;

        if !skelly.is_ready(&mut player) {
            return;
        }

        for key in pressed_keys {
            info!("Keys pressed:{:#?}", key);
            match key {
                KeyCode::Up => {
                    vector_direction += Vec3::new(1.0, 0.0, 1.0);
                },
                KeyCode::Right => {
                    vector_direction += Vec3::new(-1.0, 0.0, 1.0)
                },
                KeyCode::Down => {
                    vector_direction += Vec3::new(-1.0, 0.0, -1.0);
                },
                KeyCode::Left => {
                    vector_direction += Vec3::new(1.0, 0.0, -1.0);
                },
                KeyCode::LShift => {
                    is_shift = 1.0;
                },
                KeyCode::Numpad1 => {
                    is_action = SkellyAnimationId::Yell;
                },
                KeyCode::Numpad2 => {
                    is_action = SkellyAnimationId::Attack;
                },
                KeyCode::Numpad3 => {
                    is_action = SkellyAnimationId::Fall;
                },
                KeyCode::Numpad4 => {
                    is_action = SkellyAnimationId::Hit;
                },
                KeyCode::Numpad5 => {
                    is_action = SkellyAnimationId::Die;
                },
                KeyCode::Numpad6 => {
                    is_action = SkellyAnimationId::Spawn;
                },
                KeyCode::Numpad7 => {
                    is_action = SkellyAnimationId::Hanged;
                },
                _ => {}
            }
        }

        // if Action, do it, than return
        if is_action != SkellyAnimationId::None {
            skelly.play_animation_action(&mut player, is_action);
            return;
            // match is_action {
            //     SkellyAnimationId::Yell => {
            //         skelly.play_animation_action(&mut player, SkellyAnimationId::Yell);
            //     },
            //     SkellyAnimationId::Attack => {
            //         skelly.play_animation_action(&mut player, SkellyAnimationId::Attack);
            //     }
            //     SkellyAnimationId::Fall => {
            //         skelly.play_animation_action(&mut player, SkellyAnimationId::Fall);
            //     },
            //     // SkellyAnimationId::Attack => {
            //     //     skelly.play_animation_action(&mut player, SkellyAnimationId::Attack);
            //     // },
            //     _ => {}
            // }
        }

        // If Moving, than move and return
        if vector_direction != Vec3::ZERO && skelly.can_move() {
            info!("Vector: {}", vector_direction);

            if vector_direction.x > 1.0 {
                vector_direction.x = 1.0;
            } else if vector_direction.x < -1.0 {
                vector_direction.x = -1.0;
            }

            if vector_direction.z > 1.0 {
                vector_direction.z = 1.0;
            } else if vector_direction.z < -1.0 {
                vector_direction.z = -1.0;
            }

            let animation_to_play =
                if is_shift == 1.0 {
                    SkellyAnimationId::Run
                } else {
                    SkellyAnimationId::Walk
                };

            skelly_transform.translation += vector_direction * ENTITY_SPEED * (1.0 + (is_shift  * 2.0)) ;
            skelly_transform.rotation = direction::map_vec3_to_quat(vector_direction).unwrap();

            if skelly.current_animation_id == SkellyAnimationId::Idle || skelly.current_animation_id != animation_to_play {
                animations.play(&mut player, animation_to_play, true);
                skelly.current_animation_id = animation_to_play;
            }

            return;

        } else {
            if skelly.current_animation_id == SkellyAnimationId::Walk
                || skelly.current_animation_id == SkellyAnimationId::Run {
                animations.play(&mut player, SkellyAnimationId::Idle, true);
                skelly.current_animation_id = SkellyAnimationId::Idle;
            }
        }

        // if no action is taken
        skelly.play_animation_idle(&mut player);
    }
}