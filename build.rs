fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "fht.desktop.SharePicker.gresource",
    );

    glib_build_tools::compile_resources(
        &["resources/icons"],
        "resources/icons/resources.gresource.xml",
        "fht.desktop.SharePicker.icons.gresource",
    );
}
