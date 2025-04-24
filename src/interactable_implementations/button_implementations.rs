use macroquad::prelude::*;
use std::collections::BTreeMap;

use crate::window_objects::window_object_center::HiddenManager;
use crate::window_objects::window_object_center::NonInteractable;
use crate::window_objects::raytracer_window_object::RaytracerWindow;

use crate::object_ids::*;

// This is a trait that is used by the Buttons structure. Button methods should be on a per-button
// basis, and as a result they need a way to have one implemented method for on press for a general
// button, but also a method individually.
// This trait allows the button to store a 'button handler' which can be anything that implements
// the buttonhandler trait. When a new button is needed, add a new structure

pub trait ButtonHandler {
    fn on_click(&self, button_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>) -> Option<BTreeMap<u32, NonInteractable>>;
}

pub struct SSHConnect;

impl ButtonHandler for SSHConnect {
    fn on_click(&self, _button_id: &u32, _win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>) -> Option<BTreeMap<u32, NonInteractable>> {
               
        if let Some(HiddenManager::SSHClient(obj)) = win_man_hiddens.get_mut(&SSHCLIENT) {
            // Hostname
            // Username
            // Password
            // (or)
            // Public Key
            // Private Key
            // Passphrase
            obj.update_login_field_values(HOSTNAME_BOX, USERNAME_BOX, PASSWORD_BOX, PUBLIC_KEY_BOX, PRIVATE_KEY_BOX, PASSPHRASE_BOX);
        }
        None
    }
}
