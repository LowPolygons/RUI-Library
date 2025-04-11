use macroquad::prelude::*;
use std::collections::BTreeMap;

use crate::window_objects::window_object_center::HiddenManager;
use crate::window_objects::window_object_center::NonInteractable;
use crate::window_objects::raytracer_window_object::RaytracerWindow;

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
        let mut clone_of_parts = win_man_parts.clone();

        if let Some(NonInteractable::RaytracerWindow(obj)) = clone_of_parts.get_mut(&(button_id+1)) {
            obj.change_render_status();
        }

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
