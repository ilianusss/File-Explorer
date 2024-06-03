//INTERFACE
use FileExplorer::interface::interface::*;
use std::{env, fs, path::Path, ffi::OsStr, cell::RefCell, time::{Duration, SystemTime, UNIX_EPOCH}};
use gio::prelude::*;
use gio::{SettingsExt, AppInfo, AppInfoCreateFlags, AppLaunchContext, FileExt, AppLaunchContext as GioAppLaunchContext};
use gtk::{prelude::*, print_run_page_setup_dialog_async};
use gtk::{Application, Box, Orientation, ScrolledWindow, ListStore, TreeViewColumn, CellRendererText, Entry, Button, Label, SettingsExt as OtherSettingsExt, Window, WindowType, CheckButton, ToggleButtonExt};
use glib::{MainContext, clone};
use chrono::{DateTime, Local};
use std::rc::Rc;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::process::Command;


//BASH COMMANDS
use FileExplorer::bash_commands::bash_commands::*;

//ALGORITHMS
use FileExplorer::algorithms::*;

fn main() {
    // UI
    let app = gtk::ApplicationBuilder::new()
        .application_id("s4.FileExplorer")
        .build();

    app.connect_activate(build_ui);
    app.run(&[]);
}


// build_ui
fn build_ui(app: &Application) {
        //copy_dir("/Users/ilianus/Desktop/EPITA/S4/PROJET/test/NOUVO", "/Users/ilianus/Desktop/EPITA/S4/PROJET/test/c");

// ALGORITHMS
    println!("Indexing all files");
    //let search_files = indexing::index_files_libc("/Users/ilianus/");
    let search_files = indexing::index_files_fs("/home/");

// UI
  // SETUP
    // Initialize dir
    let current_directory = Rc::new(RefCell::new(String::new()));
    if let Ok(current_dir) = env::current_dir() {
        *current_directory.borrow_mut() = current_dir.to_string_lossy().to_string();
    }

    // Initialize paste param
    let mut paste_directory = Rc::new(RefCell::new(String::new()));

    // Initialize show_hidden
    let show_hidden = Rc::new(RefCell::new(false));
    
    // Initialize search mode variable
    let search_mode = Rc::new(RefCell::new(false));
    let search_entries = Rc::new(RefCell::new(Vec::new()));

    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Dark theme
    let settings = gtk::Settings::get_default().expect("Failed to get GTK settings");
    settings.set_property("gtk-application-prefer-dark-theme", &true)
        .expect("Failed to set dark theme preference");

    // Create window
    let window = gtk::WindowBuilder::new()
        .application(app)
        .title("File Explorer")
        .default_width(500)
        .default_height(500)
        .build();

    // Create vbox
    let vbox = gtk::Box::new(Orientation::Vertical, 10);

    // Margins
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);
    vbox.set_margin_top(15);
    vbox.set_margin_bottom(10);

    window.add(&vbox);



  // HEADER
    let header_bar = gtk::HeaderBarBuilder::new().title("File Explorer").show_close_button(true).build();
    window.set_titlebar(Some(&header_bar));
    let current_directory_label = Label::new(Some(&*current_directory.borrow()));
    let time_label = Label::new(None);

    header_bar.pack_start(&time_label);
    header_bar.pack_start(&current_directory_label);
    vbox.pack_start(&header_bar, false, false, 0);

    // Adding a search button to the header
    let search_button = gtk::ToggleButton::with_label("Search");
    header_bar.pack_end(&search_button);

    // Creating a search bar
    let search_bar = gtk::Box::new(Orientation::Horizontal, 5);

    // Binding the search button's state to the search bar's search-mode-enabled property
    

    // Creating an entry inside the search bar
    let entry = gtk::SearchEntry::new();
    entry.set_hexpand(true);
    //search_bar.add(&entry);

    /* 
    // Connecting search-related signals
    entry.connect_search_started(clone!(@weak search_button => move |_| {
        search_button.set_active(true);
    }));

    entry.connect_stop_search(clone!(@weak search_button => move |_| {
        search_button.set_active(false);
    }));*/

    // Initialize time label
    update_time_label(&time_label);

    // Update time label every second
    glib::timeout_add_seconds_local(1, clone!(@strong time_label => move || {
        update_time_label(&time_label);
        glib::Continue(true)
    }));



  // OPTION BAR
    let show_hidden_check_button = CheckButton::with_label("Show hidden");
    let new_button = Button::with_label("NEW");
    let paste_button = Button::with_label("Paste");
    let search_ok_button = Button::with_label("Search");
    let search = gtk::Box::new(Orientation::Horizontal, 5);
    let search_entry = Entry::new();
    search.pack_start(&search_entry, true, true, 0);

    header_bar.pack_end(&show_hidden_check_button);
    header_bar.pack_end(&new_button);
    header_bar.pack_end(&paste_button);
    search_bar.pack_start(&search, true, true, 0);
    search_bar.pack_end(&search_ok_button, false, false, 0);
    
    // Create a revealer for smooth animation
    let search_revealer = Rc::new(RefCell::new(gtk::Revealer::new()));
    search_revealer.borrow().add(&search_bar);
    search_revealer.borrow().set_transition_type(gtk::RevealerTransitionType::SlideDown);
    search_revealer.borrow().set_transition_duration(500);
    search_revealer.borrow().set_reveal_child(false);
    vbox.pack_start(&search_revealer.borrow().clone().upcast::<gtk::Widget>(), false, false, 0);

    search_bar.set_hexpand(true);

    // Add the option bar to the vbox
    //vbox.pack_start(&option_bar, false, false, 0);


    // New directory bar
    let new_dir_bar = gtk::Box::new(Orientation::Horizontal, 5);
    let new_dir_enter_bar_entry = Entry::new();
    let new_dir_ok_button = Button::with_label("Enter");

    new_dir_bar.pack_start(&new_dir_enter_bar_entry, true, true, 0);
    new_dir_bar.pack_end(&new_dir_ok_button, false, true, 0);

    // Create a revealer for smooth animation
    let revealer = Rc::new(RefCell::new(gtk::Revealer::new()));
    revealer.borrow().add(&new_dir_bar);
    revealer.borrow().set_transition_type(gtk::RevealerTransitionType::SlideDown);
    revealer.borrow().set_transition_duration(500);
    revealer.borrow().set_reveal_child(false);
    vbox.pack_start(&revealer.borrow().clone().upcast::<gtk::Widget>(), false, false, 0);

    new_dir_bar.set_hexpand(true);



  // MAIN PART
    // Scrolled window
    let scrolled_window = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled_window.set_min_content_width(900);
    scrolled_window.set_min_content_height(900);

    // List store
    //let list_store = gtk::ListStore::new(&[glib::Type::String, glib::Type::String, glib::Type::String, glib::Type::String]);
    let list_store = gtk::ListStore::new(&[glib::Type::String, glib::Type::String, glib::Type::String, glib::Type::String]);
    let dir_path = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    populate_list_store(&list_store, &dir_path, false);

    // Tree view
    let tree_view = gtk::TreeView::new();
    tree_view.set_model(Some(&list_store));
    tree_view.set_activate_on_single_click(true);

    add_column(&tree_view, "File Name", 0);
    add_column(&tree_view, "File Type", 1);
    add_column(&tree_view, "File Size", 2);
    add_column(&tree_view, "Last Modified", 3);

    scrolled_window.add(&tree_view);

    // Add scrolled window to the vbox
    vbox.add(&scrolled_window);





  // CONNECT UI TO ACTIONS
    // about us button
    let add_button = gtk::Button::with_label("More About Us");
        add_button.set_margin_top(5);
        add_button.set_margin_bottom(5);
        vbox.add(&add_button);

        add_button.connect_clicked(move |add_button| {
                let button = gtk::Button::with_label("About Us");

                // Create window
                let window2 = gtk::WindowBuilder::new()
                    .title("File Explorer")
                    .default_width(500)
                    .default_height(500)
                    .build();


                button.connect_clicked(glib::clone!(@weak window2 => move |_| {
                    let dialog = gtk::AboutDialogBuilder::new()
                        .transient_for(&window2)
                        .modal(true)
                        .program_name("File Explorer")
                        .version("0.2.0")
                        .website_label("GTK-RS Website")
                        .website("https://gtk-rs.org/")
                        .authors(vec![
                            "Tristan Faure".to_string(),
                            "Iliane Formet".to_string(),
                            "Arthur Garraud".to_string(),
                        ])
                        .visible(true)
                        .build();
        
                    dialog.present();
                }));

                window2.present();
        });
        // reveal search button
        let search_revealer_clone1 = Rc::clone(&search_revealer);

        search_button.connect_clicked(move |_| {
            let is_revealed = search_revealer_clone1.borrow().get_reveal_child();
            search_revealer_clone1.borrow_mut().set_reveal_child(!is_revealed);
        });
       // New directory button
        let new_dir_directory_clone = Rc::clone(&current_directory);
        let revealer_clone1 = Rc::clone(&revealer);
    let revealer_clone2 = Rc::clone(&revealer);
    let list_store_clone = list_store.clone();

    new_button.connect_clicked(move |_| {
        let is_revealed = revealer_clone1.borrow().get_reveal_child();
        revealer_clone1.borrow_mut().set_reveal_child(!is_revealed);
    });

    let new_directory_show_hidden_clone = show_hidden.clone();
    new_dir_ok_button.connect_clicked(move |_| {
        // Create new dir
        let new_dir_name = new_dir_enter_bar_entry.get_text().to_string();
        create_dir(&*new_dir_directory_clone.borrow(), &new_dir_name);

        // Close new dir bar + clean written text
        revealer_clone2.borrow_mut().set_reveal_child(false);
        new_dir_enter_bar_entry.set_text("");


        list_store_clone.clear();
        let updated_dir = format!("{}", *new_dir_directory_clone.borrow());
        let show_hidden_value = *new_directory_show_hidden_clone.borrow();
        populate_list_store(&list_store_clone, &updated_dir, show_hidden_value);
    });


   // Paste button
    let mut paste_path_clone = Rc::clone(&paste_directory);
    let paste_button_list_store_clone = Rc::new(RefCell::new(list_store.clone()));
    let paste_button_actual_dir_clone = Rc::clone(&current_directory);
    let paste_button_show_hidden_clone = show_hidden.clone();

    paste_button.connect_clicked(move |_| {
        let str_paste_path_clone = paste_path_clone.borrow().clone();
        if !str_paste_path_clone.is_empty() {
            let mut splitted: Vec<_> = str_paste_path_clone.split('/').collect();
            let paste_type = splitted.pop().unwrap_or_default().to_string();
            let item_name = splitted.pop().unwrap_or_default().to_string();
            let mut res = splitted.join("/");
            let src = format!("{}/{}", res, item_name);
            let dst = format!("{}/{}", paste_button_actual_dir_clone.borrow().clone(), item_name);

            let metadata_src_clone = src.clone();

            match paste_type.as_str() {
                "COPY" => {
                    if let Ok(metadata) = fs::metadata(&metadata_src_clone) {
                        if metadata.is_dir() {
                            copy_dir(&src, &paste_button_actual_dir_clone.borrow().clone());
                        } else if metadata.is_file() {
                            copy_file(&src, &dst);
                        } else {
                            println!("It's neither a file nor a directory!");
                        }
                    }
                }
                "CUT" => {
                    if let Ok(metadata) = fs::metadata(&metadata_src_clone) {
                        if metadata.is_dir() {
                            copy_dir(&src, &paste_button_actual_dir_clone.borrow().clone());
                            remove_dir(&src);
                        } else if metadata.is_file() {
                            copy_file(&src, &dst);
                            remove_dir(&src);
                        } else {
                            println!("It's neither a file nor a directory!");
                        }
                    }
                }
                _ => println!("error")
            }

            paste_button_list_store_clone.borrow_mut().clear();
            let list_store_ref = paste_button_list_store_clone.borrow();
            let show_hidden_value = *paste_button_show_hidden_clone.borrow();
            populate_list_store(&list_store_ref, &paste_button_actual_dir_clone.borrow(), show_hidden_value);
        }
    });

    // SHOW HIDDEN CHECK BUTTON
    let show_hidden_current_directory_clone = Rc::clone(&current_directory);
    let show_hidden_list_store_clone = Rc::new(RefCell::new(list_store.clone()));
    let show_hidden_show_hidden_clone = show_hidden.clone();

    show_hidden_check_button.connect_toggled(move |btn| {
        let show_hidden_show_hidden_clone = btn.get_active();
        let list_store_ref = show_hidden_list_store_clone.borrow();
        let dir_path = show_hidden_current_directory_clone.borrow();
        populate_list_store(&list_store_ref, &show_hidden_current_directory_clone.borrow(), show_hidden_show_hidden_clone);
    });


    // Search button
    
    // Clone async variables
    let search_entries_clone = Rc::clone(&search_entries);
    let search_mode_clone = Rc::clone(&search_mode);
    let list_store_clone = Rc::new(RefCell::new(list_store.clone()));
    let cd_directory_clone = Rc::clone(&current_directory);
    let search_button_show_hidden = show_hidden.clone();

    search_ok_button.connect_clicked(move |_| {
        let text = search_entry.get_text();
        if !text.is_empty() {

            *search_mode_clone.borrow_mut() = true;
            (*search_entries_clone.borrow_mut()).clear();

            let text = text.to_string();
            let search_results = search_prefix::search_filename(&text, &search_files);
            let list_store_ref = list_store_clone.borrow_mut();
            list_store_ref.clear();

            // Iter in all results and get their DirEntry -> add them to search_entries
            for path_string in search_results.iter() {
                let path = Path::new(&path_string);
                if let Some(parent) = path.parent() {
                    if let Some(parent_str) = parent.to_str() {
                        if let Ok(iters) = fs::read_dir(parent_str) {
                            for iter in iters {
                                match iter {
                                    Ok(entry) => if let Ok(filename) = entry.file_name().into_string() {
                                        //if filename.contains(&text) && !(*search_entries_clone.borrow()).contains(entry) {
                                        if filename == path.file_name().unwrap_or_default().to_string_lossy().into_owned() {
                                            (*search_entries_clone.borrow_mut()).push(entry);
                                        }
                                    }
                                    Err(_) => ()
                                }
                            }
                        }
                    }
                }
            }

            // Sort algo for files's importance
            (*search_entries_clone.borrow_mut()).sort_by_key(|dir_entry| dir_entry.path().to_string_lossy().matches('/').count());

            // Refresh list store using search's results
            for entry in (*search_entries_clone.borrow_mut()).iter() {
                if let Some(file_name) = entry.file_name().to_str() {
                let metadata = fs::metadata(entry.path()).ok();
                let file_type = get_file_type(&entry);
                let file_size = format_file_size(metadata.as_ref().map(|meta| meta.len()));
                let last_modified = metadata
                    .and_then(|meta| meta.modified().ok())
                    .map(|modified| modified.duration_since(UNIX_EPOCH).ok())
                    .and_then(|modified_time| Some(format_last_modified(modified_time)));

                // Insert values into the list store
                list_store_ref.insert_with_values(
                    None,
                    &[0, 1, 2, 3],
                    &[&file_name, &file_type, &file_size, &last_modified.unwrap_or_default()],
                );
                }
            }
        }
        else {

            // Exit search mode and go to /home
            *search_mode_clone.borrow_mut() = false;
            (*search_entries_clone.borrow_mut()).clear();
            *cd_directory_clone.borrow_mut() = String::from("/home");
            let show_hidden_value = *search_button_show_hidden.borrow();
            populate_list_store(&*list_store_clone.borrow_mut(), &*cd_directory_clone.borrow_mut(), show_hidden_value);
        }

    });


   // CD
    let cd_directory_clone = Rc::clone(&current_directory);
    let search_entries_clone = Rc::clone(&search_entries);
    let search_mode_clone = Rc::clone(&search_mode);
    let cd_list_store_clone = list_store.clone();
    let cd_show_hidden_clone = show_hidden.clone();

    tree_view.connect_row_activated(move |_tree_view, path, _column| {
        if let Some(iter) = cd_list_store_clone.get_iter(path) {
            let file_name = cd_list_store_clone.get_value(&iter, 0).get::<String>().unwrap_or_default();
            let file_type = cd_list_store_clone.get_value(&iter, 1).get::<String>().unwrap_or_default();

            if file_type.as_deref() == Some("Directory") {
                // Change directory and update list store
                
                // If list_store contains search's results
                if *search_mode_clone.borrow() == true {
                    if let Some(index) = path.get_indices().get(0) {
                        let absolute_path = search_entries_clone.borrow()[*index as usize].path().to_string_lossy().into_owned();
                        // Set courant path to absolute path of search result clicked
                        current_directory_label.set_text(&absolute_path);
                        *cd_directory_clone.borrow_mut() = absolute_path.clone();
                        *search_mode_clone.borrow_mut() = false;
                    }
                }
                else {
                    // If .. go to parent folder, else go to folder clicked
                    let dir_name = file_name.unwrap_or_default();
                    let selected_dir = if dir_name != ".." {
                        format!("{}/{}", *cd_directory_clone.borrow(), dir_name)}
                    else {
                        if let Some(parent) = Path::new(&*cd_directory_clone.borrow()).parent() {
                            parent.to_string_lossy().into_owned()
                        }
                        else {String::from("")}
                    };
                    current_directory_label.set_text(&selected_dir);
                    *cd_directory_clone.borrow_mut() = selected_dir.clone();
                }

                cd_list_store_clone.clear();
                let show_hidden_value = *cd_show_hidden_clone.borrow();
                populate_list_store(&cd_list_store_clone, &*cd_directory_clone.borrow_mut(), show_hidden_value);
            } else {
                let name = file_name.unwrap_or_default();
                let extension = name.split('.').last().unwrap_or_default();
                match extension {
                    "txt"|"pdf"|"toml"|"doc"|"docx"|"xls"|"xlsx"|"lock"|"rs" => {
                        let content = read_file_to_string(&format!("{}/{}", *cd_directory_clone.borrow(), name)).unwrap_or_default();
                        let window_file = gtk::Window::new(gtk::WindowType::Toplevel);
                        window_file.set_title(&name);
                        window_file.set_default_size(500, 500);

                        let vbox_file = gtk::Box::new(Orientation::Vertical, 10);

                        vbox_file.set_margin_start(10);
                        vbox_file.set_margin_end(10);
                        vbox_file.set_margin_top(15);
                        vbox_file.set_margin_bottom(10);

                        window_file.add(&vbox_file);

                        let label = Label::new(Some(&content));
                        vbox_file.add(&label);
                        println!("File opened: {}", name);
                        println!("Content: {}", content);

                        window_file.present();
                    },
                    "png"|"jpg"|"jpeg"|"gif"|"bmp"|"tiff"|"svg" => {
                        let window_image = gtk::Window::new(gtk::WindowType::Toplevel);
                        window_image.set_title(&name);
                        window_image.set_default_size(500, 500);

                        let vbox_image = gtk::Box::new(Orientation::Vertical, 10);

                        vbox_image.set_margin_start(10);
                        vbox_image.set_margin_end(10);
                        vbox_image.set_margin_top(15);
                        vbox_image.set_margin_bottom(10);

                        window_image.add(&vbox_image);

                        let image = gtk::Image::from_file(&format!("{}/{}", *cd_directory_clone.borrow(), name));
                        vbox_image.add(&image);
                        println!("Image opened: {}", name);

                        window_image.present();
                    },
                    "mp4" => {
                        let window_video = gtk::Window::new(gtk::WindowType::Toplevel);
                        window_video.set_title(&name);
                        window_video.set_default_size(500, 500);

                        let vbox_video = gtk::Box::new(Orientation::Vertical, 10);

                        vbox_video.set_margin_start(10);
                        vbox_video.set_margin_end(10);
                        vbox_video.set_margin_top(15);
                        vbox_video.set_margin_bottom(10);

                        window_video.add(&vbox_video);

                        // TODO: open the video
                        println!("Video opened: {}", name);

                        window_video.present();
                    },
                    _ => (),
                }
            }
        }
    });


   // MENU
    let menu_tree_view_clone = tree_view.clone();
    let menu_list_store_clone = list_store.clone();
    let connect_menu_list_store_clone = list_store.clone();
    let menu_directory_clone = Rc::clone(&current_directory);
    let menu_show_hidden_clone = show_hidden.clone();

    tree_view.connect_button_press_event(move |_, event| {
        let menu_list_store_clone1 = menu_list_store_clone.clone(); // Clone list_store
        let connect_menu_list_store_clone1 = connect_menu_list_store_clone.clone();

        if event.get_button() == 3 { // Right mouse button
            if let Some((path, _, _, _)) = menu_tree_view_clone.get_path_at_pos(event.get_position().0 as i32, event.get_position().1 as i32) {
                // Check if path is Some
                if let Some(path) = path {
                    let menu = gtk::Menu::new();

                    // Create menu items for common actions
                    let copy_item = gtk::MenuItem::with_label("Copy");
                    let cut_item = gtk::MenuItem::with_label("Cut");
                    let delete_item = gtk::MenuItem::with_label("Delete");
                    let duplicate_item = gtk::MenuItem::with_label("Duplicate");
                    let compress_item = gtk::MenuItem::with_label("Compress");
                    let decompress_item = gtk::MenuItem::with_label("Decompress");
                    let open_terminal_item = gtk::MenuItem::with_label("Open terminal");

                    // Append menu items to the menu
                    menu.append(&copy_item);
                    menu.append(&cut_item);
                    menu.append(&delete_item);
                    //menu.append(&duplicate_item);
                    menu.append(&compress_item);
                    menu.append(&decompress_item);
                    menu.append(&open_terminal_item);

                    let mut elem_path = String::new();

                    if let Some(iter) = menu_list_store_clone1.get_iter(&path) {
                        let file_name = menu_list_store_clone1.get_value(&iter, 0).get::<String>().unwrap_or_default();
                        elem_path = format!("{}/{}", *menu_directory_clone.borrow(), file_name.unwrap_or_default());
                    }

                    connect_menu_item_signals(&delete_item, elem_path.clone(), &connect_menu_list_store_clone1, menu_show_hidden_clone.clone());
                    //connect_menu_item_signals(&duplicate_item, elem_path.clone(), &connect_menu_list_store_clone1);
                    connect_menu_item_signals(&compress_item, elem_path.clone(), &connect_menu_list_store_clone1, menu_show_hidden_clone.clone());
                    connect_menu_item_signals(&decompress_item, elem_path.clone(), &connect_menu_list_store_clone1, menu_show_hidden_clone.clone());
                    connect_menu_item_signals(&open_terminal_item, elem_path.clone(), &connect_menu_list_store_clone1, menu_show_hidden_clone.clone());

                    let mut copy_paste_path_clone = Rc::clone(&paste_directory);
                    let copy_act_path_clone = elem_path.clone();
                    copy_item.connect_activate(move |_| {
                        *copy_paste_path_clone.borrow_mut() = format!("{}/COPY",&copy_act_path_clone);
                    });

                    let mut cut_paste_path_clone = Rc::clone(&paste_directory);
                    let cut_act_path_clone = elem_path.clone();
                    cut_item.connect_activate(move |_| {
                        *cut_paste_path_clone.borrow_mut() = format!("{}/CUT",&cut_act_path_clone);
                    });

                    // Get the mouse position
                    let (x, y) = event.get_position();

                    // Convert mouse coordinates to screen coordinates
                    let (screen_x, screen_y) = event.get_root();

                    // Popup the menu at the specified position
                    menu.popup::<gtk::Widget, gtk::Widget, _>(None, None, move |_, x: &mut i32, y: &mut i32| {
                        *x = screen_x as i32;
                        *y = screen_y as i32;
                        true
                    }, screen_x as u32, screen_y as u32);

                    // Show the menu
                    menu.show_all();
                }
            }
        }
        Inhibit(false)
    });





  // END OF EXECUTION
    // Window destroy event
    window.connect_destroy(|_| {
        gtk::main_quit();
    });

    window.show_all();

    // Start GTK main loop
    gtk::main();
    }

// FUNCTIONS
  // MENUS
    // Connect menu items to actions
    fn connect_menu_item_signals(menu_item: &gtk::MenuItem, path: String, list_store: &gtk::ListStore, show_hidden: Rc<RefCell<bool>>) {
        // Clone the menu item for use in the closure
        let menu_item_clone = menu_item.clone();
        let path_clone = path.clone();
        let update_list_store_path = path.clone();
        let list_store_clone = list_store.clone();
        let connect_show_hidden_clone = show_hidden.clone();

        // Connect the 'activate' signal to the closure
        menu_item.connect_activate(move |_| {
            // Check which menu item was activated
            match menu_item_clone.get_label().unwrap().as_str() {
                "Delete" => {
                    if let Ok(metadata) = fs::metadata(&path_clone) {
                        if metadata.is_dir() {
                            remove_dir(&path_clone);
                        } else if metadata.is_file() {
                            remove_file(&path_clone);
                        } else {
                            println!("It's neither a file nor a directory!");
                        }
                    }

                    list_store_clone.clear();
                    let mut components: Vec<_> = update_list_store_path.split('/').collect();
                    components.pop();
                    let new_path = components.join("/");
                    let show_hidden_ref = *connect_show_hidden_clone.borrow();
                    populate_list_store(&list_store_clone, &new_path, show_hidden_ref);
                }
                /*"Duplicate" => {
                    let duplicate_path_clone = path.clone();
                    let dst = format!("{}(1)",duplicate_path_clone);

                    if let Ok(metadata) = fs::metadata(&path_clone) {
                        if metadata.is_dir() {
                            ();
                        } else if metadata.is_file() {
                            copy_file(&path_clone, &dst);
                        } else {
                            println!("It's neither a file nor a directory!");
                        }
                    }

                    list_store_clone.clear();
                    let mut components: Vec<_> = update_list_store_path.split('/').collect();
                    components.pop();
                    let new_path = components.join("/");
                    populate_list_store(&list_store_clone, &new_path);
                }*/
                "Compress" => {
                    let mut output_file = path.clone();
                    if let Ok(metadata) = fs::metadata(output_file.as_str()) {
                        if metadata.is_dir() {
                            output_file.push_str(".zip");
                            match compression::compress_folder(path.as_str(), output_file.as_str()) {
                                std::result::Result::Ok(_) => println!("Compress successful"),
                                std::result::Result::Err(_) => println!("Error while compress")
                            }
                        }
                        else {
                            println!("Not a folder"); 
                        }
                    }
                    else {println!("File not found");}

                    list_store_clone.clear();
                    let mut components: Vec<_> = update_list_store_path.split('/').collect();
                    components.pop();
                    let new_path = components.join("/");
                    let show_hidden_ref = *connect_show_hidden_clone.borrow();
                    populate_list_store(&list_store_clone, &new_path, show_hidden_ref);

                }
                "Decompress" => {
                    let output_file = path.clone();
                    if output_file.ends_with(".zip") {
                        let output_file = &output_file[..output_file.len()-4];
                        match compression::uncompress_folder(path.as_str(), output_file) {
                            std::result::Result::Ok(_) => println!("Uncompress successful"),
                            std::result::Result::Err(_) => println!("Error while uncompress")
                        }
                    }
                    else {
                        println!("Not a .zip file");
                    }

                    list_store_clone.clear();
                    let mut components: Vec<_> = update_list_store_path.split('/').collect();
                    components.pop();
                    let new_path = components.join("/");
                    let show_hidden_ref = *connect_show_hidden_clone.borrow();
                    populate_list_store(&list_store_clone, &new_path, show_hidden_ref);
                }
                "Open terminal" => {
                    if let Ok(metadata) = fs::metadata(&path_clone) {
                        if metadata.is_dir() {
                            let term_dir = path.clone();
                            let result = Command::new("open")
                                .arg("-a")
                                .arg("Terminal")
                                .arg(term_dir)
                                .status();

                            match result {
                                Ok(status) if status.success() => println!("Terminal launched successfully in /Users directory"),
                                Ok(status) => eprintln!("Terminal exited with status: {}", status),
                                Err(err) => eprintln!("Failed to launch terminal: {}", err),
                            }
                        }
                    }
                }
                _ => {
                    // Implement other actions if needed
                    println!("Menu item '{}' activated", menu_item_clone.get_label().unwrap());
                }
            }
        });
    }
    
    fn read_file_to_string(filename: &str) -> io::Result<String> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
    

