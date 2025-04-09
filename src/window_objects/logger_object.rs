use macroquad::prelude::*;

use crate::window_objects::window_object_center::WindowObjectMethods;
use crate::window_objects::WIDEST_CHARACTER_PIXEL_WIDTH;

use std::cmp;

#[derive(Clone)]
pub struct Logger {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    x_padding: f32,

    lines: Vec<String>,
    string_colour: Color,
    font_size: f32,
    line_tag: String,
}

impl Logger {
    pub fn new(x_: f32, y_: f32, w_: f32, h_: f32, padding: f32, size: f32, colour: Color, tag: String) -> Self {
        Logger {
            x: x_,
            y: y_,
            w: w_,
            h: h_,
            x_padding: padding,
            lines: Vec::<String>::new(),
            string_colour: colour,
            font_size: size,
            line_tag: tag,
        }
    }

    pub fn add_line(&mut self, inp: &str) {
        let mut input: String = inp.to_string();

        let distance_from_edge: f32 = (self.w) - self.x_padding - WIDEST_CHARACTER_PIXEL_WIDTH;

        let max_num_chars: usize = (distance_from_edge / WIDEST_CHARACTER_PIXEL_WIDTH).floor() as usize;

        input = self.line_tag.clone() + &input; 

        //Split at line breaks, then as this is only a logger and not a command line, just truncate
        //the output if it exceeds the screen size
        let strings_to_add: Vec<String> = input
            .lines()
            .map(|s| s.to_string())
            .map(|line| line[0..cmp::min(line.len(), max_num_chars)].to_string())
            .collect::<Vec<String>>();

        for line in strings_to_add {
            self.lines.push(line.clone());
        }
    }
}


impl WindowObjectMethods for Logger {
    fn init(&mut self) {
    }

    fn update(&mut self) {
        //The font size is how tall the characters are
        let max_lines: usize = (self.h / (self.font_size + 2.0)).floor() as usize;
        let mut lower_index: usize = 0;
        let upper_index: usize = self.lines.len();
      
        if self.lines.len() > max_lines {
            lower_index = upper_index - max_lines - 1;
        }

        let mut current: f32 = 0.0;

        for index in lower_index..upper_index {
            let how_high_up: f32 = (upper_index - index) as f32;

            draw_text(&self.lines[index], self.x + self.x_padding, (self.y + self.h) - self.font_size /* <- padding */ - how_high_up*self.font_size, self.font_size, self.string_colour);

            current = current + 1.0;
        }
    }
}



