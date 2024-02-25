//INTERFACE
use FileExplorer::interface::interface::*;
use gtk::prelude::*;
use gtk ::{Application, ApplicationWindow, Button, Label, Box, Orientation};
use rand::random;

//BASH COMMANDS
use FileExplorer::bash_commands::bash_commands::*;

//ALGORITHMS


fn main() {
    // UI
    let app = Application::builder()
        .application_id("s4.FileExplorer")
        .build();

    app.connect_activate(build_ui);
    app.run();
}



