use gtk::prelude::*;
use gtk ::{Application, ApplicationWindow, Button, Label, Box, Orientation};
use rand::random;

pub fn build_ui(app: &Application) {
    let label = Label::builder()
        .label("Appuie pour commencer")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button = Button::builder()
        .label("Pile ou Face")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&label);
    content.append(&button);

    let window = ApplicationWindow::builder()
        .title("File Explorer")
        .application(app)
        .child(&content)
        .build();

    button.connect_clicked(move |_| test(&label));
    window.show();
}

fn test(label: &Label) {
    if random() {
        label.set_text("Face");
    } else {
        label.set_text("Pile");
    }
}