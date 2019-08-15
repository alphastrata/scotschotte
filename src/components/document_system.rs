use gdk::EventMotion;
use gtk::prelude::*;
use gtk::{
    EventBox, FileChooserAction, FileChooserDialog, Image, ResponseType, ScrolledWindowBuilder,
    Window,
};

use std::cell::RefCell;
use std::rc::Rc;

use super::menu_manager::MenuManager;

trait Document {}

// Storage for information relating to an image document to be viewed/edited
struct ImageDocument {}

struct DocumentSystemModel {
    document: Option<ImageDocument>,
}

impl DocumentSystemModel {
    fn new() -> Self {
        DocumentSystemModel { document: None }
    }
}

struct MousePosition {
    x: f64,
    y: f64,
}

impl MousePosition {
    fn default() -> Self {
        MousePosition { x: 0.0, y: 0.0 }
    }

    fn from_tuple(arg: (f64, f64)) -> Self {
        MousePosition { x: arg.0, y: arg.1 }
    }
}

struct MouseNavigator {
    left_button: Option<MousePosition>,
    middle_button: Option<MousePosition>,
    right_button: Option<MousePosition>,
    previous_position: MousePosition,
}

pub struct DocumentSystem {
    model: Rc<RefCell<DocumentSystemModel>>,
    mouse: Rc<RefCell<MouseNavigator>>,
}

impl DocumentSystem {
    pub fn new() -> Self {
        DocumentSystem {
            model: Rc::new(RefCell::new(DocumentSystemModel::new())),
            mouse: Rc::new(RefCell::new(MouseNavigator {
                left_button: None,
                middle_button: None,
                right_button: None,
                previous_position: MousePosition::default(),
            })),
        }
    }

    pub fn build_ui(&mut self, menu_manager: &mut MenuManager) -> gtk::Widget {
        let scrolled_window = ScrolledWindowBuilder::new().build();
        let image_view = Image::new();
        let event_box = EventBox::new();
        event_box.add(&image_view);
        scrolled_window.add(&event_box);

        {
            let mousecopy = self.mouse.clone();
            event_box.connect_motion_notify_event(move |_, motion| {
                let mut mouse_state = mousecopy.borrow_mut();
                if mouse_state.middle_button.is_some() {
                    let prev = &mut mouse_state.previous_position;
                    let current = motion.get_root();

                    println!("{:?}, {:?}", current.0 - prev.x, current.1 - prev.y);

                    *prev = MousePosition::from_tuple(current);
                }
                Inhibit(false)
            });
        }

        {
            let mousecopy = self.mouse.clone();
            event_box.connect_button_press_event(move |_, press| {
                let pressed = press.get_button();
                let press_pos = press.get_root();

                let mut mouse_state = mousecopy.borrow_mut();
                match pressed {
                    1 => {
                        mouse_state.left_button = Some(MousePosition::from_tuple(press_pos));
                    }
                    2 => {
                        mouse_state.middle_button = Some(MousePosition::from_tuple(press_pos));
                    }
                    3 => {
                        mouse_state.right_button = Some(MousePosition::from_tuple(press_pos));
                    }
                    _ => (),
                }
                mouse_state.previous_position = MousePosition::from_tuple(press_pos);

                Inhibit(false)
            });
        }

        {
            let mousecopy = self.mouse.clone();
            event_box.connect_button_release_event(move |_, press| {
                let pressed = press.get_button();

                match pressed {
                    1 => mousecopy.borrow_mut().left_button = None,
                    2 => mousecopy.borrow_mut().middle_button = None,
                    3 => mousecopy.borrow_mut().right_button = None,
                    _ => (),
                }
                Inhibit(false)
            });
        }

        let open_menu_item = menu_manager
            .add_menu_item("File/Open")
            .expect("Couldn't create an Open file menu item");
        open_menu_item.connect_activate(move |_| {
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

        scrolled_window.upcast::<gtk::Widget>()
    }
}
