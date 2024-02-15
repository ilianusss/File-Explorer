use std::fs;


// DIRECTORY
pub fn create_dir(path:str, name:str){
    let new_path = path + name;
    match fs::create_dir(new_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(err) => eprintln!("Error creating directory: {}", err),
    }
}

pub fn remove_dir(path:str, name:str){
    let new_path = path + name;
    match fs::remove_dir_all(new_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(err) => eprintln!("Error creating directory: {}", err),
    }
}

pub fn create_file(path:str, name:str){
    todo!();
}

pub fn remove_file(path:str, name:str){
    let new_path = path + name;
    match fs::remove_file(new_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(err) => eprintln!("Error creating directory: {}", err),
    }
}

pub fn rename(path:str, name:str){
    todo!();
}

pub fn metadata(path:str, name:str){
    todo!();
}
