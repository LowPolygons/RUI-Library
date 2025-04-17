use macroquad::prelude::*;
use std::collections::BTreeMap;

use crate::window_objects::window_object_center::HiddenManager;
use crate::window_objects::window_object_center::NonInteractable;

use crate::window_objects::get_files_in_directory;
use crate::window_objects::is_directory;

const MAX_LOGGER_LINE_LENGTH: usize = 99999;
// This is a trait that is used by the Textbox structure. Button methods should be on a per-button
// basis, and as a result they need a way to have one implemented method for on press for a general
// button, but also a method individually.
// This trait allows the button to store a 'button handler' which can be anything that implements
// the buttonhandler trait. When a new button is needed, add a new structure

pub trait TextboxMethod {
    // Buttons should be able to modify parts of the window that arent directly interactable by the user, hence the copy of the map for NonInteractables
    fn on_enter(&self, textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>>;
}


pub struct DoNothing;

impl TextboxMethod for DoNothing {
    fn on_enter(&self, _t: &u32, _w: BTreeMap<u32, NonInteractable>, _i: &mut BTreeMap<u32, HiddenManager>, _e: &str) -> Option<BTreeMap<u32, NonInteractable>> {
        None
    }
}
pub struct AddLogLine;

impl TextboxMethod for AddLogLine {
    fn on_enter(&self, _textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, _win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> {
        let mut clone_of_parts = win_man_parts.clone();       

        if let Some(NonInteractable::Logger(logger)) = clone_of_parts.get_mut(&50) {
            //The input isn't directly modifyable, therefore make a clone
            logger.add_line(text);
        }
        
        Some(clone_of_parts)
    }
}

pub struct ExecuteCommand;

impl TextboxMethod for ExecuteCommand {
    fn on_enter(&self, _textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> { 
        let mut clone_of_parts = win_man_parts.clone();

        if let Some(HiddenManager::SSHClient(obj)) = win_man_hiddens.get_mut(&100) {
            
            if obj.get_login_status() && obj.is_session_still_valid() {
                let result: Result<Vec<String>, String> = obj.execute_command(text, true);

                if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&50) {

                    log_obj.add_line(&format!(">>> {}", &text));

                    match result {
                        Ok(val) => {
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
                if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&50) {
                    log_obj.add_line("[SSH WARNING] Please log in before running commands");
                }
            }
        }

        Some(clone_of_parts)
    }
}

pub struct DownloadFile;

impl TextboxMethod for DownloadFile {
    fn on_enter(&self, _textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> { 
        let mut clone_of_parts = win_man_parts.clone();

        //Confirm you have the logger and SSHCLient
        if let Some(HiddenManager::SSHClient(obj)) = win_man_hiddens.get_mut(&100) {
            if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&50) { 
                //Ensure it is logged in
                if obj.get_login_status() && obj.is_session_still_valid() {
                    //Require the directory
                    let directory: Result<Vec<String>, String> = obj.execute_command("pwd", false);
                
                    match directory {
                        Ok(contains_directory) => {
                            //Incase the list of previous commands print anything, have to get the last
                            //item from the list of outputs
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

pub struct UploadDirectory;

impl TextboxMethod for UploadDirectory {
    fn on_enter(&self, _textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> { 
        let mut clone_of_parts = win_man_parts.clone();
        
        if is_directory(text) {
        //Confirm you have the logger and SSHCLient
            if let Some(HiddenManager::SSHClient(obj)) = win_man_hiddens.get_mut(&100) {
                if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&50) {
                    let dir_files_and_directories = get_files_in_directory(text).map_err(|err| err).ok()?;
                    let mut directory_success: bool = true;

                    for curr_dir in dir_files_and_directories.1  {
                        println!("Making directory {}", curr_dir);

                        if obj.get_login_status() && obj.is_session_still_valid() {
                            //Require the directory
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
                        println!("Attempting to upload {}", curr_file);
                        //Ensure it is logged in
                        if obj.get_login_status() && obj.is_session_still_valid() && directory_success {
                            //Require the directory
                            let directory: Result<Vec<String>, String> = obj.execute_command("pwd", false);

                            match directory {
                                Ok(contains_directory) => {
                                    //Incase the list of previous commands print anything, have to get the last
                                    //item from the list of outputs
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
            if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&50) {
                log_obj.add_line("[SSH WARN] This file is not a directory");
            }
        }
        Some(clone_of_parts)
    }
}

pub struct UploadFile;

impl TextboxMethod for UploadFile {
    fn on_enter(&self, _textbox_id: &u32, win_man_parts: BTreeMap<u32, NonInteractable>, win_man_hiddens: &mut BTreeMap<u32, HiddenManager>, text: &str) -> Option<BTreeMap<u32, NonInteractable>> { 
        let mut clone_of_parts = win_man_parts.clone();
        
        //Confirm you have the logger and SSHCLient
        if let Some(HiddenManager::SSHClient(obj)) = win_man_hiddens.get_mut(&100) {
            if let Some(NonInteractable::Logger(log_obj)) = clone_of_parts.get_mut(&50) { 
                //Ensure it is logged in
                if obj.get_login_status() && obj.is_session_still_valid() {
                    //Require the directory
                    let directory: Result<Vec<String>, String> = obj.execute_command("pwd", false);
                
                    match directory {
                        Ok(contains_directory) => {
                            //Incase the list of previous commands print anything, have to get the last
                            //item from the list of outputs
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
