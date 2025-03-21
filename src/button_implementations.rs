use crate::user_interaction::UserInteractionManager;
use crate::main_window_manager::WindowManager;

use crate::window_object::NonInteractable;
use crate::window_object::OnlyInteractable;

use crate::window_object::RaytracerWindow;
use crate::window_object::ScreenDecoration;
use crate::window_object::Button;
use crate::window_object::WindowObjectMethods;

use std::collections::BTreeMap;


pub trait ButtonHandler {
    fn on_click(&self, button_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>) -> Option<BTreeMap<u32, NonInteractable>>;
}

pub struct ToggleRaytracer;

impl ButtonHandler for ToggleRaytracer {
    fn on_click(&self, button_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>) -> Option<BTreeMap<u32, NonInteractable>> {
        //The raytracer id will always be the button_id + 1         
       
        //Be careful as this returns a Some
        let mut example: NonInteractable = win_man_parts[&(button_id+1)].clone();
        let mut clone_of_parts = win_man_parts.clone();

        match example {
            NonInteractable::RaytracerWindow(ref mut obj) => { obj.change_render_status() }
            NonInteractable::ScreenDecoration(ref obj) => {}
        }

        clone_of_parts.insert(button_id+1, example);

        Some(clone_of_parts)
    }
}
