use relm::{Relm, Update, Widget, init, Component};
use relm_derive::{Msg,};
use gtk::prelude::*;
use gtk::{
    Container, AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, CheckMenuItem, FileChooserAction,
    FileChooserDialog, IconSize, Image, Label, Layout, Menu, MenuBar, MenuItem, ResponseType,
    ScrolledWindowBuilder, Window, WindowPosition, WindowType,
};

use super::file_menu::FileMenu;

pub struct SchotteAppModel {

}

pub struct SchotteRootWindow {
    model: SchotteAppModel,
    window: Window,
    menubar: MenuBar,
    file_menu: Component<FileMenu>,
}

#[derive(Msg)]
pub enum RootMsg {
    Quit,
}

impl Update for SchotteRootWindow {
    type Model = SchotteAppModel;
    type ModelParam = ();
    type Msg = RootMsg;

    fn model(_: &Relm<Self>, _: ()) -> Self::Model {
        Self::Model{}
    }

    fn update(&mut self, event: RootMsg) {
        match event {
            RootMsg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for SchotteRootWindow {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: SchotteAppModel) -> Self {
        let window = Window::new(WindowType::Toplevel);

        window.set_title("ScotSchotte");
        window.set_position(WindowPosition::Center);
        window.set_size_request(400, 400);

        connect!(relm, window, connect_delete_event(_, _), return (Some(RootMsg::Quit), Inhibit(false)));

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let menubar = MenuBar::new();

        let file_menu = init::<FileMenu>((menubar.clone(), relm.clone())).expect("Failed to initialize file menu");

        let result = SchotteRootWindow {
            model,
            window,
            menubar,
            file_menu,
        };

        let scrolled_window = ScrolledWindowBuilder::new().build();
        let label = Label::new(Some("Image Opener"));

        vbox.pack_start(&result.menubar, false, false, 0);
        vbox.pack_start(&scrolled_window, true, true, 0);
        vbox.pack_start(&label, false, false, 0);

        result.window.add(&vbox);
        result.window.show_all();
        result
    }
}
