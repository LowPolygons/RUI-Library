use macroquad::prelude::*;
use crate::window_objects::window_object_center::WindowObjectMethods;
use ::rand::Rng;
use ::rand::rng;

#[derive(Clone)]
pub struct RaytracerWindow {
    //These are mandatory
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    colour: Color,
 
    //These are Raytracer specific
    render: bool,
    image_object: Image, //Dimensions equal to the raytracer window, can use set_pixel(x, y, colour)
}

impl RaytracerWindow {
   pub fn new(x_: f32, y_: f32, w_: f32, h_: f32, c_: Color) -> Self {
        RaytracerWindow {
            x: x_,
            y: y_,
            w: w_,
            h: h_,
            colour: c_,
           
            //TODO: AMEND AS
            render: false,
            image_object: Image::gen_image_color(w_ as u16, h_ as u16, c_),
        }
    }

   pub fn change_render_status(&mut self) {
       self.render = !self.render;
   }

   pub fn get_render_status(&self) -> bool {
       self.render
   }
}

impl WindowObjectMethods for RaytracerWindow {
    fn init(&self) {
        
    }

    fn update(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.colour);
        
        if self.render {
            let mut rng = rng();
       
            //TODO: AMEND AS
            for y_pixel in 0..(self.h as i32){
                for x_pixel in 0..(self.w as i32) {
                    let r: f32 = (rng.random_range(1..=255) as f32 / 255.0) as f32;
                    let g: f32 = (rng.random_range(1..=255) as f32 / 255.0) as f32; 
                    let b: f32 = (rng.random_range(1..=255) as f32 / 255.0) as f32;

                    self.image_object.set_pixel(x_pixel as u32, y_pixel as u32, Color::new(r, g, b, 1.0));
                }
            }
        }

        let image_texture = Texture2D::from_image(&self.image_object);
        
        //The colour parameter is a tint, therefore use white
        draw_texture(&image_texture, self.x, self.y, WHITE);
    }
}


