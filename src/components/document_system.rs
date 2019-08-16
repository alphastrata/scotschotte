use gdk_pixbuf::Pixbuf;
use glib::object::WeakRef;
use gtk::prelude::*;
use gtk::{
    FileChooserAction, FileChooserDialog, Image, ResponseType, ScrolledWindowBuilder, Window,
};
use std::cell::RefCell;
use std::rc::Rc;

use super::menu_manager::MenuManager;

trait Document {}

// Storage for information relating to an image document to be viewed/edited
struct ImageDocument {
    buffer: Pixbuf,
    image_view: WeakRef<Image>,
}

// Document system model
trait DocumentModel {}

// Document System is the user-facing system that can handle multiple
// document types (probably eventually? anyway, TODO).
pub struct DocumentSystem {
    model: Rc<RefCell<dyn DocumentModel>>,
}

impl DocumentSystem {
    pub fn new() -> Self {
        DocumentSystem {
            model: Rc::new(RefCell::new(DocumentSystemModel::new())),
        }
    }

    pub fn build_ui(&mut self, menu_manager: &mut MenuManager) -> gtk::Widget {
        let scrolled_window = ScrolledWindowBuilder::new().build();
        let image_view = Image::new();
        scrolled_window.add(&image_view);

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
