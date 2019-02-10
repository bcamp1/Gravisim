use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;
use cam::Cam;

#[derive(Copy, Clone)]
pub struct Body {
    pub past_x: f32,
    pub past_y: f32,
    pub x: f32,
    pub y: f32,
    pub a_x: f32,
    pub a_y: f32,
    pub v_x: f32,
    pub v_y: f32,
    pub mass: f32,
    pub size: f32,
    pub color: (u8, u8, u8, u8),
}

impl Body {
    pub fn new() -> Body {
        Body {
            past_x: 0f32,
            past_y: 0f32,
            x: 0f32,
            y: 0f32,
            v_x: 0f32,
            v_y: 0f32,
            a_x: 0f32,
            a_y: 0f32,
            mass: 0f32,
            size: 0f32,
            color: (255, 255, 255, 255),
        }
    }

    pub fn update_self(&mut self) {
        self.past_x = self.x;
        self.past_y = self.y;

        self.x += self.v_x;
        self.y += self.v_y;

        self.v_x += self.a_x;
        self.v_y += self.a_y;
    }

    pub fn compute_gravity(&mut self, body: Body) {
        let direction = (body.x - self.x, body.y - self.y);
        let distance = ((body.x - self.x).powi(2) + (body.y - self.y).powi(2)).sqrt();
        let unit_direction = (direction.0 / distance, direction.1 / distance);
        let force_scalar = ::GRAVITY_CONST * self.mass * body.mass / distance.powi(2);
        let acc_scalar = force_scalar / self.mass;
        let acc_vector = (unit_direction.0 * acc_scalar, unit_direction.1 * acc_scalar);
        self.a_x += acc_vector.0;
        self.a_y += acc_vector.1;
    }

    pub fn render(&self, canvas: &mut WindowCanvas, cam: &Cam) {
        canvas.filled_circle((self.x - cam.x) as i16, (self.y - cam.y) as i16, self.size as i16, self.color);
    }
}
