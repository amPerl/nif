//! Where the renderer meets egui.
//!
//! `nif-wgpu` opens no window and knows nothing about egui, so the two are joined here. Both
//! wrappers exist because the pieces belong to other crates: a trait implementation needs a
//! type of this crate's own to hang on, and a foreign type cannot be given a constructor.

use eframe::egui;
use eframe::egui_wgpu::{self, wgpu};
use nif_wgpu::scene::{Gfx, Preview, PreviewCall, Target};
use nif_wgpu::Viewport;

/// The rectangle a scene is drawn into, as the renderer wants it.
pub fn viewport(rect: egui::Rect) -> Viewport {
    Viewport {
        min: [rect.left(), rect.top()],
        size: [rect.width(), rect.height()],
    }
}

/// Builds the renderer against egui's own device and target, and leaves the preview in
/// `callback_resources`. That map is the only thing a paint callback is handed, so the preview
/// has to live there rather than on the application.
///
/// `samples` is what this application asked eframe for. `RenderState` carries the colour format
/// but not the sample count, so it is repeated here and the two have to agree.
pub fn gfx(render_state: &egui_wgpu::RenderState, samples: u32) -> Gfx {
    let (gfx, preview) = Gfx::new(
        render_state.device.clone(),
        render_state.queue.clone(),
        Target::new(render_state.target_format).with_samples(samples),
    );
    render_state
        .renderer
        .write()
        .callback_resources
        .insert(preview);
    gfx
}

/// One draw, as something egui will accept. The renderer's own call is a foreign type, so the
/// trait is implemented on this instead.
pub struct Callback(pub PreviewCall);

impl egui_wgpu::CallbackTrait for Callback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(preview) = resources.get_mut::<Preview>() {
            self.0.prepare(device, queue, preview);
        }
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        if let Some(preview) = resources.get::<Preview>() {
            self.0.paint(render_pass, preview);
        }
    }
}
