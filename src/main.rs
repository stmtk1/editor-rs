use gtk::prelude::*;
use gio::prelude::*;
use gtk::{ ScrolledWindow, TextView, Adjustment };
use std::env;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK");
        return;
    }

    let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application::new failed");

    uiapp.connect_activate(|app| {
        new_window(app).show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}

fn editor_new() -> ScrolledWindow {
    let scr_win = ScrolledWindow::new::<Adjustment, Adjustment>(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let txt_view = TextView::new();
    scr_win.add(&txt_view);
    scr_win
}

fn new_window<T: IsA<gtk::Application>>(app: &T) -> gtk::ApplicationWindow {
    let win = gtk::ApplicationWindow::new(app);
    win.set_default_size(320, 320);
    win.set_title("Basic example");
    let scr_win = editor_new();
    win.add(&scr_win);
    win
}
