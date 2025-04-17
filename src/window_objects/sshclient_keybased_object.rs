use ssh2::Session;
use std::io::prelude::*;

use std::net::TcpStream;
use std::collections::BTreeMap;
use std::path::Path; 
use std::fs;
use std::fs::File;
use crate::window_objects::window_object_center::NonInteractable;
use crate::window_objects::window_object_center::OnlyInteractable;
use crate::window_objects::window_object_center::HiddenObjectMethods;

pub enum HandshakeErrorCode {
    TcpFail,
    SessionFail,
    HandshakeFail,
    LoginAuthFail,
    SessionAuthFail,
}

pub struct SSHClientKey {
    remote_server: String,
    username: String,
    password: String,
    public_key: String,
    private_key: String,

    // These are used by ssh2
    tcp_stream: Option<TcpStream>,
    session: Option<Session>,

    // This is a variable which is checked before this structure runs code - if the tcp stream
    // fails, to prevent the whole program closing this will block it
    session_still_valid: bool,

    have_logged_in: bool,
    
    login_field_values: (u32, u32, u32, u32, u32),

    previous_commands: Vec<String>,

    logger_id: u32,
}

impl SSHClientKey {
    pub fn new() -> Self {
        SSHClientKey {
            remote_server: String::new(),
            username: String::new(),
            password: String::new(),
            public_key: String::new(),
            private_key: String::new(),
            
            tcp_stream: None,
            session: None,

            have_logged_in: false,
            session_still_valid: true,

            login_field_values: (0, 0, 0, 0, 0),

            previous_commands: Vec::<String>::new(),
            
            logger_id: 0,
        }
    }

    pub fn get_login_status(&self) -> bool {
        self.have_logged_in
    }

    pub fn update_login_field_values(&mut self, one: u32, two: u32, three: u32, four: u32, five: u32) {
        self.login_field_values = (one, two, three, four, five);
    }
   
    pub fn is_session_still_valid(&self) -> bool {
        self.session_still_valid
    }

    pub fn make_ssh_handshake(&mut self, rs: String, un: String, pw: String, pubk: String, privk: String) -> Result<i8, HandshakeErrorCode> {
        self.remote_server = rs;
        self.username = un;
        self.password = pw;
        self.public_key = pubk;
        self.private_key = privk;

        // Create a TcpStream by passing in the ssh key without the username
        let tcp_stream_attempt = TcpStream::connect(format!("{}:22", self.remote_server))
            .map_err(|_| {
                self.session_still_valid = false;
                HandshakeErrorCode::TcpFail
            })?;
        
        // Create a session value to hold the session
        let mut session_attempt = Session::new()
            .map_err(|_| {
                self.session_still_valid = false;
                HandshakeErrorCode::SessionFail
            })?;

        // Link the two
        session_attempt.set_tcp_stream(tcp_stream_attempt.try_clone().map_err(|_| {
            self.session_still_valid = false;
            HandshakeErrorCode::TcpFail
            })
        ?);

        // Attempt to handshake
        match session_attempt.handshake() {
            Ok(()) => {/* Can Continue */}
            Err(_) => {
                //The user may have entered an invalid hostname, so don't necessarily destroy the session validity yet
                return Err(HandshakeErrorCode::HandshakeFail); 
            }
        }

        let public_key = if self.public_key == "" {
            None
        } else {
            Some(Path::new(&self.public_key))
        };

        let passphrase = if self.password == "" {
            None
        } else {
            Some(self.password.as_str())
        };

        match session_attempt.userauth_pubkey_file(&self.username, public_key, Path::new(&self.private_key), passphrase) {
            Ok(()) => {
                if !session_attempt.authenticated() {
                    self.session_still_valid = false;
                    return Err(HandshakeErrorCode::SessionAuthFail);
                }
            }
            Err(e) => {
                println!("{}", e);
                return Err(HandshakeErrorCode::LoginAuthFail);
            }
        }

        self.session = Some(session_attempt);
        self.tcp_stream = Some(tcp_stream_attempt);
        self.have_logged_in = true;
        Ok(1)
    }
   
    pub fn download_file(&mut self, filename: &str, directory: &str) -> Result<String, String> { //<Filename with directory if applicable, or error message
        let current_session = self.session
            .clone()
            .unwrap();

        //Attempt to create a SFTP session
        let sftp_session = current_session.sftp()
            .map_err(|_| {
                self.session_still_valid = false;
                "[SSH ERROR] Error establishing an SFTP session".to_string()
            })?;

        let target_file_name: String = format!("{}/{}", directory.trim_matches('\n'), filename);
        let local_file_name: String = format!("{}", filename);
        
        //Open the file 
        let mut target_file = sftp_session.open(Path::new(&target_file_name))
            .map_err(|_| "[SSH WARN] Problem creating file link".to_string())?;

        //Read contents into vector of strings

        let mut downloaded_content = Vec::<u8>::new();

        target_file.read_to_end(&mut downloaded_content)
            .map_err(|_| "[SSH WARN] There was a problem trying to download the file contents")?;

        //Save the contents into the desired file
        std::fs::write(&local_file_name, downloaded_content)
            .map_err(|_| "[SSH WARN] Problem creating a local save file to store the data in")?;

        //Hurray!
        Ok(local_file_name)
    }

    pub fn upload_file(&mut self, filename: &str, directory: &str) -> Result<String, String> {

        
        let current_session = self.session
            .clone()
            .unwrap();
    
        let sftp_session = current_session.sftp()
            .map_err(|_| {
                self.session_still_valid = false;
                "[SSH ERROR] Error establishing an SFTP session".to_string()
            })?;

        let target_destination = format!("{}/{}", directory.trim_matches('\n'), filename);

        let mut local_file = File::open(filename)
            .map_err(|_| "[SSH WARN] There was a problem finding the file to upload".to_string())?;
        let mut file_contents = Vec::new();
        
        local_file.read_to_end(&mut file_contents)
            .map_err(|_| "[SSH WARN] There was a problem reading the file to upload".to_string())?;
    
        //Now create the file in the remote server
        let mut target_file = sftp_session.create(Path::new(&target_destination))
            .map_err(|_| {
                self.session_still_valid = false;
                "[SSH ERROR] Could not create file link in destination folder"
            })?;

        target_file.write_all(&file_contents)
            .map_err(|_| "[SSH WARN] Could not write data to target file")?;

        Ok(target_destination)
    }

    pub fn execute_command(&mut self, new_command: &str, add_to_command_list: bool) -> Result<Vec<String>, String> {
        let mut current_channel = self.session
            .clone()
            .unwrap()
            .channel_session();
        
        //Append all previous commands
        let mut full_command: String = "source ~/.bashrc".to_string(); 
        
        for com in &self.previous_commands {
            if com.contains("cd ") {
                full_command = format!("{}; {}", full_command, com);
            }
        }
       
        if new_command.contains("cd /") {
            full_command = new_command.to_string();
            self.previous_commands = Vec::new();
        } else {
            full_command = format!("{}; {}", full_command, new_command);
        }
        // If the new command contains 'cd /' it means no other previous cd command matters, and
        // therefore can be cleared
        let command: &str = &full_command;

        println!("Executed command: {}", command);

        let mut resulting_lines: Vec<String> = Vec::<String>::new();

        //Return type Result<Session, Error>
        match current_channel {
            Ok(ref mut channel) => {
                //exec has return type Result<(), Error>
                match channel.exec(command) {
                    Ok(()) => { /* Good! We can continue */}
                    Err(_) => {
                        self.session_still_valid = false;
                        return Err("[SSH ERROR] There was an error executing a command.".to_string());
                    }
                }

                let mut result = String::new();
                        
                //This has a Result<usize, Err> where usize is the number of bytes

                //? is propogating the error upwards to higher dimensions (wherever called the function) to handle it
                channel.read_to_string(&mut result)
                    .map_err(|_| {
                        self.session_still_valid = false;
                        "[SSH ERROR] The channel was unable to read the result of your command.".to_string()
                    })?;

                channel.wait_close()
                    .map_err(|_| {
                        self.session_still_valid = false;
                        "[SSH ERROR] The channel was unable to gracefully close.".to_string()
                    })?;

                // Now that all error-prone areas are covered, add the result to the return vector
                resulting_lines.push(result);
                
                // Now check if it needs adding to the command list
                if add_to_command_list {
                    self.previous_commands.push(new_command.to_string());
                }

                Ok(resulting_lines)
            }
            Err(_) => {
                self.session_still_valid = false;
                Err("[SSH ERROR] There was an error establishing a session-based channel.".to_string())
            }
        }
    }
}

impl HiddenObjectMethods for SSHClientKey {
    fn init(&mut self) {
        self.logger_id = 50; 
    }

    fn update(&mut self, only: &mut BTreeMap<u32, OnlyInteractable>, none: &mut BTreeMap<u32, NonInteractable>) {
        if self.session_still_valid {
            //Only if it has been established that everything is okay should things go
        }

        // If these values have been changed, it means a 'login' button has been filled in
        if  self.login_field_values.0 != 0
             && self.login_field_values.1 != 0
             && self.login_field_values.2 != 0
             && self.login_field_values.3 != 0
             && self.login_field_values.4 != 0 {

            // First, get the values from the text boxes, using the login_field_values as IDs
            let mut hn: String = String::new();
            let mut un: String = String::new();
            let mut pw: String = String::new();
            let mut pb: String = String::new();
            let mut pv: String = String::new();
          
            if let Some(OnlyInteractable::TextBox(obj)) = only.get_mut(&self.login_field_values.0) {
                hn = obj.get_text()
                    .clone()
                    .to_string();

                obj.force_clear_text();
            }

            //Username
            if let Some(OnlyInteractable::TextBox(obj)) = only.get_mut(&self.login_field_values.1) {
                un = obj.get_text()
                    .clone()
                    .to_string();

                obj.force_clear_text();
            }

            //Password
            if let Some(OnlyInteractable::TextBox(obj)) = only.get_mut(&self.login_field_values.2) {
                pw = obj.get_text()
                    .clone()
                    .to_string();

                obj.force_clear_text();
            }

            //Public Key
            if let Some(OnlyInteractable::TextBox(obj)) = only.get_mut(&self.login_field_values.3) {
                pb = obj.get_text()
                    .clone()
                    .to_string();

                obj.force_clear_text();
            }
            
            //Private Key
            if let Some(OnlyInteractable::TextBox(obj)) = only.get_mut(&self.login_field_values.4) {
                pv = obj.get_text()
                    .clone()
                    .to_string();

                obj.force_clear_text();
            }

            //Confirm they all have a value
            if hn == "" || un == "" || pv == "" {
                if let Some(NonInteractable::Logger(log_obj)) = none.get_mut(&self.logger_id) {
                    log_obj.add_line("There is a missing piece of info before attempting to log in.");
                }
            } else {
                //Now attempt handshake
                let ssh_result: Result<i8, HandshakeErrorCode> = self.make_ssh_handshake(hn, un, pw, pb, pv);
                
                if let Some(NonInteractable::Logger(log_obj)) = none.get_mut(&self.logger_id) {
                    match ssh_result {
                        Ok(_) => {
                           log_obj.add_line(&format!("[SSH INFO] Successful SSH into {}", self.remote_server)); 
                        }
                        Err(err_code) => {
                            match err_code {
                                HandshakeErrorCode::TcpFail => {
                                    log_obj.add_line("[SSH ERROR] Failed to establish a TCP Connection");
                                }
                                HandshakeErrorCode::SessionFail => {
                                    log_obj.add_line("[SSH ERROR] Failed to establish a new session");
                                }
                                HandshakeErrorCode::HandshakeFail => {
                                    log_obj.add_line("[SSH WARN] Failed to create a link between a TCP Connection and a Session");
                                    log_obj.add_line("[SSH HELP] ...Did you perhaps misspell the hostname?");
                                }
                                HandshakeErrorCode::LoginAuthFail => {
                                    log_obj.add_line("[SSH WARN] Failed to authenticate a login");
                                    log_obj.add_line("[SSH HELP] ...Did you type your username and password correctly?");
                                }
                                HandshakeErrorCode::SessionAuthFail => {
                                    log_obj.add_line("[SSH ERROR] Failed to authenticate a Session");
                                }
                            } 
                        }
                    }
                }
            }
            
            //Set back to zero so it doesn't endlessly occur
            self.login_field_values = (0,0,0,0,0);
        }
    }
}
