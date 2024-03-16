use std::io;
use std::fs::{self};
use std::path::{Path, PathBuf};


pub fn index_files_fs(path_str: &str) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = vec![];
    match __index_files_fs(&Path::new(path_str), &mut result) {
        Ok(_) => {
            result.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
            result
        }
        Err(_) => {
            println!("COOL ERROR!");
            vec![]
        }
    }
}

fn __index_files_fs(dir: &Path, result: &mut Vec<(String, String)>) -> io::Result<()> {
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
            __index_files_fs(&path, result)?;
        } else if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            result.push((file_name.to_string(), path.to_string_lossy().to_string()));
        }
    }
    Ok(())
}




use std::ffi::{CString, CStr};
use libc::{opendir, readdir, closedir, DIR, dirent, c_char, c_void};


pub fn index_files_libc(dir_path: &str) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = vec![];
    match __index_files_libc(dir_path, &mut result) {
        Ok(_) => {
            //A PERFECTIONNER ABSOLUMENT
            result.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
            result
        }
        Err(_) => {
            println!("CRITICAL ERROR!");
            vec![]
        }
    }
}


fn __index_files_libc(dir_path: &str, files_list: &mut Vec<(String, String)>) -> io::Result<()> {
    let dir_path_c = CString::new(dir_path)?;
    let dir_ptr = unsafe { opendir(dir_path_c.as_ptr()) };
    if !dir_ptr.is_null() {
        unsafe {
            loop {
                let entry = readdir(dir_ptr);
                if entry.is_null() {
                    break;
                }

                let entry_deref = *entry;
                let file_name = CStr::from_ptr(entry_deref.d_name.as_ptr()).to_string_lossy().into_owned();
                if file_name != "." && file_name != ".." {
                    let entry_path = PathBuf::from(dir_path).join(&file_name);
                    let entry_path_str = entry_path.to_string_lossy().into_owned();
                    files_list.push((file_name.clone(),entry_path_str.clone()));
                    if entry_deref.d_type == libc::DT_DIR {
                        __index_files_libc(&entry_path_str, files_list)?;
                    }
                }
            }

            if closedir(dir_ptr) != 0 {
                return Err(io::Error::last_os_error());
            }
        }

        }

    Ok(())
}

