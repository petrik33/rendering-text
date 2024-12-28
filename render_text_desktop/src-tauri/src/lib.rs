use rusttype::{Font, Scale};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[tauri::command]
fn list_fonts(app: AppHandle) -> Vec<String> {
    let mut fonts = vec!["Arial".to_string(), "Times New Roman".to_string()];
    if let Some(app_dir) = app.path().app_data_dir().ok() {
        let user_fonts_dir = app_dir.join("user_fonts");
        if let Ok(entries) = fs::read_dir(user_fonts_dir) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    fonts.push(file_name);
                }
            }
        }
    }
    fonts
}

#[tauri::command]
fn import_font(app: AppHandle, path: String) -> Result<(), String> {
    if let Some(app_dir) = app.path().app_data_dir().ok() {
        let user_fonts_dir = app_dir.join("user_fonts");
        fs::create_dir_all(&user_fonts_dir).unwrap();
        let font_name = PathBuf::from(&path)
            .file_name()
            .ok_or("Invalid file name")?
            .to_str()
            .ok_or("Invalid UTF-8")?
            .to_string();
        let dest = user_fonts_dir.join(&font_name);
        fs::copy(&path, dest).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Failed to determine app directory".to_string())
    }
}

#[tauri::command]
fn render_text(text: String, font_name: String) -> String {
    let font_data = include_bytes!("../../assets/Roboto-Regular.ttf"); // Default font
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    let scale = Scale::uniform(40.0);
    let glyphs: Vec<_> = font
        .layout(&text, scale, rusttype::point(0.0, 0.0))
        .collect();
    format!("Rendered {} glyphs", glyphs.len()) // Example, replace with actual rendering logic
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_fonts,
            import_font,
            render_text
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
