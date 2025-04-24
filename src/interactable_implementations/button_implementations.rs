use macroquad::prelude::*;
use std::collections::BTreeMap;

use crate::window_objects::window_object_center::HiddenManager;
use crate::window_objects::window_object_center::NonInteractable;

use crate::object_ids::*;

// Buttons run a command, and so that buttons with different methods can be stored in the same Box
// they have a struct which implements the below trait.

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
