use body::Body;
use cam::Cam;
use rand::Rng;
use sdl2::render::WindowCanvas;
use vector2d::Vector2d;

pub struct System {
    pub bodies: Vec<Body>,
}

impl System {
    pub fn new() -> Self {
        System { bodies: vec![] }
    }

    pub fn add(&mut self, x: f32, y: f32, v_x: f32, v_y: f32, density: f32, size: f32) {
        let mut body = Body::new();
        body.position = Vector2d { x: x, y: y };
        body.velocity = Vector2d { x: v_x, y: v_y };
        body.mass = (4.0 / 3.0) * ::PI * size.powi(3) * density;
        body.size = size;
        body.density = density;
        let mut rng = rand::thread_rng();
        let color = (
            rng.gen_range(1, 255),
            rng.gen_range(1, 255),
            rng.gen_range(1, 255),
            255,
        );
        println!("{:#?}", color);
        body.color = color;
        self.bodies.push(body);
    }

    pub fn update(&mut self, mult: &f32) {
        let mut to_remove: Vec<usize> = vec![];

        for i in 0..self.bodies.len() {
            if to_remove.contains(&(i as usize)) {
                continue;
            }

            // Compute Gravity
            self.bodies[i].acceleration.x = 0.0;
            self.bodies[i].acceleration.y = 0.0;

            for j in 0..self.bodies.len() {
                if to_remove.contains(&(j as usize)) {
                    continue;
                }
                if i != j {
                    let body_j = self.bodies[j];
                    self.bodies[i].compute_gravity(body_j);
                    let body_i = self.bodies[i];

                    // Collisions
                    let rad_sum = body_i.size + body_j.size;
                    let mut distance = body_i.position.distance(body_j.position);
                    let is_collided = distance <= rad_sum;

                    if is_collided {
                        let bigger_index = if body_i.size >= body_j.size { i } else { j };
                        let smaller_index = if body_i.size >= body_j.size { j } else { i };

                        let mut bigger_body = self.bodies[bigger_index];
                        let mut smaller_body = self.bodies[smaller_index];

                        // Reset the positions to a more accurate state
                        while smaller_body.position.distance(bigger_body.position) < rad_sum {
                            let unit_vector = smaller_body.position - bigger_body.position;
                            let unit_vector = unit_vector / (unit_vector.magnitude());
                            // let unit_vector_bigger =
                            //     smaller_body.velocity / (smaller_body.velocity.magnitude());
                            // let unit_vector_smaller = unit_vector_bigger * -1.0;
                            // // let smaller_magnitude = smaller_body.position.magnitude();
                            // // let d = smaller_body.position.distance(bigger_body.position);
                            smaller_body.position = smaller_body.position + unit_vector * 0.05;
                            // bigger_body.position = bigger_body.position + unit_vector_bigger * 0.01;
                            // bigger_body.position = bigger_body.position + unit_vector_bigger * 0.05;
                            // smaller_body.position = smaller_body.position * 1.0001;
                        }

                        // Pay attention to the velocity vector of the two bodies to calculate the result
                        // https://en.wikipedia.org/wiki/Elastic_collision
                        // v1 = smaller body, v2 = bigger body
                        // 'v1 = v1 - (2m2 / m1 + m2) * <v1 - v2, x1 - x2> / || x1 - x2 ||^2 * (x1 - x2)
                        // 'v2 = v2 - (2m1 / m1 + m2) * <v2 - v1, x2 - x1> / || x1 - x2 ||^2 * (x2 - x1)

                        let smaller_new_velocity = smaller_body.velocity
                            - ((2.0 * bigger_body.mass) / (smaller_body.mass + bigger_body.mass))
                                * ((smaller_body.velocity - bigger_body.velocity)
                                    .inner(smaller_body.position - bigger_body.position)
                                    / (smaller_body.position - bigger_body.position).magnitude())
                                * (smaller_body.position - bigger_body.position)
                                * 0.01; // Dampen the effect of the elasticity

                        let bigger_new_velocity = bigger_body.velocity
                            - ((2.0 * smaller_body.mass) / (smaller_body.mass + bigger_body.mass))
                                * ((bigger_body.velocity - smaller_body.velocity)
                                    .inner(bigger_body.position - smaller_body.position)
                                    / (bigger_body.position - smaller_body.position).magnitude())
                                * (bigger_body.position - smaller_body.position)
                                * 0.01;

                        smaller_body.velocity = smaller_new_velocity;
                        bigger_body.velocity = bigger_new_velocity;

                        // Update the two bodies
                        self.bodies[bigger_index] = bigger_body;
                        self.bodies[smaller_index] = smaller_body;
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
