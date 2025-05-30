use macroquad::prelude::*;
use std::collections::BTreeMap;

// Non Interactables
use crate::window_objects::window_object_center::NonInteractable;

// Only Interactables 
use crate::window_objects::window_object_center::OnlyInteractable;
use crate::window_objects::window_object_center::WindowObjectMethods; 

use crate::window_objects::window_object_center::HiddenManager;
use crate::window_objects::window_object_center::HiddenObjectMethods;

use crate::init_graphics_objects::init_graphics_objects_main;

use crate::InputArgs;

use clap::Parser;

/*--===--===--===--===--===--===--===--===--===--*\
|     Main Unimplemented Structure and Trait      | 
\*--===--===--===--===--===--===--===--===--===--*/

pub trait WindowManagerMethods {
    fn init(&mut self);
    fn update(&mut self); 
}

pub struct WindowManager {
    // Primitive Values
    screen_width: f32,
    screen_height: f32,
    main_window_colour: Color,

    // Graphical Components
    non_interactable_components:  BTreeMap<u32,  NonInteractable>,
    only_interactable_components: BTreeMap<u32, OnlyInteractable>,
    hidden_components: BTreeMap<u32, HiddenManager>, 
}

// Methods that are necessary for the structure itself, and therefore seperable from the Methods trait
impl WindowManager {
    pub fn new(w: i32, h: i32, r: f32, g: f32, b: f32, a: f32) -> Self {
        WindowManager {
            screen_width: w as f32,
            screen_height: h as f32,
            main_window_colour: Color::new(r, g, b, a),
            non_interactable_components: BTreeMap::new(),
            only_interactable_components: BTreeMap::new(),
            hidden_components: BTreeMap::new(),
        }
    }
    
    // Getter And Setter for the Non interactables
    pub fn get_non_interactable_graphics_components(&self) -> BTreeMap<u32, NonInteractable> {
        self.non_interactable_components.clone()
    }

    pub fn set_non_interactable_graphics_components(&mut self, value: BTreeMap<u32, NonInteractable>) {
        self.non_interactable_components = value;
    }

    pub fn get_pair_of_graphics_components(&mut self) -> (&mut BTreeMap<u32, OnlyInteractable>, &mut BTreeMap<u32, HiddenManager>) {
        (&mut self.only_interactable_components, &mut self.hidden_components)
    }
}

impl WindowManagerMethods for WindowManager {
    fn init(&mut self) {
        // Updates the screen size to be the set width and height (by default it is 800x600)    
        request_new_screen_size(self.screen_width, self.screen_height);
      
        // This method adds the graphics components. It was moved to a separate file 
        init_graphics_objects_main(&mut self.non_interactable_components, &mut self.only_interactable_components, &mut self.hidden_components);

        // Call the init functions for the graphics components
        for (_id, component) in &mut self.non_interactable_components {
           component.init(); 
        }

        for (_id, component) in &mut self.only_interactable_components {
           component.init(); 
        }

        for (_id, component) in &mut self.hidden_components {
            component.init();
        }

        // Format the input args
        let input_args = InputArgs::parse();

        if let Some(hostname) = input_args.hostname {
            if let Some(OnlyInteractable::TextBox(obj)) = self.only_interactable_components.get_mut(&1) {
                obj.force_set_text(hostname);
            }
        }

        if let Some(username) = input_args.username {
            if let Some(OnlyInteractable::TextBox(obj)) = self.only_interactable_components.get_mut(&2) {
                obj.force_set_text(username);
            }
        }

        if let Some(public) = input_args.public {
            if let Some(OnlyInteractable::TextBox(obj)) = self.only_interactable_components.get_mut(&4) {
                obj.force_set_text(public);
            }
        }

        if let Some(private) = input_args.private {
            if let Some(OnlyInteractable::TextBox(obj)) = self.only_interactable_components.get_mut(&5) {
                obj.force_set_text(private);
            }
        }

        if let Some(passphrase) = input_args.passphrase {
            if let Some(OnlyInteractable::TextBox(obj)) = self.only_interactable_components.get_mut(&6) {
                obj.force_set_text(passphrase);
            }
        }
    }

    fn update(&mut self) {
        clear_background(self.main_window_colour);

        // Then call the graphics components Updates 
        for (_id, component) in &mut self.non_interactable_components {
            component.update(); 
        }

        for (_id, component) in &mut self.only_interactable_components {
            component.update(); 
        }

        for (_id, component) in &mut self.hidden_components {
            component.update(&mut self.only_interactable_components, &mut self.non_interactable_components);
        }
    }
}

