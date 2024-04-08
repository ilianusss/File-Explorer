use std::fs;
use std::env;

//     GLOBAL
// CHANGE DIRECTORY
pub fn change_d(path:&str){
    if let Err(erreur) = env::set_current_dir(path) {
        eprintln!("Échec du changement de répertoire de travail : {}", erreur);
        return;
    }
}



//     DIRECTORY
// CREATE DIRECTORY
pub fn create_dir(path: &str, name: &str) {
    let new_path = format!("{}/{}", path, name);
    if let Err(err) = fs::create_dir(new_path) {
        eprintln!("Error creating directory: {}", err);
    }
}

// REMOVE DIRECTORY
pub fn remove_dir(path:&str){
    let new_path = format!("{}", path);
    if let Err(err) = fs::remove_dir_all(new_path) {
        eprintln!("Error creating directory: {}", err);
    }
}



//     FILE
// CREATE FILE
pub fn create_file(path:&str, name:&str){
    todo!();
}

// REMOVE FILE
pub fn remove_file(path: &str){
    let new_path = format!("{}", path);
    if let Err(err) = fs::remove_file(new_path) {
        eprintln!("Error creating directory: {}", err);
    }
}

// RENAME FILE
pub fn rename(current_name:&str, new_name:&str){
    if let Err(err) = fs::rename(current_name, new_name) {
        eprintln!("Error renaming file: {}", err);
    }
}

// FILE METADATA
pub fn metadata(path:&str, name:&str){
    todo!();
}
