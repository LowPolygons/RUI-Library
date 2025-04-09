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
use crate::interactable_implementations::textbox_implementation::AddLogLine;
use crate::interactable_implementations::textbox_implementation::DoNothing;
use crate::interactable_implementations::textbox_implementation::ExecuteCommand;      

/*--===--===--===--===--===--===--===--===--===--*\
|           Defining graphics Components          v
|
|  - All IDs must be unique. Ideally, keep them unique across even across both non/only interactables structures
|  - For Buttons that toggle on or off raytracers, the button id must be exactly 1 id less
|                                                 
|                                                 ^
\*--===--===--===--===--===--===--===--===--===--*/

pub fn init_graphics_objects_main(non_interactable_components: &mut BTreeMap<u32, NonInteractable>, only_interactable_components: &mut BTreeMap<u32, OnlyInteractable>, hidden_components: &mut BTreeMap<u32, HiddenManager>) {
    only_interactable_components.insert(71, OnlyInteractable::TextBox(
        TextBox::new(20.0, 20.0, 310.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Hostname".to_string(),
            Box::new(DoNothing),
            TextBlock::new(30.0, 50.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            false
        )
    ));

    only_interactable_components.insert(72, OnlyInteractable::TextBox(
        TextBox::new(20.0, 70.0, 310.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Username".to_string(),
            Box::new(DoNothing),
            TextBlock::new(30.0, 100.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            false
        )
    ));

    only_interactable_components.insert(73, OnlyInteractable::TextBox(
        TextBox::new(20.0, 120.0, 310.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Password".to_string(),
            Box::new(DoNothing),
            TextBlock::new(30.0, 150.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            true,
            false
        )
    ));

    only_interactable_components.insert(74, OnlyInteractable::Button(
        Button::new(20.0, 170.0, 310.0, 50.0,
            Color::new(0.5, 0.2, 0.2, 1.0),
            Color::new(0.8, 0.5, 0.5, 1.0),
            Color::new(0.3, 0.01, 0.01, 1.0),
            Box::new(SSHTest),
            TextBlock::new(70.0, 200.0, Color::new(1.0, 1.0, 1.0, 1.0), "Login".to_string(), 20.0)
        )
    ));

    only_interactable_components.insert(75, OnlyInteractable::TextBox(
        TextBox::new(20.0, 250.0, 310.0, 50.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Enter Command".to_string(),
            Box::new(ExecuteCommand),
            TextBlock::new(30.0, 280.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0),
            false,
            true
        )
    ));

    non_interactable_components.insert(0,  NonInteractable::ScreenDecoration(ScreenDecoration::new(9.0, 9.0, 332.0, 882.0, Color::new(0.2, 0.2, 0.2, 1.0))));
    non_interactable_components.insert(33, NonInteractable::ScreenDecoration(ScreenDecoration::new(349.0, 49.0, 1082.0, 842.0, Color::new(0.2, 0.2, 0.2, 1.0))));
    non_interactable_components.insert(50, NonInteractable::Logger(Logger::new(350.0, 50.0, 1080.0, 840.0, 5.0, 20.0, Color::new(1.0, 1.0, 1.0, 1.0), "".to_string())));

    non_interactable_components.insert(51, NonInteractable::TextBlock(TextBlock::new(829.0, 33.0, Color::new(0.05, 0.05, 0.05, 1.0), "Logger".to_string(), 46.0)));

    hidden_components.insert(100, HiddenManager::SSHClient(SSHClient::new()));
}
