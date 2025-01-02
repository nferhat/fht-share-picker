#[macro_use]
extern crate tracing;

// GTK/adwaita boilerplate
mod application;
mod application_window;
// The actual widgets/objects to provide a selection UI
mod output_info;
mod screencast_source;
mod selection_widget;
mod utils;
mod window_info;

use std::str::FromStr;

use gtk::{
    gdk, gio, glib,
    prelude::{ApplicationExt, ApplicationExtManual},
};

static OUTPUT_PATH: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();

fn main() -> glib::ExitCode {
    let only_message = tracing_subscriber::fmt::format::debug_fn(|writer, field, value| {
        if field.name() == "message" {
            write!(writer, "{value:?}")
        } else {
            Ok(())
        }
    });

    tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .fmt_fields(only_message)
        .init();

    // To avoid ambiguity regarding output, we write the resulting JSON string into a file whose path
    // is passed in by the compositor. Previous version of fht-share-picker used to just spit out
    // data that was manually parsed.
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        error!("You must specify a path to write the results to as a parameter!");
        std::process::exit(1);
    };

    OUTPUT_PATH
        .set(std::path::PathBuf::from_str(&path).unwrap())
        .unwrap();

    glib::set_application_name("fht-share-picker");
    glib::log_set_default_handler(glib::rust_log_handler);
    gio::resources_register_include!("fht.desktop.SharePicker.gresource").unwrap();
    gio::resources_register_include!("fht.desktop.SharePicker.icons.gresource").unwrap();

    let app = application::Application::new();
    app.connect_startup(|_| {
        // TODO: investigate why I have todo it with connect_startup instead of overriding impl
        let provider = gtk::CssProvider::new();
        provider.load_from_string(include_str!("../resources/output-grid.css"));

        // Add the provider to the default screen
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });
    let args = args.collect::<Vec<_>>();
    app.run_with_args(&args)
}
