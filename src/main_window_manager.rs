use macroquad::prelude::*;


use std::collections::HashMap;
use std::collections::BTreeMap;

use crate::window_object::WindowObject;

use crate::window_object::RaytracerWindow;
use crate::window_object::ScreenDecoration;
use crate::window_object::Button;
use crate::window_object::WindowObjectMethods;

//For box dyn type when creating a button
use crate::button_implementations::ToggleRaytracer;


//TODO: consider refactoring so that graphics components arent all stored in one big BTreeMap,
//separate buttons 
//
//
pub trait WindowManagerMethods {
    fn init(&mut self);
    fn update(&mut self); 
}

pub struct WindowManager {
    screen_width: f32,
    screen_height: f32,
    main_window_colour: Color,
    graphics_components: BTreeMap<u32, WindowObject>,
}

impl WindowManager {
    pub fn new(w: f32, h: f32, r: f32, g: f32, b: f32, a: f32) -> Self {
        WindowManager {
            screen_width: w,
            screen_height: h,
            main_window_colour: Color::new(r, g, b, a),
            graphics_components: BTreeMap::new(),
        }
    }

    pub fn get_graphics_components(&mut self) -> &mut BTreeMap<u32, WindowObject> {
        &mut self.graphics_components   
    }

    pub fn set_graphics_components(&mut self, value: BTreeMap<u32, WindowObject>) {
        self.graphics_components = value;
    }
}

impl WindowManagerMethods for WindowManager {
    fn init(&mut self) {
        //Updates the screen size    
        request_new_screen_size(self.screen_width, self.screen_height);
        
        //Create the desired graphics components

        //A button that corresponds to a raytracer window must have exactly 1 less key 

        self.graphics_components.insert(9, WindowObject::Button(
                Button::new(20.0, 20.0, 310.0, 100.0,
                    Color::new(0.5, 0.05, 0.05, 1.0),
                    Color::new(0.6, 0.1, 0.1, 1.0),
                    Color::new(0.4, 0.01, 0.01, 1.0),
                    Box::new(ToggleRaytracer)
                )
        ));
        self.graphics_components.insert(10, WindowObject::RaytracerWindow(RaytracerWindow::new(350.0, 10.0, 1080.0, 880.0, Color::new(0.0, 0.0, 0.0, 1.0))));
       
        self.graphics_components.insert(0, WindowObject::ScreenDecoration(ScreenDecoration::new(10.0, 10.0, 330.0, 880.0, Color::new(0.2, 0.2, 0.2, 1.0))));

        //Init the graphics components
        for (id, component) in &self.graphics_components {
           component.init(); 
        }
    }

    fn update(&mut self) {
        clear_background(self.main_window_colour);

        self.main_window_colour.r = (self.main_window_colour.r + 0.005) % 1.0;
        self.main_window_colour.g = (self.main_window_colour.g + 0.001) % 1.0;
        self.main_window_colour.b = (self.main_window_colour.b + 0.002) % 1.0;

        //Then call the graphics components Updates 
        for (id, component) in &mut self.graphics_components {
            component.update();
        }
    }
}

