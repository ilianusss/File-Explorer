use std::fs;
use std::env;

pub fn change_d(path:&str){
    if let Err(erreur) = env::set_current_dir(path) {
        eprintln!("Échec du changement de répertoire de travail : {}", erreur);
        return;
    }
}

pub fn create_dir(path:&str, name:&str){
    let new_path = format!("{}{}", path, name);
    match fs::create_dir(new_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(err) => eprintln!("Error creating directory: {}", err),
    }
}

pub fn remove_dir(path:&str, name:&str){
    let new_path = format!("{}{}", path, name);
    match fs::remove_dir_all(new_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(err) => eprintln!("Error creating directory: {}", err),
    }
}

pub fn create_file(path:&str, name:&str){
    todo!();
}

pub fn remove_file(path:&str, name:&str){
    let new_path = format!("{}{}", path, name);
    match fs::remove_file(new_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(err) => eprintln!("Error creating directory: {}", err),
    }
}

pub fn rename(path:&str, name:&str){
    todo!();
}

pub fn metadata(path:&str, name:&str){
    todo!();
}
