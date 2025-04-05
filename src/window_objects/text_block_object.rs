use macroquad::prelude::*;

use crate::window_objects::window_object_center::WindowObjectMethods;

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

    pub fn get_size(&self) -> f32 {
        self.font_size.clone()
    }

    pub fn get_pos(&self) -> (f32, f32) {
        (self.x.clone(), self.y.clone())
    }
    
    //This is a method implemented for the TextBox structure to display a default text when it has
    //no value
    pub fn empty_update(&mut self, default_string: &str) {
        draw_text(default_string, self.x, self.y, self.font_size, self.colour);
    }
}


impl WindowObjectMethods for TextBlock {
    fn init(&self) {
      
    }

    fn update(&mut self) {

        draw_text(&self.text, self.x, self.y, self.font_size, self.colour);
    }
}


