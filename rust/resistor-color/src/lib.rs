use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Eq, Copy, Clone)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    use ResistorColor::*;
    match color {
        Black => 0,
        Brown => 1,
        Red => 2,
        Orange => 3,
        Yellow => 4,
        Green => 5,
        Blue => 6,
        Violet => 7,
        Grey => 8,
        White => 9,
    }
}

pub fn value_to_color_string(value: u32) -> String {
    match value {
        0 => "Black".to_owned(),
        1 => "Brown".to_owned(),
        2 => "Red".to_owned(),
        3 => "Orange".to_owned(),
        4 => "Yellow".to_owned(),
        5 => "Green".to_owned(),
        6 => "Blue".to_owned(),
        7 => "Violet".to_owned(),
        8 => "Grey".to_owned(),
        9 => "White".to_owned(),
        _ => "value out of range".to_owned(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut vec = Vec::new();

    for color in ResistorColor::iter() {
        vec.push(color);
    }

    vec.sort_by_key(|color| color_to_value(color.clone()));

    vec
}
