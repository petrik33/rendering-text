mod state;

use std::{path::PathBuf, sync::Mutex};

use rusttype::{point, Font};
use state::FontsState;
use tauri::State;

type RasterizedGlyph = Vec<Vec<u8>>;

#[tauri::command]
async fn rasterize_glyphs(
    state: State<'_, Mutex<FontsState>>,
    font_name: String,
    starting_glyph: u32,
    glyph_count: u32,
    glyph_width: f32,
    glyph_height: f32,
    channel: tauri::ipc::Channel<RasterizedGlyph>,
) -> Result<(), String> {
    let state = state.lock().unwrap();
    let font = state.loaded_fonts.get(&font_name).ok_or("Font not found")?;
    let scale = rusttype::Scale {
        y: glyph_height,
        x: glyph_width,
    };

    for glyph_id in starting_glyph..(starting_glyph + glyph_count) {
        let Some(character) = char::from_u32(glyph_id as u32) else {
            continue;
        };

        let glyph = font
            .glyph(character)
            .scaled(scale)
            .positioned(point(0.0, 0.0));

        if let Some(bb) = glyph.pixel_bounding_box() {
            let width = bb.width() as usize;
            let height = bb.height() as usize;
            let mut pixmap = vec![vec![0u8; width]; height];

            glyph.draw(|x, y, v| {
                let y = y as usize;
                let x = x as usize;
                pixmap[y][x] = (v * 255.0) as u8;
            });

            if let Err(e) = channel.send(pixmap) {
                return Err(format!("Failed to send pixels: {}", e));
            }
        }
    }

    Ok(())
}

#[tauri::command]
fn load_font(path: PathBuf, state: State<'_, Mutex<FontsState>>) -> Result<String, String> {
    let data = std::fs::read(&path)
        .map_err(|err| format!("Failed to read font file '{}': {}", path.display(), err))?;

    let font = Font::try_from_vec(data)
        .ok_or_else(|| format!("Failed to parse font file '{}'", path.display()))?;

    let font_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown Font")
        .to_string();

    let mut state = state.lock().unwrap();
    state.loaded_fonts.insert(font_name.clone(), font);

    Ok(font_name)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(FontsState::default()))
        .invoke_handler(tauri::generate_handler![load_font, rasterize_glyphs])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
