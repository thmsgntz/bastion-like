use crate::{Input, KeyCode, Vec3};
use std::f32::consts::PI;

pub const ENTITY_LOOKING_UP: f32 = 45.0 * PI / 180.0;
const ENTITY_LOOKING_UP_RIGHT: f32 = PI / 180.0;
const ENTITY_LOOKING_RIGHT: f32 = -45.0 * PI / 180.0;
const ENTITY_LOOKING_UP_LEFT: f32 = (2.0 * 45.0) * PI / 180.0;
const ENTITY_LOOKING_LEFT: f32 = (3.0 * 45.0) * PI / 180.0;
const ENTITY_LOOKING_DOWN: f32 = -(3.0 * 45.0) * PI / 180.0;
const ENTITY_LOOKING_DOWN_LEFT: f32 = -(4.0 * 45.0) * PI / 180.0;
const ENTITY_LOOKING_DOWN_RIGHT: f32 = -(2.0 * 45.0) * PI / 180.0;

#[derive(PartialEq, Copy, Clone)]
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

impl Direction {
    pub fn get_angle(&self) -> f32 {
        match self {
            Direction::Up => ENTITY_LOOKING_UP,
            Direction::UpRight => ENTITY_LOOKING_UP_RIGHT,
            Direction::Right => ENTITY_LOOKING_RIGHT,
            Direction::DownRight => ENTITY_LOOKING_DOWN_RIGHT,
            Direction::Down => ENTITY_LOOKING_DOWN,
            Direction::DownLeft => ENTITY_LOOKING_DOWN_LEFT,
            Direction::Left => ENTITY_LOOKING_LEFT,
            Direction::UpLeft => ENTITY_LOOKING_UP_LEFT,
        }
    }

    pub fn get_vec3(&self) -> Vec3 {
        match self {
            Direction::Up => Vec3::new(1.0, 0.0, 1.0),
            Direction::UpRight => Vec3::new(0.0, 0.0, 1.0),
            Direction::Right => Vec3::new(-1.0, 0.0, 1.0),
            Direction::DownRight => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Down => Vec3::new(-1.0, 0.0, -1.0),
            Direction::DownLeft => Vec3::new(0.0, 0.0, -1.0),
            Direction::Left => Vec3::new(1.0, 0.0, -1.0),
            Direction::UpLeft => Vec3::new(1.0, 0.0, 0.0),
        }
    }

    pub fn difference_angle(&self, direction: &Direction) -> f32 {
        self.get_angle() - direction.get_angle()
    }

    pub fn get_strict_opposed(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::UpRight => Direction::DownLeft,
            Direction::Right => Direction::Left,
            Direction::DownRight => Direction::UpLeft,
            Direction::Down => Direction::Up,
            Direction::DownLeft => Direction::UpRight,
            Direction::Left => Direction::Right,
            Direction::UpLeft => Direction::DownRight,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Down
    }
}

fn is_keyboard_any_arrow_pressed(keyboard_input: &Input<KeyCode>) -> bool {
    for key in keyboard_input.get_pressed() {
        match *key {
            KeyCode::Up | KeyCode::Right | KeyCode::Left | KeyCode::Down => return false,

            _ => {}
        }
    }
    true
}

static KEYS_ARROWS: &'static [KeyCode] = &[
    KeyCode::Up,
    KeyCode::Right,
    KeyCode::Left,
    KeyCode::Down,
    KeyCode::LShift,
];
static KEYS_ACTIONS: &'static [KeyCode] = &[
    KeyCode::Space,
    KeyCode::Numpad1,
    KeyCode::Numpad2,
    KeyCode::Numpad3,
    KeyCode::Numpad4,
    KeyCode::Numpad5,
    KeyCode::Numpad6,
    KeyCode::Numpad7,
    KeyCode::Numpad8,
    KeyCode::Numpad9,
];

pub fn get_pressed_keys_of_interest(keyboard_input: &Input<KeyCode>) -> Vec<KeyCode> {
    let key_filter = keyboard_input.get_pressed().filter(|x| {
        // match **x {
        //     KeyCode::Up | KeyCode::Right | KeyCode::Left | KeyCode::Down => return true,
        //     _ => {}
        // }
        if KEYS_ARROWS.contains(*x) || KEYS_ACTIONS.contains(*x) {
            return true;
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

pub fn map_vec3_to_direction(vec: Vec3) -> Result<Direction, String> {
    match vec.x as i8 {
        0 => match vec.z as i8 {
            1 | 2 => Ok(Direction::UpRight),
            -1 | -2 => Ok(Direction::DownLeft),
            _ => Err(String::from("vector_direction not equals to 1 or 2")),
        },
        1 | 2 => match vec.z as i8 {
            1 | 2 => Ok(Direction::Up),
            -1 | -2 => Ok(Direction::Left),
            0 => Ok(Direction::UpLeft),
            _ => Err(String::from("vector_direction not equals to 1 or 2")),
        },
        -1 | -2 => match vec.z as i8 {
            1 | 2 => Ok(Direction::Right),
            -1 | -2 => Ok(Direction::Down),
            0 => Ok(Direction::DownRight),
            _ => Err(String::from("vector_direction not equals to 1 or 2")),
        },
        _ => Err(String::from("vector_direction not equals to 1 or 2")),
    }
}
