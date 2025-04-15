use macroquad::prelude::*;

use crate::window_objects::window_object_center::WindowObjectMethods;
use crate::window_objects::WIDEST_CHARACTER_PIXEL_WIDTH;
use crate::window_objects::ALLOWED_CHARACTERS;

use crate::interactable_implementations::textbox_implementation::TextboxMethod;

use std::collections::BTreeMap;

use crate::window_objects::window_object_center::NonInteractable;
use crate::window_objects::text_block_object::TextBlock;
use crate::window_objects::window_object_center::HiddenManager;

pub struct TextBox {
    x: f32,
    y: f32,
    w: f32,
    h: f32,

    idle_colour: Color,
    hover_colour: Color,
    depressed_colour: Color,
    active_colour: Color,

    default_text: String,
    pressed_down: bool,

    on_enter: Box<dyn TextboxMethod>,

    text_container: TextBlock,
    
    password_mode: bool,
    //This is for preventing character duplication
    previous_char: char,
    delete_failsafe: bool,
    first_frame_failsafe: bool,

    enter_clears_text: bool,
    enter_removes_focus: bool,
}

impl TextBox {
    pub fn new(x_: f32, y_: f32, w_: f32, h_: f32, idle: Color, hover: Color, depressed: Color, default: String, on_enter_: Box<dyn TextboxMethod>, text_block: TextBlock, pm: bool, ect: bool, erf: bool) -> Self {
        TextBox {
            x: x_,
            y: y_,
            w: w_,
            h: h_,
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
            first_frame_failsafe: false,
            password_mode: pm,
            enter_clears_text: ect,
            enter_removes_focus: erf,
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

    pub fn on_interact(&self, textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>) -> Option<BTreeMap<u32, NonInteractable>> {
        self.on_enter.on_enter(textbox_id, win_man_parts, win_man_hiddens, &self.text_container.get_text())
    }
    
    // For when ti is absolutely necessary
    pub fn force_clear_text(&mut self) {
        self.text_container.set_text("".to_string());
    }

    // This is a standard method for when you press enter
    pub fn clear_text(&mut self) {
        if self.enter_clears_text {
            self.text_container.set_text("".to_string());
        }
    }

    pub fn get_text(&self) -> String {
        self.text_container.get_text().clone()
    }

    pub fn does_enter_remove_focus(&self) -> bool {
        self.enter_removes_focus
    }

    pub fn force_set_text(&mut self, text: String) {
        self.text_container.set_text(text);
    }
}


impl WindowObjectMethods for TextBox {
    fn init(&mut self) {
        //Need to get text working
        self.text_container.init();
        self.text_container.set_password_mode(self.password_mode);
    }

    fn update(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.active_colour);

        // If the textbox is pressed down, this is the block of code which allows for proper user input
        if self.pressed_down {
            if !self.first_frame_failsafe {
                // Only one key per frame
                let down_key: Option<char> = get_char_pressed();
                    
                if let Some(character) = down_key {
                    // If it is valid and wasn't the character on the previous frame
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
                    self.previous_char = '\0';

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
                self.first_frame_failsafe = false;
                clear_input_queue();
            }
        } else {
            self.previous_char = '\0';
            self.first_frame_failsafe = true;
        }
        
        if self.text_container.get_text() == "" {
            self.text_container.empty_update(&self.default_text, true);
        } else {
            let distance_from_edge: f32 = (self.x + self.w) - self.text_container.get_pos().0 - WIDEST_CHARACTER_PIXEL_WIDTH;

            let max_num_chars: usize = (distance_from_edge / WIDEST_CHARACTER_PIXEL_WIDTH).floor() as usize;

            let string: String = self.text_container.get_text();

            if string.len() <= max_num_chars {
                self.text_container.update()
            } else {
                self.text_container.empty_update(&string[string.len() - max_num_chars .. string.len()], false);
            }
        }
    }
}
