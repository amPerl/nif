//! Drawing a parsed NIF with wgpu.
//!
//! The crate resolves the textures a file asks for and the shaders it names, and builds the
//! GPU resources both need. It opens no window and runs no event loop.
//!
//! A draw is two calls. `PreviewCall::prepare` builds the bind groups the frame needs and
//! writes what has moved into the buffers already on the device; `PreviewCall::paint` records
//! the draw into a pass the caller has opened. They are separate because a bind group cannot
//! be created while a pass is recording.
//!
//! The `egui` feature implements `egui_wgpu::CallbackTrait` over that pair. It is on by
//! default; without it the crate depends on wgpu alone.

pub mod dds;
#[cfg(feature = "egui")]
pub mod egui;
pub mod library;
pub mod pick;
pub mod scene;
pub mod shaders;
pub mod texture;

/// The rectangle a scene is drawn into, in the units the pointer is reported in. Picking turns
/// a pointer inside it into a ray. Nothing here needs more of a rectangle than its corner and
/// its size, so this stands in for whichever type the application already has.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Viewport {
    /// The corner a pointer's coordinates are measured from, which is the top left.
    pub min: [f32; 2],
    pub size: [f32; 2],
}
