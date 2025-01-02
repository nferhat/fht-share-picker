#[derive(serde::Serialize, serde::Deserialize)]
pub enum ScreencastSource {
    Window { foreign_toplevel_handle: String },
    Workspace { output: String, idx: usize },
    Output { name: String },
}
