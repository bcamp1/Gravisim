use body::Body;
use sdl2::render::WindowCanvas;
use cam::Cam;

pub struct System {
    pub bodies: Vec<Body>,
}

impl System {
    pub fn new() -> Self {
        System {
            bodies: vec!()
        }
    }

    pub fn add(&mut self, x: f32, y: f32, v_x: f32, v_y: f32, density: f32, size: f32) {
        let mut body = Body::new();
        body.x = x;
        body.y = y;
        body.v_x = v_x;
        body.v_y = v_y;
        body.mass = (4.0 / 3.0) * ::PI * size.powi(3) * density;
        body.size = size;
        self.bodies.push(body);
    }

    pub fn update(&mut self, mult: &f32) {
        let mut to_remove: Vec<usize> = vec!();

        for i in 0..self.bodies.len() {
            if to_remove.contains(&(i as usize)) {
                continue;
            }


            // Compute Gravity
            self.bodies[i].a_x = 0.0;
            self.bodies[i].a_y = 0.0;

            for j in 0..self.bodies.len() {
                if to_remove.contains(&(j as usize)) {
                    continue;
                }
                if i != j {
                    let body_j = self.bodies[j];
                    self.bodies[i].compute_gravity(body_j);
                    let body_i = self.bodies[i];

                    // Collisions
                    let rad_sum = (body_i.size + body_j.size).powi(2);
                    let distance = (body_i.x - body_j.x).powi(2) + (body_i.y - body_j.y).powi(2);
                    let is_collided = distance <= rad_sum;

                    if is_collided {
                        let bigger_index = if body_i.size >= body_j.size {
                            i
                        } else {
                            j
                        };
                        let smaller_index = if body_i.size >= body_j.size {
                            j
                        } else {
                            i
                        };

                        to_remove.push(smaller_index);
                        self.bodies[bigger_index].mass += self.bodies[smaller_index].mass;
                        self.bodies[bigger_index].size = (self.bodies[bigger_index].mass * (3.0 / (4.0 * ::PI))).powf(1.0 / 3.0);
                    }
                }
            }
            // Update Self
            self.bodies[i].update_self(mult);
        }

        // Remove bodies
        let mut removed = 0;
        for i in to_remove {
            self.bodies.remove((i - removed) as usize);
            removed += 1;
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, cam: &Cam) {
        for i in 0..self.bodies.len() {
            self.bodies[i].render(canvas, cam);
        }
    }
}
