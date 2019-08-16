extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;

mod components;
#[macro_use]
mod ui_util;

#[allow(unused_imports)]
use gio::prelude::*;
use gtk::prelude::*;
#[allow(unused_imports)]
use gtk::{
    AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, CheckMenuItem, EventBox,
    FileChooserAction, FileChooserDialog, IconSize, Image, Label, Layout, Menu, MenuBar, MenuItem,
    ResponseType, ScrolledWindowBuilder, WidgetExt, Window, WindowPosition,
};

use components::schotte_core::SchotteApp;
use std::env::args;

fn main() {
    println!("I. LiVe. AgAiN.");

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
