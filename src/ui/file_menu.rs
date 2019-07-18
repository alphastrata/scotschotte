use relm::{Relm, Update, Widget};
use relm_derive::{Msg,};
use gtk::prelude::*;
use gtk::{
    AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, CheckMenuItem, FileChooserAction,
    FileChooserDialog, IconSize, Image, Label, Layout, Menu, MenuBar, MenuItem, ResponseType,
    ScrolledWindowBuilder, Window, WindowPosition, Object,
};

use super::root_window::{RootMsg, SchotteRootWindow};

pub struct FileMenuModel {
    container: MenuBar,
    parent_stream: Relm<SchotteRootWindow>,
}

pub struct FileMenu {
    model: FileMenuModel,
    menu: Menu,
    root_item: MenuItem,
    open_item: MenuItem,
    quit_item: MenuItem,
}

#[derive(Msg)]
pub enum FileMenuMsg {
    Open,
}

impl Update for FileMenu {
    type Model = FileMenuModel;
    type ModelParam = (MenuBar, Relm<SchotteRootWindow>);
    type Msg = FileMenuMsg;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {
        FileMenuModel {
            container: param.0,
            parent_stream: param.1,
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            FileMenuMsg::Open => println!("Tried to open!"),
        }
    }
}

impl Widget for FileMenu {
    type Root = MenuItem;

    fn root(&self) -> Self::Root {
        self.root_item.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        println!("Loading file menu");
        let menu = Menu::new();
        let root_item = MenuItem::new_with_label("File");
        let open_item = MenuItem::new_with_label("Open...");
        let quit_item = MenuItem::new_with_label("Quit");


        connect!(relm, open_item, connect_activate(_), FileMenuMsg::Open);
        connect!(model.parent_stream, quit_item, connect_activate(_), RootMsg::Quit);

        root_item.set_submenu(Some(&menu));
        menu.append(&open_item);
        menu.append(&quit_item);

        model.container.append(&root_item);

        FileMenu {
            model,
            menu,
            root_item,
            open_item,
            quit_item,
        }
    }
}