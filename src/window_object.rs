use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::rng;

use crate::user_interaction::UserInteractionManager;
use crate::main_window_manager::WindowManager;

use crate::button_implementations::ButtonHandler;
use crate::button_implementations::ToggleRaytracer;

use std::collections::HashMap;
use std::collections::BTreeMap;

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
   pub fn new(X: f32, Y: f32, W: f32, H: f32, C: Color) -> Self {
        RaytracerWindow {
            x: X,
            y: Y,
            w: W,
            h: H,
            colour: C,
           
            //TODO: AMEND AS
            render: false,
            image_object: Image::gen_image_color(W as u16, H as u16, C),
        }
    }

   pub fn change_render_status(&mut self) {
       self.render = !self.render;
   }

   pub fn get_render_status(&self) -> bool {
       self.render
   }
}

#[derive(Clone)]
pub struct ScreenDecoration {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    colour: Color,
}

impl ScreenDecoration {
    pub fn new(X: f32, Y: f32, W: f32, H: f32, C: Color) -> Self {
        ScreenDecoration {
            x: X,
            y: Y,
            w: W,
            h: H,
            colour: C,
        }
    }
}

pub struct Button {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    active_colour: Color,
    
    depressed_colour: Color,
    idle_colour: Color,
    hover_colour: Color,

    pressed_down: bool,
    button_handler: Box<dyn ButtonHandler>,
}

impl Button {
    pub fn new(X: f32, Y: f32, W: f32, H: f32, i_c: Color, h_c: Color, d_c: Color, handler: Box<dyn ButtonHandler>) -> Self {
        Button {
            x: X,
            y: Y,
            w: W,
            h: H,
            idle_colour: i_c,
            hover_colour: h_c,
            depressed_colour: d_c,

            active_colour: i_c,
            pressed_down: false,
            button_handler: handler,
        }
    }
    
    pub fn get_intersection_values(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }

    pub fn set_pressed_down(&mut self, b: bool) {
        self.pressed_down = b;
    }

    pub fn get_pressed_down(&self) -> bool {
        self.pressed_down
    }

    pub fn set_idle(&mut self) {
        self.active_colour = self.idle_colour.clone();
    }

    pub fn set_hover(&mut self) {
        self.active_colour = self.hover_colour.clone();
    }

    pub fn set_depressed(&mut self) {
        self.active_colour = self.depressed_colour.clone();
    }

    pub fn on_interact(&self, button_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>) -> Option<BTreeMap<u32, NonInteractable>> {
        self.button_handler.on_click(button_id, win_man_parts)
    }
}



#[derive(Clone)]
pub enum NonInteractable {
    RaytracerWindow(RaytracerWindow),
    ScreenDecoration(ScreenDecoration),
}

pub enum OnlyInteractable {
    Button(Button)
}

pub trait WindowObjectMethods {
    fn init(&self);
    fn update(&mut self);
}

impl WindowObjectMethods for RaytracerWindow {
    fn init(&self) {
        
    }

    fn update(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.colour);
        
        if self.render {
            //println!("Iteration");

            let mut rng = rng();
       
            //TODO: AMEND AS
            for yPixel in 1..(self.h as i32){
                for xPixel in 1..(self.w as i32) {
                    let r: f32 = (rng.random_range(1..=255) as f32 / 255.0) as f32;
                    let g: f32 = (rng.random_range(1..=255) as f32 / 255.0) as f32; 
                    let b: f32 = (rng.random_range(1..=255) as f32 / 255.0) as f32;

                    self.image_object.set_pixel(xPixel as u32, yPixel as u32, Color::new(r, g, b, 1.0));
                }
            }
        }

        let image_texture = Texture2D::from_image(&self.image_object);
        
        //The colour parameter is a tint, therefore use white
        draw_texture(&image_texture, self.x, self.y, WHITE);
    }
}

impl WindowObjectMethods for ScreenDecoration {
    fn init(&self) {

    }

    fn update(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.colour);
    }
}

impl WindowObjectMethods for Button {
    fn init(&self) {

    }

    fn update(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.active_colour);
    }
}


impl WindowObjectMethods for NonInteractable {
    fn init(&self) {
        match self {
            NonInteractable::RaytracerWindow(object) => object.init(),
            NonInteractable::ScreenDecoration(object) => object.init(),
        }
    }

    fn update(&mut self) {
        match self {
            NonInteractable::RaytracerWindow(object) => object.update(),
            NonInteractable::ScreenDecoration(object) => object.update(),
        }
    }
}


impl WindowObjectMethods for OnlyInteractable {
    fn init(&self) {
        match self {
            OnlyInteractable::Button(object) => object.init(),
        }
    }

    fn update(&mut self) {
        match self {
            OnlyInteractable::Button(object) => object.update(),
        }
    }
}


