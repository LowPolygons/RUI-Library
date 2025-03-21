use macroquad::prelude::*;

use crate::window_object::WindowObject;

use crate::window_object::RaytracerWindow;
use crate::window_object::ScreenDecoration;
use crate::window_object::Button;
use crate::window_object::WindowObjectMethods;

use crate::main_window_manager::WindowManager;

use std::collections::BTreeMap;

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
        return (   self.mouse_position.0 >= xywh.0
                && self.mouse_position.1 >= xywh.1
                && self.mouse_position.0 <= (xywh.0 + xywh.2)
                && self.mouse_position.1 <= (xywh.1 + xywh.3))
    }
}

impl UserInteractionManagerMethods for UserInteractionManager {
    fn init(&mut self) {

    }

    //Instead, send over a copy of the graphics components
    fn update(&mut self, win_man: &mut WindowManager) {
        self.mouse_position = mouse_position();
        
        let graphics_components = win_man.get_graphics_components();
        
        let mut no_interactables: BTreeMap<u32, WindowObject> = BTreeMap::new();
        let mut only_interactables: BTreeMap<u32, WindowObject> = BTreeMap::new();

        //Loop through and filter out any "button" type, or directly interactable types
        for (id, component) in graphics_components {
            match component {
                WindowObject::ScreenDecoration(obj) => {
                    no_interactables.insert(*id, *component);
                }
                WindowObject::Button(obj) => {}
                WindowObject::RaytracerWindow(obj) => {
                    no_interactables.insert(*id, *component);
                }
            };
        }

        for (id, component) in graphics_components {
            match component {
                WindowObject::Button(obj) => {
                    //Check if it intersects, if so handle double_clicking here
                    obj.set_idle();

                    if self.check_intersection(obj.get_intersection_values()) {
                        if is_mouse_button_down(MouseButton::Left) {
                            obj.set_depressed();
                            
                            if !obj.get_pressed_down() {
                                let result: Option<BTreeMap<u32, WindowObject>> = obj.on_interact(&id, no_interactables);
                                //obj.on_interact(&id, win_man.get_graphics_components());
                                println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
                                obj.set_pressed_down(true);
                            }

                        } else {
                            obj.set_hover();
                        
                            obj.set_pressed_down(false);
                        }
                    }
                }
                WindowObject::ScreenDecoration(_) => {}
                WindowObject::RaytracerWindow(_) => {}
            } 
        }

        println!("Mouse Position: {}, {}", self.mouse_position.0, self.mouse_position.1);
    }
}
