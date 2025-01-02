use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct OutputInfo {
    pub name: String,
    pub size_w: i32,
    pub size_h: i32,
    pub position_x: i32,
    pub position_y: i32,
    pub refresh: f64,
}

mod imp {
    use std::cell::RefCell;

    use glib::Properties;

    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::OutputObject)]
    pub struct OutputObject {
        // FIXME: Use Rc<str> instead of String? Would be 100% better
        #[property(name = "name", get, set, type = String, member = name)]
        #[property(name = "size-w", get, set, type = i32, member = size_w)]
        #[property(name = "size-h", get, set, type = i32, member = size_h)]
        #[property(name = "position-x", get, set, type = i32, member = position_x)]
        #[property(name = "position-y", get, set, type = i32, member = position_y)]
        #[property(name = "refresh", get, set, type = f64, member = refresh)]
        pub data: RefCell<OutputInfo>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for OutputObject {
        const NAME: &'static str = "OutputInfo";
        type Type = super::OutputObject;
    }

    // Trait shared by all GObjects
    #[glib::derived_properties]
    impl ObjectImpl for OutputObject {}
}

glib::wrapper! {
    pub struct OutputObject(ObjectSubclass<imp::OutputObject>);
}

impl Default for OutputObject {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl OutputObject {
    pub fn new(output_info: sctk::output::OutputInfo) -> Self {
        let (x, y) = output_info.logical_position.unwrap();
        let current_mode = output_info
            .modes
            .iter()
            .find(|mode| mode.current)
            .clone()
            .expect("output no mode");
        let sctk::output::Mode {
            dimensions: (width, height),
            refresh_rate: refresh,
            ..
        } = *current_mode;

        Object::builder()
            .property("name", output_info.name.unwrap())
            .property("size-w", width)
            .property("size-h", height)
            .property("position-x", x)
            .property("position-y", y)
            .property("refresh", (refresh as f64) / 1000.)
            .build()
    }
}
