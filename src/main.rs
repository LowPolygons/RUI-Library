use macroquad::prelude::*;
use macroquad::window::Conf;
use crate::miniquad::conf::Icon;

mod window_objects;
mod interactable_implementations;
mod managers;
mod assets; 
mod object_ids;
mod init_graphics_objects;

use crate::managers::user_interaction::UserInteractionManager;
use crate::managers::user_interaction::UserInteractionManagerMethods;

use crate::managers::main_window_manager::WindowManagerMethods;
use crate::managers::main_window_manager::WindowManager;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "trssh")]
pub struct InputArgs {
    // First input
    #[arg(long)]
    hostname: Option<String>,

    #[arg(long)]
    username: Option<String>,

    #[arg(long)]
    public: Option<String>,

    #[arg(long)]
    private: Option<String>,

    #[arg(long)]
    passphrase: Option<String>,
}

const SCREEN_WIDTH: i32 = 1772;
const SCREEN_HEIGHT: i32 = 900;
const SCREEN_R: f32 = 0.7647;
const SCREEN_G: f32 = 0.235;
const SCREEN_B: f32 = 0.235;
const SCREEN_A: f32 = 1.0;

fn trssh_info() -> Conf {
    Conf {
        window_title: "TRSSH".to_string(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        fullscreen: false,
        window_resizable: false,
        icon: Some(Icon {
            small: assets::icon_data::SMALL_ICON,
            medium: assets::icon_data::MEDIUM_ICON,
            big: assets::icon_data::BIG_ICON,
        }),
        ..Default::default()
    }
}

#[cfg(target_os = "windows")]
fn set_autostart(app_name: &str) {
    use std::env;
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";

    let (key, _) = hkcu.create_subkey(path)
        .expect("Failed to open registry key");

    let exe_path = env::current_exe()
        .expect("Can't find path to current executable");

    key.set_value(app_name, &exe_path.to_str().unwrap())
        .expect("Failed to set registry key value");

    println!("Startup entry set for '{}'", app_name);
}


#[macroquad::main(trssh_info)]
async fn main() {
    // =-=-=-=== General Initialisation ===-=-=-=//
    set_autostart("TRSSH");

    //  Window Manager handles graphics for the entire window
    let mut window_manager = WindowManager::new(SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_R, SCREEN_G, SCREEN_B, SCREEN_A);
    //  User Interaction Manager handles how the user and ui will interact
    let mut user_interation_manager = UserInteractionManager::new();

    // =-=-=-=== Init Methods ===-=-=-=//
    
    // May be redundant for some, but in case any structures need any Initialisation
    window_manager.init(); 
    user_interation_manager.init();

    // =-=-=-=== Main program Loop ===-=-=-=//
    loop {
        // Interactions require the window manager hence the mutable reference
        user_interation_manager.update(&mut window_manager);

        window_manager.update();

        // Async as it allows all things to finish before moving onto the next frame, much like
        // MPI_BARRIER does
        next_frame().await;
    }
}
