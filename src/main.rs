#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use iced::{window::{self, Position}, Application, Font, Settings};
use iced_tools::{logos::IMG_LOGO_LIGHT, views::main_ui::MyTools};

fn main() -> iced::Result {
    let icon = window::icon::from_file_data(IMG_LOGO_LIGHT, Some(image::ImageFormat::Png)).ok();
    MyTools::run(Settings {
        window: window::Settings {
            icon,
            resizable: false,
            size: (1650, 800),
            decorations:false,
            position:Position::Centered,
            ..window::Settings::default()
        },
        
        default_font: Font::with_name("SimHei"),
        ..Default::default()
    })?;

    Ok(())
}
