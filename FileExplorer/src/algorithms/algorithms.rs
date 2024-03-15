use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

pub fn get_paths(path_str: &str) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = vec![];
    match __get_paths(&Path::new(path_str), &mut result) {
        Ok(_) => {
            result.sort_by(|a, b| a.0.cmp(&b.0));
            result
        }
        Err(_) => {
            println!("ERROR!");
            vec![]
        }
    }
}

fn __get_paths(dir: &Path, result: &mut Vec<(String, String)>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.file_type().is_symlink() {
            continue;
        }
        
        let path = entry.path();
        if path.is_dir() {
            result.push((
                path.file_name().unwrap().to_string_lossy().to_string(),
                path.to_string_lossy().to_string(),
            ));
            __get_paths(&path, result)?;
        } else if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            result.push((file_name.to_string(), path.to_string_lossy().to_string()));
        }
    }
    Ok(())
}


pub fn search_filename(filename: &str, files: &Vec<(String,String)>) -> Vec<String> {
    println!("Searching {} in {} files", filename, files.len());
    let search_key = filename.to_string();

    let index: i32 = match files.binary_search_by(|(a, _)| a.cmp(&search_key)) {
        Ok(index) => index as i32,
        _ => -1
    };
    if index==-1 {return vec![];}
    else {
        let search_key: String = files[index as usize].0.clone();
        let mut result = vec![];
        let mut i: i32 = index;
        while i>=0 && files[i as usize].0.contains(&search_key) {
            result.push(files[i as usize].1.clone());
            i-=1;
        }
        i = index+1;
        let len = files.len() as i32;
        while i<len && files[i as usize].0.contains(&search_key) {
            result.push(files[i as usize].1.clone());
            i+=1;
        }

        println!("{} files found", result.len());
        result
    }
}
