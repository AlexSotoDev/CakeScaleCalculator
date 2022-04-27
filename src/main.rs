use std::io;

fn main() 
{
    // println!("What is your name?");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // println!("Hello, {}!", input.trim());
    println!("Area: {}", calculate_area_pan_circle(8.0));

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
