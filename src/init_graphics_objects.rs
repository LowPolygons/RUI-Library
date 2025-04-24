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

// Buttons: range from 1 to 20
// Corresponding Backgrounds: 101 to 120

pub fn init_graphics_objects_main(non_interactable_components: &mut BTreeMap<u32, NonInteractable>, only_interactable_components: &mut BTreeMap<u32, OnlyInteractable>, hidden_components: &mut BTreeMap<u32, HiddenManager>) {
    only_interactable_components.insert(1, OnlyInteractable::TextBox(
        TextBox::new(25.0, 25.0, 300.0, 50.0,
            Color::new(1.0, 0.55, 0.55, 1.0),
            Color::new(1.0, 0.8, 0.8, 1.0),
            Color::new(0.7, 0.3, 0.3, 1.0),
            "Enter Hostname".to_string(),
            Box::new(DoNothing),
            TextBlock::new(35.0, 55.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            false,
            true
        )
    ));
    non_interactable_components.insert(101,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 20.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(2, OnlyInteractable::TextBox(
        TextBox::new(340.0, 25.0, 300.0, 50.0,
            Color::new(1.0, 0.55, 0.55, 1.0),
            Color::new(1.0, 0.8, 0.8, 1.0),
            Color::new(0.7, 0.3, 0.3, 1.0),
            "Enter Username".to_string(),
            Box::new(DoNothing),
            TextBlock::new(350.0, 55.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            false,
            true
        )
    ));
    non_interactable_components.insert(102,  NonInteractable::ScreenDecoration(ScreenDecoration::new(335.0, 20.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));
    
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
    non_interactable_components.insert(103,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 150.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));
    
    only_interactable_components.insert(4, OnlyInteractable::TextBox(
        TextBox::new(340.0, 90.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Public Key Path".to_string(),
            Box::new(DoNothing),
            TextBlock::new(350.0, 120.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            false,
            true
        )
    ));
    non_interactable_components.insert(104,  NonInteractable::ScreenDecoration(ScreenDecoration::new(335.0, 85.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(5, OnlyInteractable::TextBox(
        TextBox::new(340.0, 155.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Private Key Path".to_string(),
            Box::new(DoNothing),
            TextBlock::new(350.0, 185.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            false,
            true
        )
    ));
    non_interactable_components.insert(105,  NonInteractable::ScreenDecoration(ScreenDecoration::new(335.0, 150.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(6, OnlyInteractable::TextBox(
        TextBox::new(340.0, 220.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Passphrase".to_string(),
            Box::new(DoNothing),
            TextBlock::new(350.0, 250.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            false,
            true
        )
    ));
    non_interactable_components.insert(106,  NonInteractable::ScreenDecoration(ScreenDecoration::new(335.0, 215.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));


    only_interactable_components.insert(7, OnlyInteractable::Button(
        Button::new(182.0, 285.0, 300.0, 50.0,
            Color::new(0.5, 0.2, 0.2, 1.0),
            Color::new(0.8, 0.5, 0.5, 1.0),
            Color::new(0.3, 0.01, 0.01, 1.0),
            Box::new(SSHTest),
            TextBlock::new(227.0, 315.0, Color::new(1.0, 1.0, 1.0, 1.0), "Login".to_string(), 20.0)
        )
    ));
    non_interactable_components.insert(107,  NonInteractable::ScreenDecoration(ScreenDecoration::new(177.0, 280.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(8, OnlyInteractable::TextBox(
        TextBox::new(25.0, 415.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Command".to_string(),
            Box::new(ExecuteCommand),
            TextBlock::new(35.0, 445.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            true,
            false
        )
    ));
    non_interactable_components.insert(108,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 410.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(9, OnlyInteractable::TextBox(
        TextBox::new(25.0, 480.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter a file to download".to_string(),
            Box::new(DownloadFile),
            TextBlock::new(35.0, 510.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false, //Password mode
            true,  //Enter clears text 
            true   //Enter removes focus
        )
    ));
    non_interactable_components.insert(109,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 475.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));


    only_interactable_components.insert(10, OnlyInteractable::TextBox(
        TextBox::new(25.0, 545.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter a file to upload".to_string(),
            Box::new(UploadFile),
            TextBlock::new(35.0, 575.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false, //Password mode
            true,  //Enter clears text 
            true   //Enter removes focus
        )
    ));
    non_interactable_components.insert(110,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 540.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    only_interactable_components.insert(11, OnlyInteractable::TextBox(
        TextBox::new(25.0, 610.0, 300.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter a directory to upload".to_string(),
            Box::new(UploadDirectory),
            TextBlock::new(35.0, 640.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false, //Password mode
            true,  //Enter clears text 
            true   //Enter removes focus
        )
    ));
    non_interactable_components.insert(111,  NonInteractable::ScreenDecoration(ScreenDecoration::new(20.0, 605.0, 310.0, 60.0, Color::new(0.05, 0.05, 0.05, 1.0))));

    non_interactable_components.insert(51,  NonInteractable::ScreenDecoration(ScreenDecoration::new(9.0, 9.0, 664.0, 342.0, Color::new(0.2, 0.2, 0.2, 1.0))));
    non_interactable_components.insert(52, NonInteractable::ScreenDecoration(ScreenDecoration::new(9.0, 399.0, 664.0, 492.0, Color::new(0.2, 0.2, 0.2, 1.0))));

    non_interactable_components.insert(49, NonInteractable::ScreenDecoration(ScreenDecoration::new(681.0, 49.0, 1082.0, 842.0, Color::new(0.2, 0.2, 0.2, 1.0))));
    
    non_interactable_components.insert(50, NonInteractable::Logger(Logger::new(682.0, 50.0, 1080.0, 840.0, 5.0, 20.0, Color::new(1.0, 1.0, 1.0, 1.0), "".to_string())));

    non_interactable_components.insert(71, NonInteractable::TextBlock(TextBlock::new(1161.0, 33.0, Color::new(0.05, 0.05, 0.05, 1.0), "Logger".to_string(), 46.0)));

    non_interactable_components.insert(54, NonInteractable::ScreenDecoration(ScreenDecoration::new(335.0, 475.0, 310.0, 125.0, Color::new(0.05, 0.05, 0.05, 1.0))));
    non_interactable_components.insert(55, NonInteractable::ScreenDecoration(ScreenDecoration::new(340.0, 480.0, 300.0, 115.0, Color::new(0.5, 0.2, 0.2, 1.0))));
    
    non_interactable_components.insert(72, NonInteractable::TextBlock(TextBlock::new(350.0, 510.0, Color::new(0.05, 0.05, 0.05, 1.0), "Warning".to_string(), 40.0)));
    non_interactable_components.insert(73, NonInteractable::TextBlock(TextBlock::new(350.0, 535.0, Color::new(0.05, 0.05, 0.05, 1.0), "SFTP actions are a thread-".to_string(), 20.0)));
    non_interactable_components.insert(74, NonInteractable::TextBlock(TextBlock::new(350.0, 555.0, Color::new(0.05, 0.05, 0.05, 1.0), "blocking action".to_string(), 20.0)));
    non_interactable_components.insert(75, NonInteractable::TextBlock(TextBlock::new(350.0, 575.0, Color::new(0.05, 0.05, 0.05, 1.0), "The screen may go unresponsive ".to_string(), 20.0)));

    hidden_components.insert(100, HiddenManager::SSHClient(SSHClient::new()));
}
