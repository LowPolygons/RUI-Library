use macroquad::prelude::*;
use std::collections::BTreeMap;

use crate::window_objects::window_object_center::HiddenManager;
use crate::window_objects::window_object_center::NonInteractable;

// This is a trait that is used by the Textbox structure. Button methods should be on a per-button
// basis, and as a result they need a way to have one implemented method for on press for a general
// button, but also a method individually.
// This trait allows the button to store a 'button handler' which can be anything that implements
// the buttonhandler trait. When a new button is needed, add a new structure

pub trait TextboxMethod {
    // Buttons should be able to modify parts of the window that arent directly interactable by the user, hence the copy of the map for NonInteractables
    fn on_enter(&self, textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>>;
}

pub struct AddLogLine;

impl TextboxMethod for AddLogLine {
    fn on_enter(&self, textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> {
        // The raytracer id will always be the button_id + 1         
      
        //Raytracer window block retrieved from the map
        let mut logger: NonInteractable = win_man_parts[&50].clone();
        //The input isn't directly modifyable, therefore make a clone
        let mut clone_of_parts = win_man_parts.clone();

        // Using ref (mainly used in match statements) means raytracer_window_object isn't consumed by the different statements
        match logger {
            // Internal modify of the render status
            NonInteractable::RaytracerWindow(ref _obj) => {}

            // Need th(is)(ese) or the code won't run
            NonInteractable::ScreenDecoration(ref _obj) => {}
            NonInteractable::TextBlock(ref _obj) => {}
            NonInteractable::Logger(ref mut obj) => {
                obj.add_line(text);
            }
        }
        
        // Insert both adds and modifies
        clone_of_parts.insert(50, logger);

        Some(clone_of_parts)
    }
}

pub struct ExecuteCommand;

impl TextboxMethod for ExecuteCommand {
    fn on_enter(&self, textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> { 
        if let Some(HiddenManager::SSHClient(obj)) = win_man_hiddens.get_mut(&100) {

            /*First re-run all previosu valid commands
            let valid_previous_commands: Vec<String> = obj.get_previous_commands();

            for command in valid_previous_commands {
                let _ = obj.execute_command(&command, false);
            }*/

            let result: Result<Vec<String>, String> = obj.execute_command(text, true);

            match result {
                Ok(val) => {
                    for l in val {
                        println!("{}", l);
                    }
                }
                Err(e) => {
                    println!("Error {}", e);
                }
            }
        }

        None
    }
}
