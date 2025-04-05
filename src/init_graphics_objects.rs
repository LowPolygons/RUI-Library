use macroquad::prelude::*;
use std::collections::BTreeMap;

// Non Interactables
use crate::window_objects::window_object_center::NonInteractable;
use crate::window_objects::raytracer_window_object::RaytracerWindow;
use crate::window_objects::screen_decoration_object::ScreenDecoration;
use crate::window_objects::text_block_object::TextBlock;
use crate::window_objects::logger_object::Logger;

// Only Interactables 
use crate::window_objects::window_object_center::OnlyInteractable;
use crate::window_objects::button_object::Button;
use crate::window_objects::textbox_object::TextBox;

// Any Button Implementations Go Here
use crate::interactable_implementations::button_implementations::ToggleRaytracer;
use crate::interactable_implementations::button_implementations::SSHTest;

// Any Textbox Implementations Go Here
use crate::interactable_implementations::textbox_implementation::AddLogLine;
        
/*--===--===--===--===--===--===--===--===--===--*\
|           Defining graphics Components          v
|
|  - All IDs must be unique. Ideally, keep them unique across even across both non/only interactables structures
|  - For Buttons that toggle on or off raytracers, the button id must be exactly 1 id less
|                                                 
|                                                 ^
\*--===--===--===--===--===--===--===--===--===--*/

pub fn init_graphics_objects_main(non_interactable_components: &mut BTreeMap<u32, NonInteractable>, only_interactable_components: &mut BTreeMap<u32, OnlyInteractable>) {
    only_interactable_components.insert(9, OnlyInteractable::Button(
        Button::new(20.0, 40.0, 310.0, 50.0,
            Color::new(0.5, 0.2, 0.2, 1.0),
            Color::new(0.8, 0.5, 0.5, 1.0),
            Color::new(0.3, 0.01, 0.01, 1.0),
            Box::new(ToggleRaytracer),
            TextBlock::new(70.0, 70.0, Color::new(1.0, 1.0, 1.0, 1.0), "Toggle Raytracer On/Off".to_string(), 20.0)
        )
    ));

    only_interactable_components.insert(30, OnlyInteractable::TextBox(
        TextBox::new(20.0, 200.0, 310.0, 50.0,
            Color::new(0.0,0.0,0.0,1.0),
            Color::new(0.9, 0.9, 0.9, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
            "Click to type!".to_string(),
            Box::new(AddLogLine),
            TextBlock::new(30.0, 230.0, Color::new(0.0, 0.0, 0.0, 1.0), String::new(), 20.0)
        )
    ));

    only_interactable_components.insert(20, OnlyInteractable::Button(
        Button::new(20.0, 120.0, 310.0, 50.0,
            Color::new(0.5, 0.2, 0.2, 1.0),
            Color::new(0.8, 0.5, 0.5, 1.0),
            Color::new(0.3, 0.01, 0.01, 1.0),
            Box::new(SSHTest),
            TextBlock::new(70.0, 150.0, Color::new(1.0, 1.0, 1.0, 1.0), "SSH Test".to_string(), 20.0)
        )
    ));
    non_interactable_components.insert(5,  NonInteractable::ScreenDecoration(ScreenDecoration::new(1049.0, 9.0, 382.0, 882.0, Color::new(0.2, 0.2, 0.2, 1.0))));
    non_interactable_components.insert(10, NonInteractable::RaytracerWindow(RaytracerWindow::new(1050.0, 10.0, 380.0, 880.0, Color::new(0.0, 0.0, 0.0, 1.0))));
    non_interactable_components.insert(0,  NonInteractable::ScreenDecoration(ScreenDecoration::new(9.0, 9.0, 332.0, 882.0, Color::new(0.2, 0.2, 0.2, 1.0))));
    non_interactable_components.insert(33, NonInteractable::ScreenDecoration(ScreenDecoration::new(349.0, 9.0, 692.0, 882.0, Color::new(0.2, 0.2, 0.2, 1.0))));
    non_interactable_components.insert(50, NonInteractable::Logger(Logger::new(350.0, 10.0, 690.0, 880.0, 5.0, 20.0, Color::new(1.0, 1.0, 1.0, 1.0), ">>> ".to_string())));
}
