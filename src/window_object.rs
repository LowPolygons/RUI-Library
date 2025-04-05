use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::rng;

use crate::button_implementations::ButtonHandler;
use crate::textbox_implementation::TextboxMethod;
use std::collections::BTreeMap;

/*--===--===--===--===--===--===--===--===--===--*\
|     Main Unimplemented Structure and Trait      | 
\*--===--===--===--===--===--===--===--===--===--*/

// To be paired with .contains()
const ALLOWED_CHARACTERS: &str = "1234567890-=!@#$%^&*()_+qwertyuiop[]\\QWERTYUIOP{}|asdfghjkl:'ASDFGHJKL;\"zxcvbnm,./ZXCVBNM<>? ";

//TODO: MOVE SOME OF THE IMPLEMENTATIONS TO A SEPARATE FILES CUS ITS GETTING MESSY 

#[derive(Clone)]
pub struct TextBlock {
    x: f32,
    y: f32,
    colour: Color, 
    text: String,
    font_size: f32,
}

impl TextBlock {
    pub fn new(x_: f32, y_: f32, colour_: Color, text_: String, font_size_: f32) -> Self {
        TextBlock {
            x: x_,
            y: y_,
            colour: colour_,
            text: text_,
            font_size: font_size_,
        }
    }

    pub fn set_text(&mut self, new_text: String) {
        self.text = new_text;
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    fn empty_update(&mut self, default_string: &str) {
        draw_text(default_string, self.x, self.y, self.font_size, self.colour);
    }
}

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

#[derive(Clone)]
pub struct ScreenDecoration {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    colour: Color,
}

impl ScreenDecoration {
    pub fn new(x_: f32, y_: f32, w_: f32, h_: f32, c_: Color) -> Self {
        ScreenDecoration {
            x: x_,
            y: y_,
            w: w_,
            h: h_,
            colour: c_,
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

    text: TextBlock,
}

impl Button {
    pub fn new(x_: f32, y_: f32, w_: f32, h_: f32, i_c: Color, h_c: Color, d_c: Color, handler: Box<dyn ButtonHandler>, text_: TextBlock) -> Self {
        Button {
            x: x_,
            y: y_,
            w: w_,
            h: h_,
            idle_colour: i_c,
            hover_colour: h_c,
            depressed_colour: d_c,

            active_colour: i_c,
            pressed_down: false,
            button_handler: handler,

            text: text_
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

pub struct TextBox {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    text_colour: Color,

    idle_colour: Color,
    hover_colour: Color,
    depressed_colour: Color,
    active_colour: Color,

    default_text: String,
    pressed_down: bool,

    on_enter: Box<dyn TextboxMethod>,

    text_container: TextBlock,

    //This is for preventing character duplication
    previous_char: char,
    delete_failsafe: bool,
}

impl TextBox {
    pub fn new(x_: f32, y_: f32, w_: f32, h_: f32, text_colour_: Color, idle: Color, hover: Color, depressed: Color, default: String, on_enter_: Box<dyn TextboxMethod>, text_block: TextBlock) -> Self {
        TextBox {
            x: x_,
            y: y_,
            w: w_,
            h: h_,
            text_colour: text_colour_,
            idle_colour: idle,
            hover_colour: hover,
            depressed_colour: depressed,
            active_colour: idle,
            default_text: default,
            on_enter: on_enter_,
            pressed_down: false,
            text_container: text_block,
            previous_char: '\0',
            delete_failsafe: false,
        } 
    }
    pub fn get_intersection_values(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
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

    pub fn set_pressed_down(&mut self, b: bool) {
        self.pressed_down = b;
    }

    pub fn get_pressed_down(&self) -> bool {
        self.pressed_down
    }

    pub fn on_interact(&self, textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>) -> Option<BTreeMap<u32, NonInteractable>> {
        self.on_enter.on_enter(textbox_id, win_man_parts, &self.text_container.get_text())
    }
}

#[derive(Clone)]
//These are objects that are not directly interactable with 
pub enum NonInteractable {
    // These are visually apparent onscreen
    RaytracerWindow(RaytracerWindow),
    ScreenDecoration(ScreenDecoration),
    TextBlock(TextBlock),
}

// TODO: THIS, iMPLEMENTING ssh2 SO I CAN UPLOAD AND DOWNLOAD FILES FROM SCARF NICELY
//These are objects which do not appear directly onto the screen. They can be accessed by any
//object, but not directly by the user either
pub enum HiddenManager {
    //SSHClient(SSHClient),
}

pub enum OnlyInteractable {
    Button(Button),
    TextBox(TextBox),
}

pub trait WindowObjectMethods {
    fn init(&self);
    fn update(&mut self);
}

/*--===--===--===--===--===--===--===--===--===--*\
|      Implement main Trait into Structures       | 
\*--===--===--===--===--===--===--===--===--===--*/

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


impl WindowObjectMethods for ScreenDecoration {
    fn init(&self) {

    }

    fn update(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.colour);
    }
}

impl WindowObjectMethods for TextBlock {
    fn init(&self) {
        
    }

    fn update(&mut self) {
        draw_text(&self.text, self.x, self.y, self.font_size, self.colour);
    }
}

impl WindowObjectMethods for Button {
    fn init(&self) {
        self.text.init();
    }

    fn update(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.active_colour);
        self.text.update();
    }
}

impl WindowObjectMethods for TextBox {
    fn init(&self) {
        //Need to get text working
        self.text_container.init();
    }

    fn update(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.active_colour);
        if self.pressed_down {
            //TODO: Implement backspace
            let down_key: Option<char> = get_char_pressed();
           
            if let Some(character) = down_key {
                if ALLOWED_CHARACTERS.contains(&character.to_string()) 
                    && character != self.previous_char {
                    let mut current: String = self.text_container.get_text();

                    current.push(character);

                    self.text_container.set_text(current); 

                    self.previous_char = character;
                } else {
                    self.previous_char = '\0';
                }
            } else {
                if is_key_down(KeyCode::Backspace) {
                    if !self.delete_failsafe {
                        self.delete_failsafe = true;

                        let mut current: String = self.text_container.get_text();

                        if current.len() > 0 {
                            current = current[0..current.len()-1].to_string(); //inclusive of start_index, not inclusive of end_index
                        }

                        self.text_container.set_text(current);
                    }
                } else {
                    self.delete_failsafe = false;
                }
            }
        } else {
            self.previous_char = '\0';

            clear_input_queue()
        }
        
        if self.text_container.get_text() == "" {
            self.text_container.empty_update(&self.default_text);
        } else {
            self.text_container.update();
        }
    }
}
/*--===--===--===--===--===--===--===--===--===--*\
|       Implementing main Trait into Enums        | 
|  - By doing things this way it lets you store the various graphics types in one array
|                                                 |
\*--===--===--===--===--===--===--===--===--===--*/


impl WindowObjectMethods for NonInteractable {
    fn init(&self) {
        match self {
            NonInteractable::RaytracerWindow(object) => object.init(),
            NonInteractable::ScreenDecoration(object) => object.init(),
            NonInteractable::TextBlock(object) => object.init(),
        }
    }

    fn update(&mut self) {
        match self {
            NonInteractable::RaytracerWindow(object) => object.update(),
            NonInteractable::ScreenDecoration(object) => object.update(),
            NonInteractable::TextBlock(object) => object.update(),
        }
    }
}


impl WindowObjectMethods for OnlyInteractable {
    fn init(&self) {
        match self {
            OnlyInteractable::Button(object) => object.init(),
            OnlyInteractable::TextBox(object) => object.init(),
        }
    }

    fn update(&mut self) {
        match self {
            OnlyInteractable::Button(object) => object.update(),
            OnlyInteractable::TextBox(object) => object.update(),
        }
    }
}
/*
impl WindowObjectMethods for HiddenManager {
    fn init(&self) {
        match self {
            HiddenManager::SSHClient(object) => object.init(),
        }
    }

    fn update(&mut self) {
        match self {
            HiddenManager::SSHClient(object) => object.update(),
        }
    }
}*/

