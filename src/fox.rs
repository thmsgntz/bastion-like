use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

pub struct FoxPlugin;
impl Plugin for FoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(link_animations)
            .add_system(keyboard_control);
    }
}

pub struct Animations {
    vec_anim: Vec<Handle<AnimationClip>>,
    entity: u32,
}

pub struct VecAnimations(Vec<Animations>);

#[derive(Component)]
struct AnimationDuration {
    time: Timer,
}

#[derive(Component)]
pub struct Creature;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Fox
    let id_fox = commands
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
            parent.spawn_scene(asset_server.load("models/Fox.glb#Scene0"));
        })
        .insert(Creature)
        .insert(AnimationDuration {
            time: Timer::new(Duration::from_secs(2), true),
        })
        .id();

    // Skeleton
    let id_sk = commands
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
            parent.spawn_scene(asset_server.load("models/skeleton/scene.gltf#Scene0"));
        })
        .insert(Creature)
        .insert(AnimationDuration {
            time: Timer::new(Duration::from_secs(2), true),
        })
        .id();

    info!("Fox id: {}", id_fox.id());
    info!("Ske id: {}", id_sk.id());

    // Insert a resource with the current scene information
    commands.insert_resource(VecAnimations(vec![
        Animations {
            vec_anim: vec![
                asset_server.load("models/Fox.glb#Animation2"),
                asset_server.load("models/Fox.glb#Animation1"),
                asset_server.load("models/Fox.glb#Animation0"),
            ],
            entity: id_fox.id(),
        },
        Animations {
            vec_anim: vec![
                asset_server.load("models/skeleton/scene.gltf#Animation1"),
                asset_server.load("models/skeleton/scene.gltf#Animation2"),
                asset_server.load("models/skeleton/scene.gltf#Animation3"),
            ],
            entity: id_sk.id(),
        },
    ]));
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
