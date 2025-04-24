use macroquad::prelude::*;

use crate::window_objects::window_object_center::WindowObjectMethods;

#[derive(Clone)]
pub struct TextBlock {
    x: f32,
    y: f32,
    colour: Color, 
    text: String,
    font_size: f32,
    password_mode: bool,
}

impl TextBlock {
    pub fn new(x_: f32, y_: f32, colour_: Color, text_: String, font_size_: f32) -> Self {
        TextBlock {
            x: x_,
            y: y_,
            colour: colour_,
            text: text_,
            font_size: font_size_,
            password_mode: false,
        }
    }

    pub fn set_text(&mut self, new_text: String) {
        self.text = new_text;
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn get_pos(&self) -> (f32, f32) {
        (self.x.clone(), self.y.clone())
    }
   
    pub fn set_password_mode(&mut self, inp: bool) {
        self.password_mode = inp;
    }
    // This is a method implemented for the TextBox structure to display a default text when it has no value
    pub fn empty_update(&mut self, default_string: &str, default_override: bool) {
        if !self.password_mode || default_override {
            draw_text(default_string, self.x, self.y, self.font_size, self.colour);
        } else {
            draw_text(&"*".repeat(default_string.len()), self.x, self.y, self.font_size, self.colour);
        }
    }
}


impl WindowObjectMethods for TextBlock {
    fn init(&mut self) {}

    fn update(&mut self) {
        if !self.password_mode {
            draw_text(&self.text, self.x, self.y, self.font_size, self.colour);
        } else {
            draw_text(&"*".repeat(self.text.len()), self.x, self.y, self.font_size, self.colour);
        }
    }
}


