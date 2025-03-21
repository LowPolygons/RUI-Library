use macroquad::prelude::*;

mod user_interaction;
mod main_window_manager;
mod window_object;
mod button_implementations;

use crate::user_interaction::UserInteractionManager;
use crate::user_interaction::UserInteractionManagerMethods;

use crate::main_window_manager::WindowManagerMethods;
use crate::main_window_manager::WindowManager;

#[macroquad::main("Raytracer")]
async fn main() {
    //=-=-=-=== General Initialisation ===-=-=-=//

    //  Window Manager handles graphics for the entire window
    let mut window_manager = WindowManager::new(1440.0, 900.0, 0.1, 0.1, 0.1, 1.0);
    let mut user_interation_manager = UserInteractionManager::new();

    window_manager.init(); 
    user_interation_manager.init();

    //Main program loop
    loop {
        window_manager.update();
        user_interation_manager.update(&mut window_manager);

        next_frame().await;
    }
}
