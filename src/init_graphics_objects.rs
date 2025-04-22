use macroquad::prelude::*;
use std::collections::BTreeMap;

// Non Interactables
use crate::window_objects::window_object_center::NonInteractable;
use crate::window_objects::screen_decoration_object::ScreenDecoration;
use crate::window_objects::text_block_object::TextBlock;
use crate::window_objects::logger_object::Logger;

use crate::window_objects::window_object_center::HiddenManager;
use crate::window_objects::sshclient_object::SSHClient;

// Only Interactables 
use crate::window_objects::window_object_center::OnlyInteractable;
use crate::window_objects::button_object::Button;
use crate::window_objects::textbox_object::TextBox;

// Any Button Implementations Go Here
use crate::interactable_implementations::button_implementations::SSHTest;

// Any Textbox Implementations Go Here
use crate::interactable_implementations::textbox_implementation::DoNothing;
use crate::interactable_implementations::textbox_implementation::ExecuteCommand;   
use crate::interactable_implementations::textbox_implementation::DownloadFile;
use crate::interactable_implementations::textbox_implementation::UploadFile;
use crate::interactable_implementations::textbox_implementation::UploadDirectory;

/*--===--===--===--===--===--===--===--===--===--*\
|           Defining graphics Components          v
|
|  - All IDs must be unique. Ideally, keep them unique across even across both non/only interactables structures
|  - For Buttons that toggle on or off raytracers, the button id must be exactly 1 id less
|                                                 
|                                                 ^
\*--===--===--===--===--===--===--===--===--===--*/

pub fn init_graphics_objects_main(non_interactable_components: &mut BTreeMap<u32, NonInteractable>, only_interactable_components: &mut BTreeMap<u32, OnlyInteractable>, hidden_components: &mut BTreeMap<u32, HiddenManager>) {
    only_interactable_components.insert(1, OnlyInteractable::TextBox(
        TextBox::new(25.0, 25.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Hostname".to_string(),
            Box::new(DoNothing),
            TextBlock::new(35.0, 55.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            false,
            true
        )
    ));
    non_interactable_components.insert(11,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 20.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(2, OnlyInteractable::TextBox(
        TextBox::new(25.0, 90.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Username".to_string(),
            Box::new(DoNothing),
            TextBlock::new(35.0, 120.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            false,
            true
        )
    ));
    non_interactable_components.insert(12,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 85.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(3, OnlyInteractable::TextBox(
        TextBox::new(25.0, 155.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Password".to_string(),
            Box::new(DoNothing),
            TextBlock::new(35.0, 185.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            true,
            false,
            true
        )
    ));
    non_interactable_components.insert(13,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 150.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(4, OnlyInteractable::Button(
        Button::new(25.0, 220.0, 300.0, 50.0,
            Color::new(0.5, 0.2, 0.2, 1.0),
            Color::new(0.8, 0.5, 0.5, 1.0),
            Color::new(0.3, 0.01, 0.01, 1.0),
            Box::new(SSHTest),
            TextBlock::new(70.0, 250.0, Color::new(1.0, 1.0, 1.0, 1.0), "Login".to_string(), 20.0)
        )
    ));
    non_interactable_components.insert(14,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 215.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(5, OnlyInteractable::TextBox(
        TextBox::new(25.0, 315.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Command".to_string(),
            Box::new(ExecuteCommand),
            TextBlock::new(35.0, 345.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            true,
            false
        )
    ));
    non_interactable_components.insert(15,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 310.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(6, OnlyInteractable::TextBox(
        TextBox::new(25.0, 380.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter a file to download".to_string(),
            Box::new(DownloadFile),
            TextBlock::new(35.0, 410.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false, //Password mode
            true,  //Enter clears text 
            true   //Enter removes focus
        )
    ));
    non_interactable_components.insert(16,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 375.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));


    only_interactable_components.insert(7, OnlyInteractable::TextBox(
        TextBox::new(25.0, 600.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter a file to upload".to_string(),
            Box::new(UploadFile),
            TextBlock::new(35.0, 630.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false, //Password mode
            true,  //Enter clears text 
            true   //Enter removes focus
        )
    ));
    non_interactable_components.insert(17,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 595.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(8, OnlyInteractable::TextBox(
        TextBox::new(25.0, 665.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter a directory to upload".to_string(),
            Box::new(UploadDirectory),
            TextBlock::new(35.0, 695.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false, //Password mode
            true,  //Enter clears text 
            true   //Enter removes focus
        )
    ));
    non_interactable_components.insert(18,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 660.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));


    non_interactable_components.insert(0,  NonInteractable::ScreenDecoration(ScreenDecoration::new(9.0, 9.0, 332.0, 882.0, Color::new(0.2, 0.2, 0.2, 1.0))));
    non_interactable_components.insert(33, NonInteractable::ScreenDecoration(ScreenDecoration::new(349.0, 49.0, 1082.0, 842.0, Color::new(0.2, 0.2, 0.2, 1.0))));
    non_interactable_components.insert(50, NonInteractable::Logger(Logger::new(350.0, 50.0, 1080.0, 840.0, 5.0, 20.0, Color::new(1.0, 1.0, 1.0, 1.0), "".to_string())));

    non_interactable_components.insert(51, NonInteractable::TextBlock(TextBlock::new(829.0, 33.0, Color::new(0.05, 0.05, 0.05, 1.0), "Logger".to_string(), 46.0)));

    non_interactable_components.insert(48, NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 460.0, 310.0, 118.0, Color::new(0.05, 0.05, 0.05, 1.0))));
    non_interactable_components.insert(49, NonInteractable::ScreenDecoration(ScreenDecoration::new(25.0, 465.0, 300.0, 108.0, Color::new(0.5, 0.2, 0.2, 1.0))));
    
    non_interactable_components.insert(52, NonInteractable::TextBlock(TextBlock::new(30.0, 495.0, Color::new(0.05, 0.05, 0.05, 1.0), "Warning".to_string(), 40.0)));
    non_interactable_components.insert(53, NonInteractable::TextBlock(TextBlock::new(30.0, 520.0, Color::new(0.05, 0.05, 0.05, 1.0), "Downloading files is a thread-".to_string(), 20.0)));
    non_interactable_components.insert(54, NonInteractable::TextBlock(TextBlock::new(30.0, 540.0, Color::new(0.05, 0.05, 0.05, 1.0), "blocking action".to_string(), 20.0)));
    non_interactable_components.insert(55, NonInteractable::TextBlock(TextBlock::new(30.0, 560.0, Color::new(0.05, 0.05, 0.05, 1.0), "The screen may go unresponsive ".to_string(), 20.0)));

    hidden_components.insert(100, HiddenManager::SSHClient(SSHClient::new()));
}
