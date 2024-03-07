use std::{env, fs, path::Path, ffi::OsStr, cell::RefCell, time::{Duration, SystemTime, UNIX_EPOCH}};
use gio::prelude::*;
use gio::{SettingsExt, AppInfo, AppInfoCreateFlags, AppLaunchContext, File, FileExt, AppLaunchContext as GioAppLaunchContext};
use gtk::prelude::*;
use gtk::{Application, Box, Orientation, ScrolledWindow, ListStore, TreeViewColumn, CellRendererText, Entry, Button, Label, SettingsExt as OtherSettingsExt, Window, WindowType};
use glib::{MainContext, clone};
use chrono::{DateTime, Local};
use std::rc::Rc;

// HEADER
    // Time Format
pub fn format_last_modified(modified_time: Option<Duration>) -> String {
    // Format last modified time in a human-readable format
    if let Some(modified) = modified_time {
        let local_time = DateTime::<Local>::from(UNIX_EPOCH + modified);
        local_time.format("%m/%d/%Y at %H:%M").to_string()
    } else {
        String::from("Unknown Modified Time")
    }
}

    // Update time
pub fn update_time_label(label: &Label) {
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    label.set_text(&current_time);
}





// MAIN PART
  // SETUP

    // Add columns to the tree
pub fn add_column(tree_view: &gtk::TreeView, title: &str, col_id: i32) {
    // Create a new tree view column with a cell renderer
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    // Pack the cell renderer into the column and set attributes
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", col_id);

    // Set the column title and append it to the tree view
    column.set_title(title);
    tree_view.append_column(&column);
}


    // Populate a list store
pub fn populate_list_store(list_store: &gtk::ListStore, dir_path: &str) {
    // Clear the list store
    list_store.clear();

    // Read directory entries and filter out errors
    let mut entries: Vec<_> = if let Ok(entries) = fs::read_dir(dir_path) {
        entries.filter_map(Result::ok).collect()
    } else {
        Vec::new()
    };

    // Sort entries alphabetically by name
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    // Add ".." entry for the parent directory
    if let Some(parent) = Path::new(dir_path).parent() {
        list_store.insert_with_values(
            None,
            &[0, 1, 2, 3],
            &[&"..".to_string(), &"Directory".to_string(), &"".to_string(), &"".to_string()],
        );
    }

    // Iterate through directory entries and populate the list store
    for entry in entries {
        if let Some(file_name) = entry.file_name().to_str() {
            let metadata = fs::metadata(entry.path()).ok();
            let file_type = get_file_type(&entry);
            let file_size = format_file_size(metadata.as_ref().map(|meta| meta.len()));
            let last_modified = metadata
                .and_then(|meta| meta.modified().ok())
                .map(|modified| modified.duration_since(UNIX_EPOCH).ok())
                .and_then(|modified_time| Some(format_last_modified(modified_time)));

            // Insert values into the list store
            list_store.insert_with_values(
                None,
                &[0, 1, 2, 3],
                &[&file_name, &file_type, &file_size, &last_modified.unwrap_or_default()],
            );
        }
    }
}


    // Get file type
pub fn get_file_type(entry: &std::fs::DirEntry) -> String {
    // Check file type and return as a string
    if let Ok(file_type) = entry.file_type() {
        if file_type.is_dir() {
            String::from("Directory")
        } else {
            String::from("File")
        }
    } else {
        String::from("Unknown")
    }
}



    // Best format for file size
pub fn format_file_size(size: Option<u64>) -> String {
    // Format file size based on magnitude
    match size {
    Some(size) if size < 1024 => format!("{} B", size),
    Some(size) if size < 1024 * 1024 => format!("{:.2} KB", size as f64 / 1024.0),
    Some(size) if size < 1024 * 1024 * 1024 => format!("{:.2} MB", size as f64 / (1024.0 * 1024.0)),
    Some(size) => format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0)),
    None => String::from("Unknown Size"),
    }
}




  // UPDATE
    // Open file
    /// A FIX
pub fn open_file(file_path: &str) -> Result<(), String> {
    // Open a file with the default program associated with its type
    let gio_file = File::new_for_path(file_path);
    let app_info = match AppInfo::create_from_commandline(
        "default-program-name",
        Some("Default Program"),
        AppInfoCreateFlags::NONE,
    ) {
        Ok(app_info) => app_info,
        Err(err) => return Err(format!("Error creating AppInfo: {}", err)),
    };

    let launch_context = GioAppLaunchContext::new();
    match app_info.launch(&[gio_file], Some(&launch_context)) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error launching file: {}", err)),
    }
}


