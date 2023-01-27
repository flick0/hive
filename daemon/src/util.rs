pub struct Anchor {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}
pub enum NumWithUnit {
    Percent(f32),
    Pixels(i32),
}

impl NumWithUnit {
    pub fn pixels_relative_to(&self, max: i32) -> i32 {
        match *self {
            NumWithUnit::Percent(n) => ((max as f64 / 100.0) * n as f64) as i32,
            NumWithUnit::Pixels(n) => n,
        }
    }

    pub fn perc_relative_to(&self, max: i32) -> f32 {
        match *self {
            NumWithUnit::Percent(n) => n,
            NumWithUnit::Pixels(n) => ((n as f64 / max as f64) * 100.0) as f32,
        }
    }
}

pub struct Coord {
    pub x: NumWithUnit,
    pub y: NumWithUnit,
}

pub struct Position {
    pub anchor: Anchor,
    pub offset_x: NumWithUnit,
    pub offset_y: NumWithUnit,  
}