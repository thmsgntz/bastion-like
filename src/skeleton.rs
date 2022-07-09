use bevy::prelude::*;
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
    //facing: direction::Direction,
    animation_id: SkellyAnimationId
}

impl Default for Skelly {
    fn default() -> Self {
        Skelly {
            //facing: direction::Direction::default(),
            animation_id: SkellyAnimationId::Idle
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum SkellyAnimationId {
    Idle, // duration: 1.5800002
    LookingAround,  // duration: 3.1800003
    Attack,  // duration: 2.3200002
    Yell,  // duration: 1.5800002
    Walk,  // duration: 0.9800001
    Run,  // duration: 0.78000003
    None,
}

const SKELLY_ANIM_DURATION_YELL : f32 = 1.58;
const SKELLY_ANIM_DURATION_ATTACK : f32 = 2.32;


struct Animations(Vec<Handle<AnimationClip>>);


fn inspect_animation_clip(
    assets_handle: Res<Assets<AnimationClip>>,
    animations: Res<Animations>,
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
    assets_handle: Res<Assets<AnimationClip>>,
    animations: Res<Animations>,
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

impl Animations {
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("models/skeleton/scene.gltf#Animation1"),
        asset_server.load("models/skeleton/scene.gltf#Animation2"),
        asset_server.load("models/skeleton/scene.gltf#Animation3"),
        asset_server.load("models/skeleton/scene.gltf#Animation4"),
        asset_server.load("models/skeleton/scene.gltf#Animation5"),
        asset_server.load("models/skeleton/scene.gltf#Animation6"),
    ]));

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
        .insert(Skelly::default());
}




fn keyboard_animation_control(
    keyboard_input_res: Res<Input<KeyCode>>,
    animations: Res<Animations>,
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

        if skelly.animation_id == SkellyAnimationId::Yell {
            if player.elapsed() >= SKELLY_ANIM_DURATION_YELL {
                animations.play(&mut player, SkellyAnimationId::Idle, true);
                skelly.animation_id = SkellyAnimationId::Idle;
            }

            return;
        }

        if skelly.animation_id == SkellyAnimationId::Attack {
            if player.elapsed() >= SKELLY_ANIM_DURATION_ATTACK {
                animations.play(&mut player, SkellyAnimationId::Idle, true);
                skelly.animation_id = SkellyAnimationId::Idle;
            }

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
                _ => {}
            }
        }

        if is_action != SkellyAnimationId::None {
            match is_action {
                SkellyAnimationId::Yell => {
                    if skelly.animation_id != SkellyAnimationId::Yell
                    {
                        skelly.animation_id = SkellyAnimationId::Yell;
                        animations.play(&mut player, SkellyAnimationId::Yell, false);

                    }
                },
                SkellyAnimationId::Attack => {
                    if skelly.animation_id != SkellyAnimationId::Attack
                    {
                        skelly.animation_id = SkellyAnimationId::Attack;
                        animations.play(&mut player, SkellyAnimationId::Attack, false);
                    }
                }
                _ => {}
            }

            return;
        }



        // Moving
        if vector_direction != Vec3::ZERO && (
            skelly.animation_id == SkellyAnimationId::Idle
        ||  skelly.animation_id == SkellyAnimationId::Walk
        ||  skelly.animation_id == SkellyAnimationId::Run
        ) {
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

            if skelly.animation_id == SkellyAnimationId::Idle || skelly.animation_id != animation_to_play {
                animations.play(&mut player, animation_to_play, true);
                skelly.animation_id = animation_to_play;
            }

        } else {
            if skelly.animation_id == SkellyAnimationId::Walk
                || skelly.animation_id == SkellyAnimationId::Run {
                animations.play(&mut player, SkellyAnimationId::Idle, true);
                skelly.animation_id = SkellyAnimationId::Idle;
            }
        }

    }
}