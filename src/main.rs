use macroquad::prelude::*;

mod window_objects;
mod interactable_implementations;
mod managers;

mod init_graphics_objects;

use crate::managers::user_interaction::UserInteractionManager;
use crate::managers::user_interaction::UserInteractionManagerMethods;

use crate::managers::main_window_manager::WindowManagerMethods;
use crate::managers::main_window_manager::WindowManager;

#[macroquad::main("Raytracer")]
async fn main() {
    //=-=-=-=== General Initialisation ===-=-=-=//

    //  Window Manager handles graphics for the entire window
    let mut window_manager = WindowManager::new(1440.0, 900.0, 0.1, 0.1, 0.1, 1.0);
    //  User Interaction Manager handles how the user and ui will interact
    let mut user_interation_manager = UserInteractionManager::new();

    //=-=-=-=== Init Methods ===-=-=-=// 
    
    //May be redundant for some, but in case any structures need any Initialisation
    window_manager.init(); 
    user_interation_manager.init();

    //=-=-=-=== Main program Loop ===-=-=-=// 
    loop {
        // Interactions require the window manager hence the mutable reference
        user_interation_manager.update(&mut window_manager);

        window_manager.update();

        // Async as it allows all things to finish before moving onto the next frame, much like
        // MPI_BARRIER does
        next_frame().await;
    }
}
