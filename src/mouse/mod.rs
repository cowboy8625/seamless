#[derive(Debug)]
pub enum ScreenSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct Mouse {
    pub screen_side: ScreenSide,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
