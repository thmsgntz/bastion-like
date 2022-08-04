use crate::creatures;
use crate::creatures::{Creature, HashMapAnimations};
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
    let mut hash_animations = HashMapAnimations::default();

    hash_animations.add_animation(
        2,
        asset_server.load("models/golem/scene.gltf#Animation1"),
        3.18,
        true,
    );

    commands
        .spawn()
        .insert_bundle(Creature {
            hashmap_animations: hash_animations,
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

/*fn setup_scene_once_loaded(
    mut done: Local<bool>,
    mut player: Query<&mut AnimationPlayer>,
    mut query_creature: Query<&HashMapAnimations, Without<Skelly>>,
) {
    if !*done {
        // if let Some(anim) = assets_handle.get(&hashmap_animations.0[0]) {
        //     info!("Inspection! 3");
        //     info!("Anim {:#?}", anim); // duration: 1.5800002
        // }
        for mut player in player.iter_mut() {
            info!("scene without skelly : player found");
            if let Ok(mut hmanimations) = query_creature.get_single_mut() {
               info!("  scene without skelly : HashMapAnimations found");
                if let Some(animation) = hmanimations.get(2) {
                    player.play(animation.handle.clone_weak()).repeat();
                    *done = true;
                }
            }
        }
    }
}*/

/*
Propre à Creature :
    Importer Animations


 */

/*
Propre à Skelly :
    Gestion des inputs pour player

 */
