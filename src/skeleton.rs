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

#[derive(PartialEq)]
enum SkellyAnimationId {
    Idle,
    LookingAround,
    Attack,
    Yell,
    Walk,
    Run,
}

struct Animations(Vec<Handle<AnimationClip>>);

impl Animations {
    pub fn play (&self, player : &mut AnimationPlayer, id_animation: SkellyAnimationId) {
        player.play(self.0[id_animation as usize].clone_weak()).repeat();
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

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
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

        let vec_input = direction::get_keyboard_arrows_pressed(keyboard_input);
        let mut vector_direction = Vec3::ZERO;

        for x in vec_input {
            match x {
                KeyCode::Up => {
                    vector_direction += Vec3::new(1.0, 0.0, 1.0);
                }
                KeyCode::Right => {
                    vector_direction += Vec3::new(-1.0, 0.0, 1.0)

                }
                KeyCode::Down => {
                    vector_direction += Vec3::new(-1.0, 0.0, -1.0);

                }
                KeyCode::Left => {
                    vector_direction += Vec3::new(1.0, 0.0, -1.0);

                }
                _ => {}
            }
        }

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

        if vector_direction != Vec3::ZERO {
            info!("Vector: {}", vector_direction);

            skelly_transform.translation += vector_direction * ENTITY_SPEED;
            skelly_transform.rotation = direction::map_vec3_to_quat(vector_direction).unwrap();

            if skelly.animation_id == SkellyAnimationId::Idle {
                animations.play(&mut player, SkellyAnimationId::Walk);
                skelly.animation_id = SkellyAnimationId::Walk;
            }

        } else {
            animations.play(&mut player, SkellyAnimationId::Idle);
            skelly.animation_id = SkellyAnimationId::Idle;
        }

    }
}