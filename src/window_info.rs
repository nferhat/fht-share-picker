use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use std::cell::RefCell;

    use glib::Properties;
    use sctk::foreign_toplevel_list::ForeignToplevelInfo;

    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::WindowObject)]
    pub struct WindowObject {
        #[property(name = "identifier", get, set, type = String, member = identifier)]
        #[property(name = "title", get, set, type = String, member = title)]
        #[property(name = "app-id", get, set, type = String, member = app_id)]
        pub data: RefCell<ForeignToplevelInfo>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for WindowObject {
        const NAME: &'static str = "WindowInfo";
        type Type = super::WindowObject;
    }

    // Trait shared by all GObjects
    #[glib::derived_properties]
    impl ObjectImpl for WindowObject {}
}

glib::wrapper! {
    pub struct WindowObject(ObjectSubclass<imp::WindowObject>);
}

impl WindowObject {
    pub fn new<S: AsRef<str>>(identifier: S, title: S, app_id: S) -> Self {
        Object::builder()
            .property("identifier", identifier.as_ref())
            .property("title", title.as_ref())
            .property("app-id", app_id.as_ref())
            .build()
    }
}
