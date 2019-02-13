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

    pub fn transform(&self, point: (f32, f32)) -> (f32, f32) {
        return (point.0 * self.zoom - self.x, point.1 * self.zoom - self.y);
    }

    pub fn reverse_transform(&self, t: (f32, f32)) -> (f32, f32) {
        return ((t.0 + self.x) / self.zoom, (t.1 + self.y) / self.zoom);
    }
}
