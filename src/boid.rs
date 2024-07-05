pub mod boid {
    const SPEED_LIMIT: f32 = 2.0; 
    const VISUAL_RANGE: f32 = 80.0; 
    const MIN_DISTANCE: f32 = 50.0;
    const WIN_WIDTH: f32 = 800.0;
    const WIN_HEIGHT: f32 = 600.0;
    use crate::Vec2;
    use rand::Rng;
   
    pub struct Boid {
        pub x: f32,
        pub y: f32,
        pub dx: f32,
        pub dy: f32,
       
    }

    impl Boid {
        pub fn new(win_width: f32, win_height: f32) -> Boid {
            let mut rng = rand::thread_rng();
            Boid {
                x: rng.gen::<f32>() * win_width / 2.0 + win_width / 4.0,
                y: rng.gen::<f32>() * win_height / 2.0 + win_height / 4.0,
                dx: (rng.gen::<f32>() - 0.5) * SPEED_LIMIT,
                dy: (rng.gen::<f32>() - 0.5) * SPEED_LIMIT,
               
            }
        }

        pub fn update(&mut self,boids : &Vec<&Boid>) {
            self.x += self.dx;
            self.y += self.dy;
            self.wrap_edges();

            let mut average_velocity = Vec2::new(0.0, 0.0);
            let mut average_position = Vec2::new(0.0, 0.0);
            let mut average_separation = Vec2::new(0.0, 0.0);
            let mut neighbours = 0.0;

            for other in boids {
                let distance = self.distance_to_other(other);
                if distance < VISUAL_RANGE {
                    average_velocity.x += other.dx;
                    average_velocity.y += other.dy;
                    average_position.x += other.x;
                    average_position.y += other.y;

                    if distance < MIN_DISTANCE {
                        let diff = Vec2::new(self.x - other.x, self.y - other.y);
                        average_separation += diff;
                    }
                    neighbours += 1.0;
                }
            }

            if neighbours > 0.0 {
                average_velocity /= neighbours;
                average_position /= neighbours;
                average_separation /= neighbours;

                self.dx += average_velocity.x * 0.02;
                self.dy += average_velocity.y * 0.02;
                self.dx += (average_position.x - self.x) * 0.01;
                self.dy += (average_position.y - self.y) * 0.01;
                self.dx -= average_separation.x * 0.03;
                self.dy -= average_separation.y * 0.03;
            }

            // Limit speed
            let speed = (self.dx.powi(2) + self.dy.powi(2)).sqrt();
            if speed > SPEED_LIMIT {
                self.dx = (self.dx / speed) * SPEED_LIMIT;
                self.dy = (self.dy / speed) * SPEED_LIMIT;
            }
        }

        fn wrap_edges(&mut self) {
            if self.x < 0.0 {
                self.x = WIN_WIDTH;
            } else if self.x > WIN_WIDTH {
                self.x = 0.0;
            }
            if self.y < 0.0 {
                self.y = WIN_HEIGHT;
            } else if self.y > WIN_HEIGHT {
                self.y = 0.0;
            }
        }

        fn distance_to_other(&self, boid: &Boid) -> f32 {
            let dx = self.x - boid.x;
            let dy = self.y - boid.y;
            (dx.powi(2) + dy.powi(2)).sqrt()
        }
    }
}
