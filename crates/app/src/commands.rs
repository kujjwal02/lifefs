//! Commands the window calls. tauri-specta turns each one into a typed TypeScript function.

use tauri_specta::{Builder, collect_commands};

pub fn builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![app_version])
}

/// The version the window shows in its footer.
#[tauri::command]
#[specta::specta]
fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
