pub struct Cam {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Cam {
    pub fn new() -> Cam {
        Cam {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }
}
