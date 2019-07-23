extern crate gio;
extern crate gtk;

mod components;
#[macro_use]
mod ui;

use gio::prelude::*;
use gtk::prelude::*;
#[allow(unused_imports)]
use gtk::{
    AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, CheckMenuItem, FileChooserAction,
    FileChooserDialog, IconSize, Image, Label, Layout, Menu, MenuBar, MenuItem, ResponseType,
    ScrolledWindowBuilder, Window, WindowPosition,
};

use std::env::args;
use ui::root_window::SchotteApp;

fn _build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("MenuBar example");
    window.set_position(WindowPosition::Center);
    window.set_size_request(400, 400);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let menu_bar = MenuBar::new();

    // The item itself that gets drawn for the menu
    let file_root_item = MenuItem::new_with_label("File");

    let file_open_item = MenuItem::new_with_label("Open...");
    let file_quit_item = MenuItem::new_with_label("Quit");

    // The menu which contains more items
    let file_menu = Menu::new();
    file_menu.append(&file_open_item);
    file_menu.append(&file_quit_item);

    file_root_item.set_submenu(Some(&file_menu));

    menu_bar.append(&file_root_item);

    let image_view = Image::new();

    let window_weak = window.downgrade();
    file_quit_item.connect_activate(move |_| {
        let window = upgrade_weak!(window_weak);
        window.destroy();
    });

    // `Primary` is `Ctrl` on Windows and Linux, and `command` on macOS
    // It isn't available directly through gdk::ModifierType, since it has
    // different values on different platforms.
    //let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    //file_quit_item.add_accelerator("activate", &accel_group, key, modifier, AccelFlags::VISIBLE);

    let label = Label::new(Some("Image Opener"));

    let scrolled_window = ScrolledWindowBuilder::new().build();
    scrolled_window.add(&image_view);

    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&scrolled_window, true, true, 0);
    v_box.pack_start(&label, false, false, 0);
    window.add(&v_box);

    file_open_item.connect_activate(move |_| {
        let dialog = FileChooserDialog::with_buttons::<Window>(
            Some("Open File"),
            None,
            FileChooserAction::Open,
            &[
                ("_Cancel", ResponseType::Cancel),
                ("_Open", ResponseType::Accept),
            ],
        );
        let res: ResponseType = dialog.run();
        dbg!(res);
        if let ResponseType::Accept = res {
            dbg!(dialog.get_filename());
            if let Some(p) = dialog.get_filename() {
                image_view.set_from_file(p);
            }
        }
        dialog.destroy();
    });

    window.show_all();

    //about.connect_activate(move |_| {
    //    let p = AboutDialog::new();
    //    p.set_authors(&["gtk-rs developers"]);
    //    p.set_website_label(Some("gtk-rs"));
    //    p.set_website(Some("http://gtk-rs.org"));
    //    p.set_authors(&["Gtk-rs developers"]);
    //    p.set_title("About!");
    //    p.set_transient_for(Some(&window));
    //    p.run();
    //    p.destroy();
    //});
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.menu_bar"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        SchotteApp::build(app);
    });
    application.run(&args().collect::<Vec<_>>());
}
