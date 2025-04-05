use macroquad::prelude::*;
use std::collections::BTreeMap;

use crate::window_object::NonInteractable;

// This is a trait that is used by the Textbox structure. Button methods should be on a per-button
// basis, and as a result they need a way to have one implemented method for on press for a general
// button, but also a method individually.
// This trait allows the button to store a 'button handler' which can be anything that implements
// the buttonhandler trait. When a new button is needed, add a new structure

pub trait TextboxMethod {
    // Buttons should be able to modify parts of the window that arent directly interactable by the user, hence the copy of the map for NonInteractables
    fn on_enter(&self, textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, text: &str) -> Option<BTreeMap<u32, NonInteractable>>;
}

pub struct Test;

impl TextboxMethod for Test {
    fn on_enter(&self, textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> {
        println!("TEXT BOX ENTER PRESSED");
        None
    }
}
