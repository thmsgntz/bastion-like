use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;
use bevy::ecs::system::Command;
use bevy::scene::InstanceId;

pub struct FoxPlugin;
impl Plugin for FoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(link_animations)
            //.add_system(keyboard_control)
        ;
    }
}

pub struct Animations {
    vec_anim: Vec<Handle<AnimationClip>>,
    entity: u32,
}

pub struct VecAnimations(Vec<Animations>);

struct SceneHandle {
    handle: Handle<Scene>,
    vec_animations: Vec<Handle<AnimationClip>>,
    is_loaded: bool,
    creature_entity_id: Option<u32>, // creature id associated with this scene
    // has_camera: bool,
    // has_light: bool,
}

#[derive(Component)]
struct AnimationDuration {
    time: Timer,
}

#[derive(Component)]
pub struct Creature;

fn setup_fox(asset_server: &Res<AssetServer>, scene_path: &str) -> SceneHandle {

    let scene_handle = SceneHandle {
        handle: asset_server.load(scene_path),
        vec_animations: vec![
            asset_server.load("models/Fox.glb#Animation2"),
            asset_server.load("models/Fox.glb#Animation1"),
            asset_server.load("models/Fox.glb#Animation0"),
        ],
        is_loaded: false,
        creature_entity_id: None
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
        is_loaded: false,
        creature_entity_id: None
    };

    scene_handle
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Fox

    let mut fox_scene_handle = setup_fox(
        &asset_server,
        "models/Fox.glb#Scene0",
    );

   let fox_id =
        commands
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
                parent.spawn_scene(?);
            })
            .insert(Creature)
            .insert(AnimationDuration {
                time: Timer::new(Duration::from_secs(2), true),
            })
            .id();

    fox_scene_handle.creature_entity_id = Some(fox_id.id());

    /*let mut skelly_scene_handle = setup_fox(
        &asset_server,
        "models/skeleton/scene.gltf#Scene0",
    );

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
            parent.spawn_scene(skelly_scene_handle.handle);
        })
        .insert(Creature)
        .insert(AnimationDuration {
            time: Timer::new(Duration::from_secs(2), true),
        })
        .id();

    skelly_scene_handle.creature_entity_id = Some(skelly_id.id());*/
}

#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);

fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.0;
        } else {
            break;
        }
    }
    curr_entity
}

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy

    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        info!("Calling: link_animations. {:#?}", entity);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}

fn keyboard_control(
    mut player_query: Query<&mut AnimationPlayer>,
    animations: Res<VecAnimations>,
    mut query: Query<(Entity, &AnimationEntityLink, &mut AnimationDuration), With<Creature>>,
    time: Res<Time>,
) {
    for (entity, animation_entity, mut animation_duration) in query.iter_mut() {
        animation_duration.time.tick(time.delta());

        if animation_duration.time.finished() {
            info!(
                "keyboard: entity: {:#?}, animationEntity.0: {:#?}",
                entity, animation_entity.0
            );
            info!("timer Finished! ");
            animation_duration.time = Timer::new(Duration::from_secs(2), true);

            for entity_animations in &animations.0 {
                if entity_animations.entity == entity.id() {
                    info!("I found animations for entity : {}", entity.id());

                    if let Ok(mut player) = player_query.get_mut(animation_entity.0) {
                        let mut rng = rand::thread_rng();
                        let number = rng.gen_range(0..3);

                        let handle_animation = &entity_animations.vec_anim[number];

                        player.play(handle_animation.clone_weak()).repeat();

                        info!("Playing! animation number {}", number);
                    }
                }
            }
        }
    }
}
