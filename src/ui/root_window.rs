use gtk::prelude::*;
#[allow(unused_imports)]
use gtk::{
    AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, CheckMenuItem, Container,
    FileChooserAction, FileChooserDialog, IconSize, Image, Label, Layout, Menu, MenuBar, MenuItem,
    ResponseType, ScrolledWindowBuilder, Window, WindowPosition, WindowType,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::components::document_system::DocumentSystem;
use crate::components::menu_manager::MenuManager;
use crate::upgrade_weak;

pub struct SchotteAppModel {
    menu_manager: Rc<RefCell<MenuManager>>,
    document_system: DocumentSystem,
}

impl SchotteAppModel {
    pub fn get_menu_manager(&mut self) -> Rc<RefCell<MenuManager>> {
        self.menu_manager.clone()
    }
}

pub struct SchotteApp {
    model: Rc<RefCell<SchotteAppModel>>,
    app_window: gtk::ApplicationWindow,
}

impl SchotteApp {
    pub fn build(application: &gtk::Application) {
        let app_window = gtk::ApplicationWindow::new(application);

        app_window.set_title("ScotSchotte");
        app_window.set_position(WindowPosition::Center);
        app_window.set_size_request(400, 400);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let menu_manager = MenuManager::new();
        let document_system = DocumentSystem::new();
        let menubar = menu_manager.get_menubar();
        vbox.pack_start(&menubar, false, false, 0);

        let model = SchotteAppModel {
            menu_manager: Rc::new(RefCell::new(menu_manager)),
            document_system,
        };

        let result = SchotteApp {
            model: Rc::new(RefCell::new(model)),
            app_window: app_window.clone(),
        };

        {
            let mut mut_model = result.model.borrow_mut();
            let menu_manager_rc = mut_model.get_menu_manager();
            let mut menu_manager = menu_manager_rc.borrow_mut();
            let document_ui = mut_model.document_system.build_ui(&mut menu_manager);
            vbox.pack_start(&document_ui, true, true, 0);
        }
        // Keep this after all the other components so that File/Quit stays
        // at the bottom where it belongs. Nice and cozy down there.
        {
            let model = result.model.borrow();
            let quit_item = model
                .menu_manager
                .borrow_mut()
                .add_menu_item("File/Quit")
                .expect("Error creating quit menu item");
            let weak_window = app_window.downgrade();
            quit_item.connect_activate(move |_| {
                let window = upgrade_weak!(weak_window);
                window.destroy();
            });
        }

        //let scrolled_window = ScrolledWindowBuilder::new().build();
        let label = Label::new(Some("Image Opener"));

        //vbox.pack_start(&scrolled_window, true, true, 0);
        vbox.pack_start(&label, false, false, 0);

        result.app_window.add(&vbox);
        result.app_window.show_all();
    }
}
