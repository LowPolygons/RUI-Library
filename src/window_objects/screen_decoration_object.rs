use macroquad::prelude::*;
use crate::window_objects::window_object_center::WindowObjectMethods;


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

impl WindowObjectMethods for ScreenDecoration {
    fn init(&self) {}

    fn update(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.colour);
    }
}
