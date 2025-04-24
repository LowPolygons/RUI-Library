use macroquad::prelude::*;
use std::collections::BTreeMap;

use crate::window_objects::window_object_center::HiddenManager;
use crate::window_objects::window_object_center::NonInteractable;

use crate::interactable_implementations::get_files_in_directory;
use crate::interactable_implementations::is_directory;

use crate::object_ids::*;

// In the event of a command (like 'cat') which displays too much data, this is a good failsafe
const MAX_LOGGER_LINE_LENGTH: usize = 99999;

// Define structs which implement this trait; doing so allows them to all be stored in the same Box
pub trait TextboxMethod {
    fn on_enter(&self, textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>>;
}

pub struct DoNothing;
pub struct AddLogLine;
pub struct ExecuteCommand;
pub struct DownloadFile;
pub struct UploadDirectory;
pub struct UploadFile;

// Not all text boxes should do something upon pressing enter directly
impl TextboxMethod for DoNothing {
    fn on_enter(&self, _t: &u32, _w: BTreeMap<u32, NonInteractable>, _i: &mut BTreeMap<u32, HiddenManager>, _e: &str) -> Option<BTreeMap<u32, NonInteractable>> {
        None
    }
}


impl TextboxMethod for AddLogLine {
    fn on_enter(&self, _textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, _win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> {
        let mut clone_of_parts = win_man_parts.clone();       

        if let Some(NonInteractable::Logger(logger)) = clone_of_parts.get_mut(&LOGGER) {
            logger.add_line(text);
        }
        
        Some(clone_of_parts)
    }
}


impl TextboxMethod for ExecuteCommand {
    fn on_enter(&self, _textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> { 
        let mut clone_of_parts = win_man_parts.clone();

        // Mandatory line to confirm the obj is an SSHClient
        if let Some(HiddenManager::SSHClient(obj)) = win_man_hiddens.get_mut(&SSHCLIENT) {
           
            // Confirm the user is logged in and that the session has not an invalidating error
            if obj.get_login_status() && obj.is_session_still_valid() {

                // Run command, where the boolean is whether it should be added to the list 
                // of previous commands (Always true in this context)
                let result: Result<Vec<String>, String> = obj.execute_command(text, true);

                // Confirm the logger object still exists
                if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&LOGGER) {

                    log_obj.add_line(&format!(">>> {}", &text));

                    match result {
                        // Val is a Vector
                        Ok(val) => {
                            // Filter out excessive results
                            for l in val {
                                if l.len() > MAX_LOGGER_LINE_LENGTH {
                                    log_obj.add_line(&l[0..MAX_LOGGER_LINE_LENGTH]);
                               } else {
                                    log_obj.add_line(&l)
                                }
                            }
                        }
                        Err(e) => {
                            log_obj.add_line(&format!("Execution Error: {}", &e));
                        }
                    }                    
                    // If the command was 'clear', clear the log
                    if text == "clear" {
                        log_obj.clear_lines(); 
                    }
                }
            } else {
                if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&LOGGER) {
                    log_obj.add_line("[SSH WARNING] Please log in before running commands");
                }
            }
        }

        Some(clone_of_parts)
    }
}



impl TextboxMethod for DownloadFile {
    fn on_enter(&self, _textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> { 
        let mut clone_of_parts = win_man_parts.clone();

        // Confirm you have the logger and SSHCLient
        if let Some(HiddenManager::SSHClient(obj)) = win_man_hiddens.get_mut(&SSHCLIENT) {
            if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&LOGGER) { 

                // Ensure it is logged in and valid
                if obj.get_login_status() && obj.is_session_still_valid() {

                    // Require the directory, dont add command to previous ones
                    let directory: Result<Vec<String>, String> = obj.execute_command("pwd", false);
                
                    match directory {
                        Ok(contains_directory) => {
                            // Incase the list of previous commands print anything, have to get the last
                            // item from the list of outputs
                            let result: Result<String, String> = obj.download_file(text, &contains_directory[contains_directory.len()-1]);
                            
                            match result {
                                Ok(filepath) => {
                                    log_obj.add_line(&format!("Downloaded file to {}", filepath)); 
                                }
                                Err(e) => {
                                    log_obj.add_line(&e);
                                }
                            }
                        }
                        Err(err) => {
                            log_obj.add_line(&err);
                        }
                    }
                }
            }
        }

        Some(clone_of_parts)
    }
}



impl TextboxMethod for UploadDirectory {
    fn on_enter(&self, _textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> { 
        let mut clone_of_parts = win_man_parts.clone();
        
        // Confirm the name they have given is a directory or an error will be caused further down
        if is_directory(text) {
            
            // Once again confirm these are valid
            if let Some(HiddenManager::SSHClient(obj)) = win_man_hiddens.get_mut(&SSHCLIENT) {
                if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&LOGGER) {

                    // Get files and directories. Loop through directories and create them in the
                    // new location if they haven't been already
                    let dir_files_and_directories = get_files_in_directory(text).map_err(|err| err).ok()?;
                    let mut directory_success: bool = true;

                    for curr_dir in dir_files_and_directories.1  {

                        // Log in terminal as debug info
                        println!("Making directory {}", curr_dir);

                        if obj.get_login_status() && obj.is_session_still_valid() {

                            // Require the directory
                            let result: Result<Vec<String>, String> = obj.execute_command(&format!("mkdir {}", curr_dir), false);

                            match result {
                                Ok(_) => {/* Success */}
                                Err(_) => {
                                    directory_success = false;    
                                }
                            }
                        }
                    }

                    for curr_file in dir_files_and_directories.0 {

                        // Log in terminal as debug info
                        println!("Attempting to upload {}", curr_file);

                        if obj.get_login_status() && obj.is_session_still_valid() && directory_success {
                            
                            // Aquire the directory
                            let directory: Result<Vec<String>, String> = obj.execute_command("pwd", false);

                            match directory {
                                Ok(contains_directory) => {

                                    // Incase the list of previous commands print anything, have to get the last
                                    // item from the list of outputs
                                    let result: Result<String, String> = obj.upload_file(&curr_file, &contains_directory[contains_directory.len()-1]);
                                
                                    match result {
                                        Ok(filepath) => {
                                            log_obj.add_line(&format!("Uploaded file to {}", filepath)); 
                                        }
                                        Err(e) => {
                                            log_obj.add_line(&e);
                                       }
                                    }
                                }
                                Err(err) => {
                                    log_obj.add_line(&err);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&LOGGER) {
                log_obj.add_line("[SSH WARN] This file is not a directory");
            }
        }
        Some(clone_of_parts)
    }
}


impl TextboxMethod for UploadFile {
    fn on_enter(&self, _textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> { 
        let mut clone_of_parts = win_man_parts.clone();
        
        if let Some(HiddenManager::SSHClient(obj)) = win_man_hiddens.get_mut(&SSHCLIENT) {
            if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&LOGGER) { 
                if obj.get_login_status() && obj.is_session_still_valid() {

                    // Require the directory
                    let directory: Result<Vec<String>, String> = obj.execute_command("pwd", false);
                
                    match directory {
                        Ok(contains_directory) => {

                            // Incase the list of previous commands print anything, have to get the last
                            // item from the list of outputs
                            let result: Result<String, String> = obj.upload_file(text, &contains_directory[contains_directory.len()-1]);
                            
                            match result {
                                Ok(filepath) => {
                                    log_obj.add_line(&format!("Uploaded file to {}", filepath)); 
                                }
                                Err(e) => {
                                    log_obj.add_line(&e);
                                }
                            }
                        }
                        Err(err) => {
                            log_obj.add_line(&err);
                        }
                    }
                }
            }
        }

        Some(clone_of_parts)
    }
}
