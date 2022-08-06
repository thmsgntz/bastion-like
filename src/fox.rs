use bevy::prelude::*;

pub struct FoxPlugin;
impl Plugin for FoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(link_animations)
            .add_system(sync_animation_to_velocity);
    }
}
struct Animations(Vec<Handle<AnimationClip>>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("models/Fox.glb#Animation2"),
        asset_server.load("models/Fox.glb#Animation1"),
        asset_server.load("models/Fox.glb#Animation0"),
    ]));

    commands.insert_resource(Animations(vec![
        asset_server.load("models/skeleton/scene.gltf#Animation1"),
        asset_server.load("models/skeleton/scene.gltf#Animation2"),
        asset_server.load("models/skeleton/scene.gltf#Animation3"),
    ]));

    // Fox
    scene_spawner.spawn(asset_server.load("models/Fox.glb#Scene0"));
    scene_spawner.spawn(asset_server.load("models/skeleton/scene.gltf#Scene0"));

}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut animation_players: Query<(Entity, &mut AnimationPlayer)>,
    mut done: Local<u8>,
) {
    if *done < 2 {
        for (entity, mut player) in animation_players.iter_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done += 1;
        }
    }
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
    info!("Calling: link_animations.");
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

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

pub fn sync_animation_to_velocity(
    mut player_query: Query<&mut AnimationPlayer>,
    //mut query: Query<(&Movable, &mut AnimationConfig, &AnimationEntityLink), Changed<Movable>>,
    animations: Res<Animations>,
    //game_speed: Res<GameSpeed>,
) {
    for (movable, mut animation_config, animation_entity) in query.iter_mut() {
        if let Ok(mut player) = player_query.get_mut(animation_entity.0) {
//Stuff
        }
    }
}
