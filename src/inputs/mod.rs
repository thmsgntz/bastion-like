mod keys_mapping;

use crate::{App, default, Plugin};
use crate::inputs::keys_mapping::InputMap;

pub struct InputsPlugin;
impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<InputMap>(Default::default());
    }
}