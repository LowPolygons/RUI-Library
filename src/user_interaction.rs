use macroquad::prelude::*;

use crate::window_object::NonInteractable;
use crate::window_object::OnlyInteractable;

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
              
        let mut no_interactables: BTreeMap<u32, NonInteractable> = win_man.get_non_interactable_graphics_components(); 
        let only_interactables: &mut BTreeMap<u32, OnlyInteractable> = win_man.get_only_interactable_graphics_components();
        
        let mut news: BTreeMap<u32, NonInteractable> = BTreeMap::new();
        let mut has_changed: bool = false;

        for (id, component) in only_interactables {
            match component {
                OnlyInteractable::Button(obj) => {
                    //Check if it intersects, if so handle double_clicking here
                    obj.set_idle();

                    if !is_mouse_button_down(MouseButton::Left) {
                        obj.set_pressed_down(false);
                    }
                    if self.check_intersection(obj.get_intersection_values()) {
                        if is_mouse_button_down(MouseButton::Left) {
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
            } 
        }

        if has_changed { 
            println!("The state has changed");
            win_man.set_non_interactable_graphics_components(news);
        }
    }
}
