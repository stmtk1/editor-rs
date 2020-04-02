use gtk::prelude::*;
use gio::prelude::*;
use gtk::{ ScrolledWindow, TextView, Adjustment, MenuItem, MenuBar, Menu };
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
    win.add(&new_vbox());
    win
}

fn new_menubar() -> MenuBar {
    let menu = MenuBar::new();
    menu.append(&new_file_menu());
    menu
}

fn new_vbox() -> gtk::Box {
    let ret = gtk::Box::new(gtk::Orientation::Vertical, 10);
    ret.pack_start(&new_menubar(), false, false, 0);
    ret.pack_start(&editor_new(), false, true, 0);
    ret
}

fn new_file_menu() -> MenuItem {
    let menu_item = MenuItem::new_with_label("File");
    menu_item
}
