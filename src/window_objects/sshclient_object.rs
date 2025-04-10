use ssh2::Session;
use std::io::prelude::*;

use std::net::TcpStream;
use std::collections::BTreeMap;
use std::path::Path; 

use crate::window_objects::window_object_center::NonInteractable;
use crate::window_objects::window_object_center::OnlyInteractable;
use crate::window_objects::window_object_center::HiddenObjectMethods;

const NO_REPEATERS: [&str; 4] = ["ls", "head ", "tail ", "pwd"];
// Whichever button calls the make_ssh_handshake method should handle these errors for eg give
// useful error messages to a logger
pub enum HandshakeErrorCode {
    TcpFail,
    SessionFail,
    HandshakeFail,
    LoginAuthFail,
    SessionAuthFail,
}

//#[define(Copy)]
pub struct SSHClient {
    remote_server: String,
    username: String,
    password: String,

    // These are used by ssh2
    tcp_stream: Option<TcpStream>,
    session: Option<Session>,

    // This is a variable which is checked before this structure runs code - if the tcp stream
    // fails, to prevent the whole program closing this will block it
    session_still_valid: bool,

    have_logged_in: bool,
    
    login_field_values: (u32, u32, u32),

    previous_commands: Vec<String>,

    logger_id: u32,
}

impl SSHClient {
    pub fn new() -> Self {
        SSHClient {
            remote_server: String::new(),
            username: String::new(),
            password: String::new(),

            tcp_stream: None,
            session: None,

            have_logged_in: false,
            session_still_valid: true,

            login_field_values: (0, 0, 0),

            previous_commands: Vec::<String>::new(),
            //TODO: THIS 
            logger_id: 0,
        }
    }

    pub fn get_login_status(&self) -> bool {
        self.have_logged_in
    }

    pub fn update_login_field_values(&mut self, one: u32, two: u32, three: u32) {
        self.login_field_values = (one, two, three);
    }
    
    pub fn make_ssh_handshake(&mut self, rs: String, un: String, pw: String) -> Result<i8, HandshakeErrorCode> {
        self.remote_server = rs;
        self.username = un;
        self.password = pw;
        
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
        session_attempt.set_tcp_stream(tcp_stream_attempt.try_clone().map_err(|_| HandshakeErrorCode::TcpFail)?);

        // Attempt to handshake
        match session_attempt.handshake() {
            Ok(()) => {/* Can Continue */}
            Err(_) => { 
                return Err(HandshakeErrorCode::HandshakeFail); 
            }
        }

        //Attempt to authenticate a login with password
        match session_attempt.userauth_password(&self.username, &self.password) {
            Ok(()) => {
                if !session_attempt.authenticated() {
                    return Err(HandshakeErrorCode::SessionAuthFail);
                }
            }
            Err(_) => {
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

        println!("Downloading {}", target_file_name);

        let local_file_name: String = format!("DOWNLOADED_{}", filename);

        //TODO: Put a file size in the console and a warning for patience for larger files

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

    pub fn execute_command(&mut self, new_command: &str, add_to_command_list: bool) -> Result<Vec<String>, String> {
        let mut current_channel = self.session
            .clone()
            .unwrap()
            .channel_session();
        
        //Append all previous commands
        let mut full_command: String = "source ~/.bashrc".to_string(); 
        
        for com in &self.previous_commands {
            let mut dont_add: bool = false;
            for disallowed in NO_REPEATERS {
                if com.contains(disallowed) {
                    dont_add = true;
                    break;
                }
            }
            if !dont_add {
                full_command = format!("{}; {}", full_command, com);
            }
        }

        full_command = format!("{}; {}", full_command, new_command);
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
                        return Err("[SSH ERROR] There was an error executing a command.".to_string());
                    }
                }

                let mut result = String::new();
                        
                //This has a Result<usize, Err> where usize is the number of bytes

                //? is propogating the error upwards to higher dimensions (wherever called the function) to handle it
                channel.read_to_string(&mut result)
                    .map_err(|_| "[SSH ERROR] The channel was unable to read the result of your command.".to_string())?;

                channel.wait_close()
                    .map_err(|_| "[SSH ERROR] The channel was unable to gracefully close.".to_string())?;

                // Now that all error-prone areas are covered, add the result to the return vector
                resulting_lines.push(result);
                
                // Now check if it needs adding to the command list
                if add_to_command_list {
                    self.previous_commands.push(new_command.to_string());
                }

                Ok(resulting_lines)
            }
            Err(_) => {
                Err("[SSH ERROR] There was an error establishing a session-based channel.".to_string())
            }
        }
    }
}

impl HiddenObjectMethods for SSHClient {
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
             && self.login_field_values.2 != 0 {

            // First, get the values from the text boxes, using the login_field_values as IDs
            let mut hn: String = String::new();
            let mut un: String = String::new();
            let mut pw: String = String::new();
          
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
            
            //Confirm they all have a value
            if hn == "" || un == "" || pw == "" {
                //TODO: Send logger message
                println!("There is a missing piece of info before attempting to log in.");
            } else {
                //Now attempt handshake
                let ssh_result: Result<i8, HandshakeErrorCode> = self.make_ssh_handshake(hn, un, pw);
                
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
                                    log_obj.add_line("[SSH ERROR] Failed to create a link between a TCP Connection and a Session");
                                }
                                HandshakeErrorCode::LoginAuthFail => {
                                    log_obj.add_line("[SSH ERROR] Failed to authenticate a login");
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
            self.login_field_values = (0,0,0);
        }
    }
}
