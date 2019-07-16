use relm::{Relm, Update, Widget};
use relm_derive::{Msg,};
use gtk::prelude::*;
use gtk::{
    Container, AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, CheckMenuItem, FileChooserAction,
    FileChooserDialog, IconSize, Image, Label, Layout, Menu, MenuBar, MenuItem, ResponseType,
    ScrolledWindowBuilder, Window, WindowPosition, WindowType,
};

pub struct SchotteAppModel {

}

pub struct SchotteRootWindow {
    model: SchotteAppModel,
    window: Window,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

impl Update for SchotteRootWindow {
    type Model = SchotteAppModel;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Self::Model {
        Self::Model{}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
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

        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        window.show_all();

        SchotteRootWindow {
            model,
            window,
        }
    }
}