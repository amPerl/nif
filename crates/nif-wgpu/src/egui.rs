//! Drawing through egui's wgpu integration.
//!
//! The draw is `PreviewCall`'s own `prepare` and `paint`, neither of which knows about egui.
//! This hands them what a paint callback is given.

use crate::scene::{Gfx, Preview, PreviewCall};
use crate::Viewport;

impl From<egui::Rect> for Viewport {
    fn from(rect: egui::Rect) -> Self {
        Self {
            min: [rect.left(), rect.top()],
            size: [rect.width(), rect.height()],
        }
    }
}

impl Gfx {
    /// Builds against egui's own device and target, and leaves the preview in
    /// `callback_resources`. That map is the only thing a paint callback is handed, so the
    /// preview has to live there rather than on the application.
    pub fn from_render_state(render_state: &egui_wgpu::RenderState) -> Self {
        let (gfx, preview) = Gfx::new(
            render_state.device.clone(),
            render_state.queue.clone(),
            render_state.target_format,
        );
        render_state
            .renderer
            .write()
            .callback_resources
            .insert(preview);
        gfx
    }
}

impl egui_wgpu::CallbackTrait for PreviewCall {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(preview) = resources.get_mut::<Preview>() {
            PreviewCall::prepare(self, device, queue, preview);
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
            PreviewCall::paint(self, render_pass, preview);
        }
    }
}
