pub mod window_object_center;

pub mod logger_object;
pub mod text_block_object;
pub mod raytracer_window_object;
pub mod screen_decoration_object;

pub mod button_object;
pub mod textbox_object;
pub mod sshclient_object;

use std::path::Path; 
use std::fs;
use std::fs::File;
// To be paired with .contains()
pub const ALLOWED_CHARACTERS: &str = "1234567890-=!@#$%^&*()_+qwertyuiop[]\\QWERTYUIOP{}|asdfghjkl:'ASDFGHJKL;\"zxcvbnm,./ZXCVBNM<>? ~";
pub const WIDEST_CHARACTER_PIXEL_WIDTH: f32 = 9.0;

pub fn get_files_in_directory(dir: &str) -> Result<(Vec<String>, Vec<String>), String> {
    let path_attempt = Path::new(dir);
    let mut items: Vec<String> = Vec::<String>::new();
    let mut directories: Vec<String> = Vec::<String>::new();
    
    directories.push(dir.to_string());

    if !path_attempt.exists() {
        return Err("[SSH WARN] Path does not exist".to_string());
    }
    
    let things_in_dir = fs::read_dir(path_attempt)
        .map_err(|_| "[SSH WARN] Could not read directory".to_string())?;

    for item in things_in_dir {
        let item = item.map_err(|_| "[SHH WARN] Item not valid in directory".to_string())?;
        let path: String = item.path().to_string_lossy().to_string();
        let item_type = item.file_type()
            .map_err(|_| "[SSH WARN] Couldnt get type of file, likely doesnt exist")?;

        //Check if it is a directory
        if item_type.is_dir() {
            directories.push(path.clone());

            let list_of_files: (Vec<String>, Vec<String>) = get_files_in_directory(&path)
                .map_err(|err| err)?;
            
            //Push to return val
            for val in list_of_files.0 {
                items.push(val);
            }
            for val in list_of_files.1 {
                directories.push(val);
            }

        } else if item_type.is_file() {
            items.push(path);
        }
    }

    Ok((items, directories))
}

pub fn is_directory(dir: &str) -> bool {
    let path_attempt = Path::new(dir);

    path_attempt.is_dir()
}
