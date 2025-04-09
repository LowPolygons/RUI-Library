use macroquad::prelude::*;
use crate::window_objects::window_object_center::WindowObjectMethods;
use crate::interactable_implementations::button_implementations::ButtonHandler;

use std::collections::BTreeMap;
use crate::window_objects::text_block_object::*;
use crate::window_objects::window_object_center::NonInteractable;
use crate::window_objects::window_object_center::HiddenManager;


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

    pub fn on_interact(&self, button_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>) -> Option<BTreeMap<u32, NonInteractable>> {
        self.button_handler.on_click(button_id, win_man_parts, win_man_hiddens)
    }
}


impl WindowObjectMethods for Button {
    fn init(&mut self) {
        self.text.init();
    }

    fn update(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.active_colour);
        self.text.update();
    }
}
