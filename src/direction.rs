use std::f32::consts::PI;
use crate::{Input, KeyCode, Quat, Vec3};

pub const ENTITY_LOOKING_UP: f32 = 45.0 * PI/180.0;
const ENTITY_LOOKING_UP_RIGHT: f32 = PI/180.0;
const ENTITY_LOOKING_RIGHT: f32 = -45.0 * PI/180.0;
const ENTITY_LOOKING_UP_LEFT: f32 = (2.0 * 45.0) * PI/180.0;
const ENTITY_LOOKING_LEFT: f32 = (3.0 * 45.0) * PI/180.0;
const ENTITY_LOOKING_DOWN: f32 = -(3.0 * 45.0) * PI/180.0;
const ENTITY_LOOKING_DOWN_LEFT: f32 = -(4.0 * 45.0) * PI/180.0;
const ENTITY_LOOKING_DOWN_RIGHT: f32 = -(2.0 * 45.0) * PI/180.0;




#[derive(PartialEq, Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Down
    }
}

fn is_keyboard_any_arrow_pressed (keyboard_input: &Input<KeyCode>) -> bool {



    for key in keyboard_input.get_pressed() {
        match *key {
            KeyCode::Up | KeyCode::Right | KeyCode::Left | KeyCode::Down => return false,

            _ => {}
        }
    }
    true
}

pub fn get_keyboard_arrows_pressed(keyboard_input: &Input<KeyCode>) -> Vec<KeyCode> {

    let key_filter = keyboard_input
        .get_pressed()
        .filter(|x| {
            match **x {
                KeyCode::Up | KeyCode::Right | KeyCode::Left | KeyCode::Down => return true,
                _ => {}
            }
            return false;
        });
    let mut vec = <Vec<KeyCode>>::new();

    // key_filter.flatten().collect::<Vec<KeyCode>>()
    for x in key_filter {
        vec.push(*x)
    }
    vec
}


pub fn map_vec3_to_quat(vec: Vec3) -> Option<Quat> {
    match vec.x as i8 {
        0 => match vec.z as i8 {
            1 | 2  => Some(Quat::from_rotation_y(ENTITY_LOOKING_UP_RIGHT)),
            -1 | -2 => Some(Quat::from_rotation_y(ENTITY_LOOKING_DOWN_LEFT)),
            _ => None
        },
        1 | 2 => match vec.z as i8 {
            1 | 2  => Some(Quat::from_rotation_y(ENTITY_LOOKING_UP)),
            -1 | -2 => Some(Quat::from_rotation_y(ENTITY_LOOKING_LEFT)),
            0 => Some(Quat::from_rotation_y(ENTITY_LOOKING_UP_LEFT)),
            _ => None
        }
        -1 | -2 => match vec.z as i8 {
            1 | 2  => Some(Quat::from_rotation_y(ENTITY_LOOKING_RIGHT)),
            -1 | -2 => Some(Quat::from_rotation_y(ENTITY_LOOKING_DOWN)),
            0 => Some(Quat::from_rotation_y(ENTITY_LOOKING_DOWN_RIGHT)),
            _ => None
        }
        _ => None
    }
}
