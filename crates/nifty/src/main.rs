//! nifty, a NIF inspector.
//!
//! Usage:
//!   cargo run -p nifty -- [file.nif ...] [texture-dir ...]
//!
//! Arguments are sorted by what they are, so files and directories can be given in any order.
//! Drop a .nif onto the window to open it, or a folder to add it as a texture directory.

mod app;
mod dds;
mod details;
mod library;
mod pick;
mod scene;
mod shaders;
mod texture;

use std::path::PathBuf;

use eframe::egui;

use crate::app::Nifty;

fn main() -> eframe::Result {
    let (roots, files): (Vec<PathBuf>, Vec<PathBuf>) = std::env::args()
        .skip(1)
        .map(PathBuf::from)
        .partition(|path| path.is_dir());

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        depth_buffer: 32,
        multisampling: crate::scene::MSAA_SAMPLES as u16,
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
            // roots first, so the scenes resolve textures and shaders as they are built
            for root in roots {
                app.add_root(root);
            }
            for file in files {
                app.open(file);
            }
            app.focus_first();
            Ok(Box::new(app))
        }),
    )
}
