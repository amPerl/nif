//! nifty, a NIF inspector.
//!
//! Usage:
//!   cargo run -p nifty -- [file.nif]
//!
//! Drop a .nif onto the window to open it.

mod app;
mod scene;
mod texture;

use std::path::PathBuf;

use eframe::egui;

use crate::app::Nifty;

fn main() -> eframe::Result {
    let path = std::env::args().nth(1).map(PathBuf::from);

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        depth_buffer: 32,
        ..Default::default()
    };

    eframe::run_native(
        "nifty",
        options,
        Box::new(move |cc| {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            let mut app = Nifty::new(cc);
            if let Some(path) = path {
                app.open(path);
            }
            Ok(Box::new(app))
        }),
    )
}
