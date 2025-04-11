use macroquad::prelude::*;
use std::collections::BTreeMap;

use crate::window_objects::window_object_center::HiddenManager;
use crate::window_objects::window_object_center::NonInteractable;

// This is a trait that is used by the Buttons structure. Button methods should be on a per-button
// basis, and as a result they need a way to have one implemented method for on press for a general
// button, but also a method individually.
// This trait allows the button to store a 'button handler' which can be anything that implements
// the buttonhandler trait. When a new button is needed, add a new structure

pub trait ButtonHandler {
    // Buttons should be able to modify parts of the window that arent directly interactable by the user, hence the copy of the map for NonInteractables
    fn on_click(&self, button_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>) -> Option<BTreeMap<u32, NonInteractable>>;
}

pub struct ToggleRaytracer;

impl ButtonHandler for ToggleRaytracer {
    fn on_click(&self, button_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, _win_man_hiddens: &mut BTreeMap<u32, HiddenManager>) -> Option<BTreeMap<u32, NonInteractable>> {
        // The raytracer id will always be the button_id + 1         
      
        //Raytracer window block retrieved from the map
        let mut raytracer_window_object: NonInteractable = win_man_parts[&(button_id+1)].clone();
        //The input isn't directly modifyable, therefore make a clone
        let mut clone_of_parts = win_man_parts.clone();

        // Using ref (mainly used in match statements) means raytracer_window_object isn't consumed by the different statements
        match raytracer_window_object {
            // Internal modify of the render status
            NonInteractable::RaytracerWindow(ref mut obj) => { obj.change_render_status() }

            // Need th(is)(ese) or the code won't run
            NonInteractable::ScreenDecoration(ref _obj) => {}
            NonInteractable::TextBlock(ref _obj) => {}
            NonInteractable::Logger(ref _obj) => {}
        }
        
        // Insert both adds and modifies
        clone_of_parts.insert(button_id+1, raytracer_window_object);

        Some(clone_of_parts)
    }
}

pub struct SSHTest;

impl ButtonHandler for SSHTest {
    fn on_click(&self, _button_id: &u32, _win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>) -> Option<BTreeMap<u32, NonInteractable>> {
               
        if let Some(HiddenManager::SSHClient(obj)) = win_man_hiddens.get_mut(&100) {
            obj.update_login_field_values(1, 2, 3);
        }
        None
    }
}
