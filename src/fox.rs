use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;
use crate::animations_handler;
use crate::animations_handler::{AnimationDuration, AnimationEntityLink, ChangeAnimation, SceneHandle, VecSceneHandle};

pub struct FoxPlugin;
impl Plugin for FoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(animations_handler::link_animations)
            .add_system(keyboard_control);
    }
}

#[derive(Component)]
pub struct Creature(String);

fn setup_fox(asset_server: &Res<AssetServer>, scene_path: &str) -> SceneHandle {
    let asset_scene_handle = asset_server.load(scene_path);

    let scene_handle = SceneHandle {
        handle: asset_scene_handle,
        vec_animations: vec![
            asset_server.load("models/Fox.glb#Animation2"),
            asset_server.load("models/Fox.glb#Animation1"),
            asset_server.load("models/Fox.glb#Animation0"),
        ],
        creature_entity_id: None,
    };

    scene_handle
}

fn setup_skelly(asset_server: &Res<AssetServer>, scene_path: &str) -> SceneHandle {
    let asset_scene_handle = asset_server.load(scene_path);

    let scene_handle = SceneHandle {
        handle: asset_scene_handle,
        vec_animations: vec![
            asset_server.load("models/skeleton/scene.gltf#Animation1"),
            asset_server.load("models/skeleton/scene.gltf#Animation2"),
            asset_server.load("models/skeleton/scene.gltf#Animation3"),
        ],
        creature_entity_id: None,
    };

    scene_handle
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Fox

    let mut fox_scene_handle = setup_fox(&asset_server, "models/Fox.glb#Scene0");

    let fox_id = commands
        .spawn()
        .insert_bundle(PbrBundle {
            transform: Transform {
                translation: Vec3::new(4.0, 0.0, 4.0),
                scale: Vec3::ONE * 0.01,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_scene(fox_scene_handle.handle.clone());
        })
        .insert(Creature(String::from("Fox")))
        .insert(AnimationDuration {
            time: Timer::new(Duration::from_secs(2), true),
        })
        .id();

    fox_scene_handle.creature_entity_id = Some(fox_id.id());

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
        .insert(AnimationDuration {
            time: Timer::new(Duration::from_secs(2), true),
        })
        .id();

    skelly_scene_handle.creature_entity_id = Some(skelly_id.id());

    commands.insert_resource(VecSceneHandle(vec![fox_scene_handle, skelly_scene_handle]));
}

fn keyboard_control(
    time: Res<Time>,
    scene_handlers: Res<VecSceneHandle>,
    mut player_query: Query<&mut AnimationPlayer>,
    mut query: Query<(Entity, &AnimationEntityLink, &mut AnimationDuration), With<Creature>>,
    mut event_writer: EventWriter<ChangeAnimation>
) {
    // TODO:
    // pour simplifier :
    //   - Créer un Event qui contient l'id de la créature à animer + index de l'animation
    //   - Faire une fonction un peu comme celle ci, les mêmes query qui :
    //          - récupère l'event et retrouve la créature par son id
    //          - récupère son player et joue l'animation à l'index donné
    // animation_entity contient l'entity id de chaque AnimationPlayer
    for (entity, animation_entity, mut animation_duration) in query.iter_mut() {
        animation_duration.time.tick(time.delta());

        if animation_duration.time.finished() {
            info!(
                "keyboard: entity: {:#?}, animationEntity.0: {:#?}",
                entity, animation_entity.0
            );
            info!("timer Finished! ");
            animation_duration.time = Timer::new(Duration::from_secs(2), true);

            for scene_handler in &scene_handlers.0 {
                if scene_handler.creature_entity_id == Some(entity.id()) {
                    info!("I found animations for entity : {}", entity.id());

                    let mut rng = rand::thread_rng();
                    let number = rng.gen_range(0..3);

                    //let handle_animation = &scene_handler.vec_animations[number];
                    //player.play(handle_animation.clone_weak()).repeat();
                    event_writer.send(
                        ChangeAnimation{
                            target: entity,
                            index: number as usize,
                            repeat: true,
                        }
                    );

                    info!("Sending event!");

                }
            }
        }
    }
}
