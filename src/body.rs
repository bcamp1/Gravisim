use cam::Cam;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use vector2d::Vector2d;

#[derive(Copy, Clone)]
pub struct Body {
    pub prev_position: Vector2d,
    pub position: Vector2d,
    pub velocity: Vector2d,
    pub acceleration: Vector2d,
    pub mass: f32,
    pub size: f32,
    pub density: f32,
    pub color: (u8, u8, u8, u8),
}

impl Body {
    pub fn new(x: f32, y: f32, v_x: f32, v_y: f32, density: f32, size: f32) -> Body {
        Body {
            prev_position: Vector2d { x: 0f32, y: 0f32 },
            position: Vector2d { x: 0f32, y: 0f32 },
            velocity: Vector2d { x: 0f32, y: 0f32 },
            acceleration: Vector2d { x: 0f32, y: 0f32 },
            mass: 0f32,
            size: 0f32,
            density: 0f32,
            color: (255, 255, 255, 255),
        }
    }

    pub fn update_self(&mut self, mult: &f32) {
        self.prev_position = self.position;
        self.position.x += self.velocity.x * mult + 0.5 * self.acceleration.x * mult * mult;
        self.position.y += self.velocity.y * mult + 0.5 * self.acceleration.y * mult * mult;

        self.velocity.x += self.acceleration.x * mult;
        self.velocity.y += self.acceleration.y * mult;
    }

    pub fn compute_gravity(&mut self, body: Body, mult: &f32) {
        let min_distance = 0.0001;
        let direction = (
            body.position.x - self.position.x,
            body.position.y - self.position.y,
        );
        let mut distance = self.position.distance(body.position);
        if distance < min_distance {
            distance = min_distance;
        }
        let unit_direction = (direction.0 / distance, direction.1 / distance);
        let force_scalar = ::GRAVITY_CONST * self.mass * body.mass / distance.powi(2);
        let acc_scalar = force_scalar / self.mass;
        let acc_vector = (unit_direction.0 * acc_scalar, unit_direction.1 * acc_scalar);
        if distance + 5.0 >= self.size + body.size {
            self.acceleration.x += acc_vector.0;
            self.acceleration.y += acc_vector.1;
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, cam: &Cam) {
        // let color_g = if self.density > 255f32 {
        //     0 as u8
        // } else {
        //     (255f32 - self.density) as u8
        // };

        let t = cam.transform((self.position.x, self.position.y));
        match canvas.filled_circle(
            t.0 as i16,
            t.1 as i16,
            (self.size * cam.zoom) as i16,
            self.color,
        ) {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
