use bevy::prelude::*;

mod chess_board;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(chess_board::Board);
    }
}