//use std::io;
use macroquad::{prelude::*, input};
use macroquad::experimental::scene::*;

use macroquad::experimental::{
    collections::storage,
    scene::{Node, RefMut},
};

struct Calculator 
{
    // collider: Actor,
    // speed: Vec2,
}

struct Pan
{
    pan: PanType
}

impl Calculator {
    // pub const JUMP_SPEED: f32 = -700.0;
    // pub const GRAVITY: f32 = 2000.0;
    // pub const MOVE_SPEED: f32 = 300.0;

    fn new() -> Calculator 
    {
        // let mut resources = storage::get_mut::<Resources>().unwrap();

        Calculator 
        {
            // fn calculate_area_pan_rect(&self, pan_width: f32, pan_length: f32)->f32
            // {

            // }
        //     collider: resources.physics.add_actor(vec2(200.0, 100.0), 36, 66),
        //     speed: vec2(0., 0.),
        // }
        }
    }
}

// use macroquad::experimental::{
//     collections::storage,
//     scene::{Node, RefMut},
// };

// fn main() 
// {
//     // println!("What is your name?");
//     // let mut input = String::new();
//     // io::stdin().read_line(&mut input).unwrap();
//     // println!("Hello, {}!", input.trim());
//     println!("Area: {}", calculate_area_pan_circle(8.0));

// }


#[macroquad::main("Experimental")]
async fn main() 
{
    // loop {
    //     clear_background(RED);

    //     draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    //     draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    //     draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
    //     draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

    //     next_frame().await
    // }
    debug!("main test");
    
    let player = Calculator::new();
    scene::add_node(player);

    loop 
    {
        clear_background(RED);
        draw_text("Baking Calculator", 100.0, 100.0, 100.0, macroquad::color::WHITE);
        next_frame().await;

    }
}
impl Node for Calculator
{
    fn update(_node: RefMut<Self>)
    where Self: Sized 
    {
        //debug!("test");
        debug!("{:?}", input::get_char_pressed());
    }
}

fn calculate_area_pan_rect(pan_width: f32, pan_length: f32)->f32
{
    let area:f32 = pan_width * pan_length;
    return area;

}

fn calculate_area_pan_square(pan_size: f32)->f32
{
    return calculate_area_pan_rect(pan_size, pan_size);
}

fn calculate_area_pan_circle(pan_diameter: f32)->f32
{
    let area:f32 = std::f32::consts::PI * (pan_diameter/2.0).powi(2);
    return area;
}

fn calculate_area_pan_oval(pan_major_radius: f32, pan_minor_radius: f32)->f32
{
    let area:f32 = std::f32::consts::PI * ((pan_major_radius/2.0)*(pan_minor_radius/2.0)).powi(2);
    return area;
}

enum PanType
{
    CIRCLE,
    OVAL,
    RECT,
    SQUARE,
}
