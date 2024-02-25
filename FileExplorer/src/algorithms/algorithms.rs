use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

pub fn get_paths(path_str: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    __get_paths(&Path::new(path_str), &mut result);
    result.sort();
    result
}

fn __get_paths(dir: &Path, result: &mut Vec<String>) -> io::Result<()>{
    result.push(dir.file_name().unwrap().to_str().unwrap().to_string());
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                __get_paths(&path, result)?;
            }
        }
    }
    Ok(())
}

pub fn search_filename(filename: &str, path: &str) -> (bool, usize) {
    let result = get_paths(path).binary_search(&filename.to_string());
    if result.is_ok() {
        return (true, result.unwrap());
    }
    dbg!(result);
    (false, 0)
}