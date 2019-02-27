use std::ops;
#[derive(Copy, Clone)]
pub struct Vector2d {
    pub x: f32,
    pub y: f32,
}

impl Vector2d {
    pub fn print(&self) {
        println!("Vector2d x: {}, y: {}", self.x, self.y);
    }

    pub fn inner(&self, v: Vector2d) -> f32 {
        // Calculates the dot product of two vectors
        (v.x * self.x) + (v.y * self.y)
    }

    pub fn distance(&self, v: Vector2d) -> f32 {
        ((self.x - v.x).powi(2) + (self.y - v.y).powi(2)).sqrt()
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl ops::Add<Vector2d> for Vector2d {
    type Output = Vector2d;
    fn add(self, _rhs: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Sub<Vector2d> for Vector2d {
    type Output = Vector2d;
    fn sub(self, _rhs: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl ops::Mul<Vector2d> for Vector2d {
    type Output = Vector2d;
    fn mul(self, _rhs: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        }
    }
}

impl ops::Mul<f32> for Vector2d {
    type Output = Vector2d;
    fn mul(self, _rhs: f32) -> Vector2d {
        Vector2d {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

impl ops::Mul<Vector2d> for f32 {
    type Output = Vector2d;
    fn mul(self, _rhs: Vector2d) -> Vector2d {
        Vector2d {
            x: _rhs.x * self,
            y: _rhs.y * self,
        }
    }
}

impl ops::Div<f32> for Vector2d {
    type Output = Vector2d;
    fn div(self, _rhs: f32) -> Vector2d {
        Vector2d {
            x: self.x / _rhs,
            y: self.y / _rhs,
        }
    }
}
