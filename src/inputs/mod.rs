use bevy::prelude::*;

mod keys_mapping;

use crate::animations_handler::AddAnimation;
use crate::inputs::keys_mapping::InputMap;
use crate::{App, Plugin};

use crate::creatures;
use crate::creatures::CreatureTrait;



pub struct InputsPlugin;
impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<InputMap>(Default::default())
            .add_system(spawn_creature);
    }
}

fn spawn_creature(
    keyboard_input: Res<Input<KeyCode>>,
    command: Commands,
    asset_server: Res<AssetServer>,
    event_writer: EventWriter<AddAnimation>,
) {
    if keyboard_input.just_pressed(KeyCode::Numpad1) || keyboard_input.just_pressed(KeyCode::Key1) {
        creatures::skelly::Skelly::spawn(command, asset_server, event_writer);
        return
    }

    if keyboard_input.just_pressed(KeyCode::Numpad2) || keyboard_input.just_pressed(KeyCode::Key2) {
        creatures::mob::Gollum::spawn(command, asset_server, event_writer);
    }
}
