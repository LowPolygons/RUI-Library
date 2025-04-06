use crate::window_objects::logger_object::*;
use crate::window_objects::text_block_object::*;
use crate::window_objects::raytracer_window_object::*;
use crate::window_objects::screen_decoration_object::*;
use crate::window_objects::button_object::*;
use crate::window_objects::textbox_object::*;
use crate::window_objects::sshclient_object::*;

pub trait WindowObjectMethods {
    fn init(&mut self);
    fn update(&mut self);
}

#[derive(Clone)]
pub enum NonInteractable {
    RaytracerWindow(RaytracerWindow),
    ScreenDecoration(ScreenDecoration),
    TextBlock(TextBlock),
    Logger(Logger),
}

impl WindowObjectMethods for NonInteractable {
    fn init(&mut self) {
        match self {
            NonInteractable::RaytracerWindow(object) => object.init(),
            NonInteractable::ScreenDecoration(object) => object.init(),
            NonInteractable::TextBlock(object) => object.init(),
            NonInteractable::Logger(object) => object.init(),
        }
    }

    fn update(&mut self) {
        match self {
            NonInteractable::RaytracerWindow(object) => object.update(),
            NonInteractable::ScreenDecoration(object) => object.update(),
            NonInteractable::TextBlock(object) => object.update(),
            NonInteractable::Logger(object) => object.update(),
        }
    }
}

pub enum OnlyInteractable {
    Button(Button),
    TextBox(TextBox),
}

impl WindowObjectMethods for OnlyInteractable {
    fn init(&mut self) {
        match self {
            OnlyInteractable::Button(object) => object.init(),
            OnlyInteractable::TextBox(object) => object.init(),
        }
    }

    fn update(&mut self) {
        match self {
            OnlyInteractable::Button(object) => object.update(),
            OnlyInteractable::TextBox(object) => object.update(),
        }
    }
}


pub enum HiddenManager {
    SSHClient(SSHClient),
}


impl WindowObjectMethods for HiddenManager {
    fn init(&mut self) {
        match self {
            HiddenManager::SSHClient(object) => object.init(),
        }
    }

    fn update(&mut self) {
        match self {
            HiddenManager::SSHClient(object) => object.update(),
        }
    }
}




