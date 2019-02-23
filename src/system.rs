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
        let body = Body::new(x, y, v_x, v_y, density, size);
        self.bodies.push(body);
    }

    pub fn update(&mut self, mult: &f32, total_time: &f32) {
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
                        let (bigger_index, smaller_index) = if body_i.size >= body_j.size {
                            (i, j)
                        } else {
                            (j, i)
                        };

                        to_remove.push(smaller_index);
                        self.bodies[bigger_index].mass += self.bodies[smaller_index].mass;
                        self.bodies[bigger_index].size = (self.bodies[bigger_index].mass * (3.0 / (4.0 * ::PI * self.bodies[bigger_index].density))).powf(1.0 / 3.0);
                    }
                }
            }
            // Update Self
            self.bodies[i].update_self(mult, total_time);
        }

        // Remove bodies
        let mut remaining_bodies: Vec<Body> = self.bodies
            .drain(..)
            .enumerate()
            .filter_map(|(index, body)| {
                if to_remove.contains(&index) { None }
                else { Some(body) }
            })
            .collect();
        self.bodies.append(&mut remaining_bodies);
    }

    pub fn render(&self, canvas: &mut WindowCanvas, cam: &Cam) {
        self.bodies.iter().for_each(|body| body.render(canvas, cam));
    }
}
