use crate::creatures;
use crate::creatures::{Creature, HashMapAnimations};
use crate::direction::Direction;
use bevy::prelude::*;

pub struct SkellyPlugin;
impl Plugin for SkellyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(setup_scene_once_loaded);
    }
}

/// Player marker
#[derive(Component)]
pub(crate) struct Skelly;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut hash_animations = creatures::HashMapAnimations::default();

    hash_animations.add_animation(
        1,
        asset_server.load("models/skeleton/scene.gltf#Animation1"),
        1.58,
        true,
    );

    commands
        .spawn()
        .insert_bundle(creatures::Creature {
            hashmap_animations: hash_animations,
            transform: PbrBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    rotation: Quat::from_rotation_y(Direction::Up.get_angle()),
                    scale: Vec3::ONE * 0.6,
                },
                ..Default::default()
            },
        })
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/skeleton/scene.gltf#Scene0"));
        })
        .insert(Skelly);
}

fn setup_scene_once_loaded(
    mut animation_players: Query<(Entity, &mut AnimationPlayer)>,
    mut done_skelly: Local<i32>,
    mut query_skelly: Query<&HashMapAnimations, With<Skelly>>,
) {
    /*
        TODO: T'es là
        Deux créatures spawned entre skelly et mob.
        Par contre, impossible de comprendre comment avoir le bon player pour chaque creature

    */

    if *done_skelly < 2 {
        // if let Some(anim) = assets_handle.get(&hashmap_animations.0[0]) {
        //     info!("Inspection! 3");
        //     info!("Anim {:#?}", anim); // duration: 1.5800002
        // }
        for (e, mut player) in animation_players.iter_mut() {
            // if let Ok(mut player) = player.get_single_mut() {

            info!("scene with skelly : player found: {:#?} {:?} {}", e, e, *done_skelly);
            if let Ok(mut skelly_creature) = query_skelly.get_single_mut() {
                info!("  scene with skelly : HashMapAnimations found");
                if let Some(animation) = skelly_creature.get(1) {
                    info!("scene with skelly : player found: {:#?}", e);
                    player.play(animation.handle.clone_weak()).repeat();
                    *done_skelly += 1;
                }
            }
        }
    }
}

/*
Propre à Creature :
    Importer Animations


 */

/*
Propre à Skelly :
    Gestion des inputs pour player

 */
