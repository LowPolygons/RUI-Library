use macroquad::prelude::*;
use std::collections::BTreeMap;

// Non Interactables
use crate::window_object::NonInteractable;
use crate::window_object::RaytracerWindow;
use crate::window_object::ScreenDecoration;
use crate::window_object::TextBlock;

// Only Interactables 
use crate::window_object::OnlyInteractable;
use crate::window_object::Button;
use crate::window_object::TextBox;

// Any Button Implementations Go Here
use crate::button_implementations::ToggleRaytracer;
use crate::button_implementations::SSHTest;

use crate::textbox_implementation::Test;
        
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
            Box::new(Test),
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
    non_interactable_components.insert(5, NonInteractable::RaytracerWindow(RaytracerWindow::new(349.0, 9.0, 1082.0, 882.0, Color::new(0.0, 0.0, 0.0, 1.0))));
    non_interactable_components.insert(10, NonInteractable::RaytracerWindow(RaytracerWindow::new(350.0, 10.0, 1080.0, 880.0, Color::new(0.0, 0.0, 0.0, 1.0)))); 
    non_interactable_components.insert(0,  NonInteractable::ScreenDecoration(ScreenDecoration::new(10.0, 10.0, 330.0, 880.0, Color::new(0.2, 0.2, 0.2, 1.0))));
}
