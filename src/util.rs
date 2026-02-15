#[derive(Debug)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl Vec2 {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
