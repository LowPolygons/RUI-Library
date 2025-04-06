use macroquad::prelude::*;
use std::collections::BTreeMap;

// Non Interactables
use crate::window_objects::window_object_center::NonInteractable;

// Only Interactables 
use crate::window_objects::window_object_center::OnlyInteractable;
use crate::window_objects::window_object_center::WindowObjectMethods;

use crate::init_graphics_objects::init_graphics_objects_main;


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
}

// Methods that are necessary for the structure itself, and therefore seperable from the Methods trait
impl WindowManager {
    pub fn new(w: f32, h: f32, r: f32, g: f32, b: f32, a: f32) -> Self {
        WindowManager {
            screen_width: w,
            screen_height: h,
            main_window_colour: Color::new(r, g, b, a),
            non_interactable_components: BTreeMap::new(),
            only_interactable_components: BTreeMap::new(),
        }
    }
    
    // Getter And Setter for the Non interactables
    pub fn get_non_interactable_graphics_components(&self) -> BTreeMap<u32, NonInteractable> {
        self.non_interactable_components.clone()
    }

    pub fn set_non_interactable_graphics_components(&mut self, value: BTreeMap<u32, NonInteractable>) {
        self.non_interactable_components = value;
    }
    
    // Getter [and setter] for the Only Interactables as a Mutable reference, as Box<dyns> cannot implement copy or clone 
    pub fn get_only_interactable_graphics_components(&mut self) -> &mut BTreeMap<u32, OnlyInteractable> {
        &mut self.only_interactable_components
    }
}

impl WindowManagerMethods for WindowManager {
    fn init(&mut self) {
        // Updates the screen size to be the set width and height (by default it is 800x600)    
        request_new_screen_size(self.screen_width, self.screen_height);
      
        // This method adds the graphics components. It was moved to a separate file 
        init_graphics_objects_main(&mut self.non_interactable_components, &mut self.only_interactable_components);

        // Call the init functions for the graphics components
        for (_id, component) in &mut self.non_interactable_components {
           component.init(); 
        }

        for (_id, component) in &mut self.only_interactable_components {
           component.init(); 
        }

    }

    fn update(&mut self) {
        clear_background(self.main_window_colour);

        self.main_window_colour.r = (self.main_window_colour.r + 0.005) % 1.0;
        self.main_window_colour.g = (self.main_window_colour.g + 0.001) % 1.0;
        self.main_window_colour.b = (self.main_window_colour.b + 0.002) % 1.0;

        // Then call the graphics components Updates 
        for (_id, component) in &mut self.non_interactable_components {
           component.update(); 
        }

        //println!("Frame");
        for (_id, component) in &mut self.only_interactable_components {
           component.update(); 
        }
    }
}

