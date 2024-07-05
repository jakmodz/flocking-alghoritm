mod boid;

use boid::boid::Boid;
use macroquad::prelude::*;
use std::time::Duration;

fn window_conf() -> Conf 
{
    let window_width = 800;
    let window_height = 600;
    Conf {
        window_title: "My Game".to_string(),
        window_width,
        window_height,
        ..Default::default() 
    }
}
#[macroquad::main("BasicShapes")]

async fn main() 
{
  
    const TARGET_FPS: f64 = 60.0;
    const TARGET_FRAME_TIME: f64 = 1.8 / TARGET_FPS;
    let window_width = 800;
    let window_height = 600;


 

   
   
    let mut  boids: Vec<Boid> = Vec::new();
    for _i  in  0..100  
    {
        boids.push(Boid::new(window_width as f32, window_height as f32));
    }
    loop {
        let start_time = get_time();
        clear_background(BLACK);

        for i in 0..boids.len()
       {
        let (left, right) = boids.split_at_mut(i);
        let (boid, right) = right.split_first_mut().unwrap();
        let all_boids: Vec<_> = left.iter().chain(right.iter()).collect();
        draw_circle(boid.x,boid.y, 6.0, RED);    
        boid.update(&all_boids);
       }

      

        next_frame().await;
        let elapsed = get_time() - start_time;
        let target_frame_time = Duration::from_secs_f64(1.0 / 60.0); // Target 60 FPS

        let elapsed = get_time() - start_time;
        let target_frame_time = Duration::from_secs_f64(1.0 / 60.0); // Target 60 FPS

        // Sleep to limit the frame rate
        let elapsed = get_time() - start_time;

        // Limit frame rate to TARGET_FPS
        if elapsed < TARGET_FRAME_TIME {
            let sleep_time = (TARGET_FRAME_TIME - elapsed) as f32;
            std::thread::sleep(std::time::Duration::from_secs_f32(sleep_time));
        }
    }
    }
    
