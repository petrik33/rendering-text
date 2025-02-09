use std::collections::HashMap;

#[derive(Default)]
pub struct FontsState {
    pub loaded_fonts: HashMap<String, rusttype::Font<'static>>,
}
