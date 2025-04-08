use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::BTreeMap;

use crate::window_objects::window_object_center::NonInteractable;
use crate::window_objects::window_object_center::OnlyInteractable;
use crate::window_objects::window_object_center::HiddenObjectMethods;

// Whichever button calls the make_ssh_handshake method should handle these errors for eg give
// useful error messages to a logger
pub enum HandshakeErrorCode {
    TcpFail,
    SessionFail,
    HandshakeFail,
    LoginAuthFail,
    SessionAuthFail,
}

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

    // TODO: have the login button call a new method to be defined in here. This will send over the
    // IDs for the objects (hardcoded) that need to have the text cleared. this will in the update
    // function check the values arent all equal to zero, then clear the box and reset to zero 
    login_field_values: (u32, u32, u32),
}

impl SSHClient {
    pub fn new(rs: String, un: String, pw: String) -> Self {
        SSHClient {
            remote_server: String::new(),
            username: String::new(),
            password: String::new(),

            tcp_stream: None,
            session: None,

            session_still_valid: true,

            login_field_values: (0, 0, 0),
        }
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
        Ok(1)
    }
    
    pub fn execute_command(&mut self, command: &str) -> Result<Vec<String>, String> {
        let mut current_channel = self.session
            .clone()
            .unwrap()
            .channel_session();

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
                match channel.read_to_string(&mut result) {
                    Ok(num_bytes) => {
                        let mut info_line: String = "[SSH INFO] The result in bytes is ".to_string();
                        info_line = info_line + &(num_bytes.to_string());

                        resulting_lines.push(info_line.to_string());
                    }
                    Err(_) => {
                        //As the Ok section returns a () in code, this has to type match therefore
                        //just Err() is returning a Result. Doing this prevents that
                        return Err("[SSH ERROR] The channel was unable to read the result of your command.".to_string());
                    }
                }

                match channel.wait_close() {
                    Ok(()) => {
                        let info_line: &str = "[SSH INFO (temp)] Graceful channel closure";
                        resulting_lines.push(info_line.to_string());
                    }
                    Err(_) => {
                        return Err("[SSH ERROR] The channel was unable to gracefully close.".to_string());
                    }
                }
                // Now that all error-prone areas are covered, add the result to the return vector
                resulting_lines.push(result);

                Ok(resulting_lines)
            }
            Err(_) => {
                Err("[SSH ERROR] There was an error establishing a session-based channel.".to_string())
            }
        }
    }
}

impl HiddenObjectMethods for SSHClient {
    fn init(&mut self) {}

    fn update(&mut self, only: &mut BTreeMap<u32, OnlyInteractable>, none: &mut BTreeMap<u32, NonInteractable>) {
        if self.session_still_valid {
            //Only if it has been established that everything is okay should things go
        }

        if  self.login_field_values.0 != 0
             && self.login_field_values.1 != 0
             && self.login_field_values.2 != 0 {
            //TODO: THIS
        }
    }
}
