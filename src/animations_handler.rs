use crate::fox::Creature;
use bevy::prelude::*;

pub struct AnimationHandler;
impl Plugin for AnimationHandler {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeAnimation>()
            .add_system_to_stage(CoreStage::PostUpdate, change_animation);
    }
}

#[derive(Debug)]
pub struct ChangeAnimation {
    pub(crate) target: Entity,
    pub(crate) index: usize,
    pub(crate) repeat: bool
}

pub struct VecSceneHandle(pub Vec<SceneHandle>);

pub struct SceneHandle {
    pub handle: Handle<Scene>,
    pub vec_animations: Vec<Handle<AnimationClip>>,
    //is_loaded: bool,
    pub creature_entity_id: Option<u32>, // creature id associated with this scene
}

#[derive(Component)]
pub struct AnimationDuration {
    pub time: Timer,
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
    // Get all the Animation players which can be deep and hidden in the hierarchy

    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        info!("Calling: link_animations. {:#?}", entity);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animations players for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}

fn change_animation(
    mut events: EventReader<ChangeAnimation>,
    scene_handlers: Res<VecSceneHandle>,
    mut query_player: Query<&mut AnimationPlayer>,
    mut query_entity: Query<(Entity, &AnimationEntityLink), With<Creature>>,
) {
    for event in events.iter() {
        // retrouver l'entity
        info!("Event found! {:#?}", event);
        for (entity, animation_link) in query_entity.iter_mut() {
            if entity.id() == event.target.id() {
                for scene_handler in &scene_handlers.0 {
                    if scene_handler.creature_entity_id == Some(entity.id()) {
                        if let Ok(mut player) = query_player.get_mut(animation_link.0) {
                            if event.repeat {
                                player.play(scene_handler.vec_animations[event.index].clone_weak());
                            } else {
                                player.play(scene_handler.vec_animations[event.index].clone_weak()).repeat();
                            }
                        }
                    }
                }
            }
        }
    }
}
