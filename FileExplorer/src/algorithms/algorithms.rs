use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;


pub fn get_paths(path_str: &str) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = vec![];
    match __get_paths(&Path::new(path_str), &mut result) {
        Ok(_) => {
            result.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
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


fn binary_search(slice: &[(String,String)], search_key: &String) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = slice.len();

    while left < right {
        let mid = left + (right - left) / 2;
        let mid_str = &slice[mid];

        if mid_str.0.to_lowercase().starts_with(search_key) {
            return Ok(mid);
        }

        match mid_str.0.to_lowercase().cmp(search_key) {
            std::cmp::Ordering::Less => {
                left = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                right = mid;
            }
            std::cmp::Ordering::Equal => {
                return Ok(mid);
            }
        }
    }
    Err(left)
}


pub fn search_filename(filename: &str, files: &Vec<(String,String)>) -> Vec<String> {
    println!("Searching {} in {} files", filename, files.len());
    let search_key = filename.to_string().to_lowercase();

    let index = match binary_search(&files, &search_key) {
        Ok(idx) => idx as i32,
        Err(_) => -1 as i32,
    };

    if index==-1 {return vec![];}
    else {
        let mut result = vec![];
        let mut i: i32 = index;
        while i>=0 && files[i as usize].0.to_lowercase().contains(&search_key) {
            result.push(files[i as usize].1.clone());
            i-=1;
        }
        i = index+1;
        let len = files.len() as i32;
        while i<len && files[i as usize].0.to_lowercase().contains(&search_key) {
            result.push(files[i as usize].1.clone());
            i+=1;
        }

        // Sort is managed in main.rs (to be able to use metadatas & cie)
        //result.sort_by_key(|path| path.matches('/').count());
        result
    }
}
