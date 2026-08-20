//! nifty, a NIF inspector.
//!
//! Usage:
//!   cargo run -p nifty -- [file.nif] [texture-dir ...]
//!
//! Drop a .nif onto the window to open it, or a folder to add it as a texture directory.

mod app;
mod dds;
mod library;
mod pick;
mod scene;
mod texture;

use std::path::PathBuf;

use eframe::egui;

use crate::app::Nifty;

fn main() -> eframe::Result {
    let mut args = std::env::args().skip(1).map(PathBuf::from);
    let path = args.next();
    let roots: Vec<PathBuf> = args.collect();

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
            for root in roots {
                app.add_texture_root(root);
            }
            if let Some(path) = path {
                app.open(path);
            }
            Ok(Box::new(app))
        }),
    )
}
