use crate::window_object::WindowObject;
use crate::user_interaction::UserInteractionManager;
use crate::main_window_manager::WindowManager;
use crate::window_object::Button;

use std::collections::BTreeMap;

pub trait ButtonHandler {
    fn on_click(&self, button_id: &u32, win_man_parts: BTreeMap<u32, WindowObject>) -> Option<BTreeMap<u32, WindowObject>>;
}

pub struct ToggleRaytracer;

impl ButtonHandler for ToggleRaytracer {
    fn on_click(&self, button_id: &u32, win_man_parts: BTreeMap<u32, WindowObject>) -> Option<BTreeMap<u32, WindowObject>> {
        //The raytracer id will always be the button_id + 1         
       
        Some(win_man_parts)
    }
}
