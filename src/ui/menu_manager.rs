use gtk::prelude::*;
use gtk::{Container, Label, Menu, MenuBar, MenuItem, MenuShell, Widget};

#[derive(Debug)]
pub enum MenuManagerError {
    PathContainsEmptySegments,
    NotImplemented,
}

pub struct MenuManager {
    menubar: MenuBar,
}

impl MenuManager {
    pub fn new() -> Self {
        MenuManager {
            menubar: MenuBar::new(),
        }
    }

    pub fn get_menubar(&self) -> MenuBar {
        self.menubar.clone()
    }

    // Add a new menu option at the specified path, using either '/' or '\' as
    // path separators. If menus/submenus do not already exist, they will be
    // added as needed. The MenuItem at the leaf of the path is returned.
    // TODO: Allow menu items which allow separators in them somehow.
    pub fn add_menu_item(&mut self, path: &'static str) -> Result<MenuItem, MenuManagerError> {
        let segments: Vec<&str> = path.split(|c| c == '/' || c == '\\').collect();

        // Check for empty segments and return an error if found
        for seg in &segments {
            if (*seg).eq("") {
                return Err(MenuManagerError::PathContainsEmptySegments);
            }
        }

        // At this point, all segments SHOULD be valid...
        let mut working_item_opt: Option<MenuItem> = None;
        for seg in segments {
            // Get or create a menushell (submenu) from the current item
            let shell = match &working_item_opt {
                Some(wi) => Self::get_menu_shell(wi.clone()),
                None => self.menubar.clone().upcast::<MenuShell>(),
            };

            // Get/create a menu item for the current

            working_item_opt = Some(Self::get_menu_item(seg, shell));
        }

        Ok(working_item_opt.expect("No final menu item was created"))
    }

    // Get (or create) a menu item in the specified container whose label text
    // matches the `name` argument
    fn get_menu_item(name: &str, in_menushell: MenuShell) -> MenuItem {
        for c in in_menushell.get_children() {
            let mi = c
                .clone()
                .downcast::<MenuItem>()
                .expect("Non-menu item found! D8");
            let mi_children = mi.get_children();
            for mi_c in mi_children {
                if let Ok(label) = mi_c.clone().downcast::<Label>() {
                    if let Some(text) = label.get_text() {
                        if text == name {
                            return mi_c
                                .downcast::<MenuItem>()
                                .expect("MenuItem > Widget roundtrip failed");
                        }
                    }
                }
            }
        }

        // If we got here, there isn't a MenuItem with a matching label
        // Make one.
        let result = MenuItem::new_with_label(name);
        in_menushell.append(&result);
        result
    }
    fn get_menu_shell(menu_item: MenuItem) -> MenuShell {
        if let Some(sub) = menu_item.get_submenu() {
            return sub
                .downcast::<MenuShell>()
                .expect("Sub-menu Widget was not a MenuShell");
        }

        let shell = Menu::new();
        menu_item.set_submenu(Some(&shell));
        shell.upcast::<MenuShell>()
    }
}
