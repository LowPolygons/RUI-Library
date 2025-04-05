use macroquad::prelude::*;

use crate::window_object::NonInteractable;
use crate::window_object::OnlyInteractable;

use crate::main_window_manager::WindowManager;

use std::collections::BTreeMap;

// Main Structure and Methods which will be used as an (or perhaps in the future, by an) Event Polling System
pub trait UserInteractionManagerMethods {
    fn init(&mut self);
    fn update(&mut self, win_man: &mut WindowManager);
}

pub struct UserInteractionManager {
    mouse_position: (f32, f32),
}

impl UserInteractionManager {
    pub fn new() -> Self {
        UserInteractionManager {
            mouse_position: (0.0, 0.0),
        }
    }

    pub fn check_intersection(&self, xywh: (f32, f32, f32, f32)) -> bool {
        return     self.mouse_position.0 >= xywh.0
                && self.mouse_position.1 >= xywh.1
                && self.mouse_position.0 <= (xywh.0 + xywh.2)
                && self.mouse_position.1 <= (xywh.1 + xywh.3)
    }
}

// Implementation of the methods
impl UserInteractionManagerMethods for UserInteractionManager {
    fn init(&mut self) {}

    fn update(&mut self, win_man: &mut WindowManager) {
        self.mouse_position = mouse_position();
              
        let no_interactables: BTreeMap<u32, NonInteractable> = win_man.get_non_interactable_graphics_components(); 
        let only_interactables: &mut BTreeMap<u32, OnlyInteractable> = win_man.get_only_interactable_graphics_components();
        
        let mut news: BTreeMap<u32, NonInteractable> = BTreeMap::new();
        let mut has_changed: bool = false;
        let mut enter_press_failsafe: bool = false;
 

        //TODO: IT WOULD BE NICE TO MAKE THIS NEATER AND NOT BE 8-indent levels at peak
        for (id, component) in only_interactables {
            match component {
                OnlyInteractable::Button(obj) => {
                    obj.set_idle();

                    if !is_mouse_button_down(MouseButton::Left) {
                        obj.set_pressed_down(false);
                    }
                    if self.check_intersection(obj.get_intersection_values()) {
                        if is_mouse_button_down(MouseButton::Left) /*TODO: add a check so that if the mouse whilst pressed down is dragged onto the button, that it doesn't toggle the button */{
                            obj.set_depressed();
                            
                            if !obj.get_pressed_down() {
                                let result: Option<BTreeMap<u32, NonInteractable>> = obj.on_interact(&id, no_interactables.clone());
                                
                                obj.set_pressed_down(true);

                                if let Some(new_non_interactables) = result {
                                    news = new_non_interactables;
                                    has_changed = true;
                                    break;
                                }
                            }

                        } else {
                            obj.set_hover();
                        
                            obj.set_pressed_down(false);
                        }
                    }
                }
                OnlyInteractable::TextBox(obj) => {
                    obj.set_idle();

                    if self.check_intersection(obj.get_intersection_values()) {
                        if is_mouse_button_down(MouseButton::Left) { /*TODO: add a check so that if the mouse whilst pressed down is dragged onto the button, that it doesn't toggle the button */
                            obj.set_depressed();
                            
                            if !obj.get_pressed_down() {
                                obj.set_pressed_down(true);
                            }

                        } else {
                            obj.set_hover();
                        }
                    } else {
                        //Doing it this way allows you to enter text without the mouse being
                        //pressed down
                        if is_mouse_button_down(MouseButton::Left) {
                            obj.set_pressed_down(false);
                        }
                    }

                    if obj.get_pressed_down() {
                        if is_key_down(KeyCode::Enter) {
                            if !enter_press_failsafe {
                                enter_press_failsafe = true;
                                obj.set_pressed_down(false);

                                let result: Option<BTreeMap<u32, NonInteractable>> = obj.on_interact(&id, no_interactables.clone());
                             
                                if let Some(new_non_interactables) = result {
                                    news = new_non_interactables;
                                    has_changed = true;
                                    break;
                                }
                            }
                        } else {
                            enter_press_failsafe = false;
                        }
                    } else {
                        enter_press_failsafe = false;
                    }
                }
            }
        }

        if has_changed { 
            println!("The state has changed");
            win_man.set_non_interactable_graphics_components(news);
        }
    }
}
