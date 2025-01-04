use std::collections::HashMap;

use tauri::async_runtime::Mutex;

#[derive(Default)]
pub struct FontsInnerState {
    pub loaded_fonts: HashMap<String, rusttype::Font<'static>>,
}

pub type FontsState = Mutex<FontsInnerState>;
