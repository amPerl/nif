//! nifty, a NIF inspector.
//!
//! The desktop build takes a file list and options on the command line; the browser build has
//! neither, so a file arrives by being dropped onto the window. What differs between them is
//! only the launch and what the platform offers, and both end up in the same `Nifty`.

mod app;
// Compiled into the web build too, where nothing can start one: reaching it needs a command
// line. Cutting it out would mean threading the same cfg through everything that holds one.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
mod capture;
mod details;
mod preview;

#[cfg(not(target_arch = "wasm32"))]
mod native;

use eframe::egui;

/// A clock that runs on both. `std::time::Instant` compiles for the browser and panics when it
/// is read, so the web build takes its readings from `performance.now()` instead.
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;
#[cfg(target_arch = "wasm32")]
pub use web_time::Instant;

/// Samples per pixel to ask eframe for, and to build the renderer's pipelines against. The two
/// have to agree: the pass is eframe's and a pipeline that disagrees with it is rejected.
///
/// Only the native options carry a multisampling setting, so the browser's pass is single
/// sampled and the pipelines have to be built to match it.
#[cfg(not(target_arch = "wasm32"))]
pub const MULTISAMPLING: u32 = 4;
#[cfg(target_arch = "wasm32")]
pub const MULTISAMPLING: u32 = 1;

/// The icon font both builds draw their eyes and arrows from.
pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    ctx.set_fonts(fonts);
}

/// Starts the viewer in the page's canvas.
///
/// Nothing is opened at startup: a browser has no argument list and no directory to scan, so a
/// file arrives by being dropped onto the window.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    use wasm_bindgen::JsCast as _;

    // a panic in the browser is otherwise a bare "unreachable executed" in the console
    std::panic::set_hook(Box::new(|info| {
        web_sys::console::error_1(&format!("{info}").into());
    }));

    let canvas = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("nifty"))
        .and_then(|element| element.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        .expect("a canvas with id \"nifty\"");

    let options = eframe::WebOptions {
        depth_buffer: 32,
        ..Default::default()
    };

    wasm_bindgen_futures::spawn_local(async move {
        let result = eframe::WebRunner::new()
            .start(
                canvas,
                options,
                Box::new(|cc| {
                    install_fonts(&cc.egui_ctx);
                    Ok(Box::new(app::Nifty::new(cc)))
                }),
            )
            .await;
        if let Err(error) = result {
            web_sys::console::error_1(&format!("nifty failed to start: {error:?}").into());
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    native::main()
}

/// The browser starts through `start` above, which the module calls once it is instantiated.
#[cfg(target_arch = "wasm32")]
fn main() {}
