pub mod Boid
{
    const SPEED_LIMIT: f32 = 400.0; 
    const VISUAL_RANGE: f32 = 32.0; 
    const MIN_DISTANCE: f32 = 16.0; 
    use rand::*;
    use ggez::graphics::{self, Color};
pub struct Boid
{
    pub x : f32,
    pub y :f32,
    pub dx : f32,
    pub dy : f32,
    pub color : Color

}
 pub impl Boid
 {
    
    
    pub fn new(win_width: f32, win_height: f32) -> Boid
    {
        Boid
        {
            x : (rand::random::<f32>() * win_width / 2.0 + win_width / 4.0),
            y: (rand::random::<f32>() * win_height/ 2.0 + win_width / 4.0),
            dx: (rng.gen() - 0.5)* SPEED_LIMIT,
            dy: (rng.gen() - 0.5)* SPEED_LIMIT,
            color: 
            Color::from_rgb((rand::random::<u8>() * 128.0 + 128.0) / 255.0,
            (rand::random::<u8>() * 128.0 + 128.0) / 255.0,
            (rand::random::<u8>() * 128.0 + 128.0) / 255.0,
        )
                
            
        }
    }
    
 }
    fn seperated()
    {

    }
    pub fn update()
    {

    }
    fn setPosition()
    {

    }
    fn matchVelocity()
    {

    }
    
    
}