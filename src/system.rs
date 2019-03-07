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

    pub fn update(&mut self, mult: &f32, total_time: &f32, elastic_collisions: bool) {
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
                    self.bodies[i].compute_gravity(body_j, mult);
                    let body_i = self.bodies[i];

                    // Collisions
                    let rad_sum = (body_i.size + body_j.size).powi(2);
                    let distance = (body_i.x - body_j.x).powi(2) + (body_i.y - body_j.y).powi(2);
                    let is_collided = distance <= rad_sum;

                    if is_collided {
                        if elastic_collisions {
                            // vf2 = a*v1 - b*v2
                            // vf1 = b*v1 + c*v2
                            // a = (2*m1) / (m1 + m2)
                            // b = (m1 - m2) / (m1 + m2)
                            // c = (2*m2) / (m1 + m2)

                            let m1 = body_i.mass;
                            let v1_x = body_i.v_x;
                            let v1_y = body_i.v_y;

                            let m2 = body_j.mass;
                            let v2_x = body_j.v_x;
                            let v2_y = body_j.v_y;

                            let a = (2.0 * m1) / (m1 + m2);
                            let b = (m1 - m2) / (m1 + m2);
                            let c = (2.0 * m2) / (m1 + m2);

                            let new_v1_x = b*v1_x + c*v2_x;
                            let new_v1_y = b*v1_y + c*v2_y;

                            let new_v2_x = a*v1_x - b*v2_x;
                            let new_v2_y = a*v1_y - b*v2_y;

                            self.bodies[i].v_x = new_v1_x;
                            self.bodies[i].v_y = new_v1_y;
                            self.bodies[j].v_x = new_v2_x;
                            self.bodies[j].v_y = new_v2_y;
                        } else {
                            let (bigger_index, smaller_index) = if body_i.size >= body_j.size {
                                (i, j)
                            } else {
                                (j, i)
                            };
                            to_remove.push(smaller_index);
                            self.bodies[bigger_index].mass += self.bodies[smaller_index].mass;
                            self.bodies[bigger_index].size = (self.bodies[bigger_index].mass * (3.0 / (4.0 * ::PI * self.bodies[bigger_index].density))).powf(1.0 / 3.0);

                            // Conservation of Momentum
                            let bigger = self.bodies[bigger_index];
                            let smaller = self.bodies[smaller_index];
                            let m1 = bigger.mass;
                            let v1_x = bigger.v_x;
                            let v1_y = bigger.v_y;

                            let m2 = smaller.mass;
                            let v2_x = smaller.v_x;
                            let v2_y = smaller.v_y;

                            self.bodies[bigger_index].v_x = (m1*v1_x + m2*v2_x) / (m1 + m2);
                            self.bodies[bigger_index].v_y = (m1*v1_y + m2*v2_y) / (m1 + m2);
                        }
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
