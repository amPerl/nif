//! Draws a NIF in a window of its own, with winit and wgpu and nothing else.
//!
//! `cargo run -p nif-wgpu --example window -- file.nif [texture root...] [--anisotropy=N]`
//!
//! `--anisotropy` is samples along the long axis of a surface seen at an angle, 1 for none and
//! 4 by default. wgpu clamps it to what the device supports.
//!
//! The crate builds GPU resources and records draws. Everything around that is the caller's:
//! the window, the surface, the device and queue, the colour and depth attachments, and the
//! render pass they are bound to. This shows what that amounts to.
//!
//! Drag to orbit, scroll to zoom, and drag with the middle or right button, or with shift
//! held, to pan.
//!
//! A file that animates runs from the moment it opens and keeps running, looping over the span
//! its controllers cover. It does not pick or watch the file for changes.

use std::sync::Arc;

use nif::glam::camera::rh::{proj::directx::perspective, view::look_at_mat4};
use nif::glam::Vec3;
use nif_wgpu::library::TextureLibrary;
use nif_wgpu::scene::{
    camera_uniform, resolve, Camera, CameraBinding, Frame, Gfx, Instance, Light, LodMode, Preview,
    PreviewCall, Scene, Target, Viewpoint,
};
use nif_wgpu::shaders::Shaders;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::ModifiersState;
use winit::window::{Window, WindowId};

/// How wide the view is, vertically.
const FOV: f32 = 60.0;

/// Samples per pixel. The colour and depth attachments below are made at this count and the
/// renderer builds its pipelines against it, so the two cannot drift apart.
const SAMPLES: u32 = 4;

/// What `--anisotropy` starts at. Sharpens a surface seen at an angle, which the engine had no
/// setting for, so an application reproducing a frame faithfully would leave this at 1.
const ANISOTROPY: u16 = 4;

fn main() {
    let mut anisotropy = ANISOTROPY;
    let mut path = None;
    let mut library = TextureLibrary::default();

    // sorted by what each argument is, so a file, a directory and a flag can be given in any
    // order
    for arg in std::env::args().skip(1) {
        if let Some(value) = arg.strip_prefix("--anisotropy=") {
            match value.parse() {
                Ok(level) => anisotropy = level,
                Err(_) => {
                    eprintln!("--anisotropy wants a number, not {value:?}");
                    std::process::exit(2);
                }
            }
        } else if std::path::Path::new(&arg).is_dir() {
            library.add_root(arg.into());
        } else if path.is_none() {
            path = Some(arg);
        } else {
            eprintln!("only one file is drawn, so {arg:?} has nowhere to go");
            std::process::exit(2);
        }
    }

    let Some(path) = path else {
        eprintln!("usage: window <file.nif> [texture root...] [--anisotropy=N]");
        std::process::exit(2);
    };

    let bytes = std::fs::read(&path).expect("read the file");
    let nif = nif::Nif::parse(&mut std::io::Cursor::new(&bytes)).expect("parse the file");

    let event_loop = EventLoop::new().expect("an event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App {
        nif: Arc::new(nif),
        library,
        shaders: Shaders::default(),
        anisotropy,
        window: None,
        state: None,
    };
    event_loop.run_app(&mut app).expect("run");
}

struct App {
    nif: Arc<nif::Nif>,
    library: TextureLibrary,
    shaders: Shaders,
    anisotropy: u16,
    window: Option<Arc<Window>>,
    state: Option<State>,
}

/// What the pointer is doing, since winit reports buttons and movement separately rather than
/// as the drag the camera wants.
#[derive(Default)]
struct Pointer {
    at: Option<(f64, f64)>,
    primary: bool,
    panning: bool,
    modifiers: ModifiersState,
}

impl Pointer {
    /// How far the pointer moved since it was last here, and which of the two drags it is.
    /// `None` when no button is down.
    fn drag(&mut self, x: f64, y: f64) -> Option<(f32, f32, bool)> {
        let last = self.at.replace((x, y));
        let (from_x, from_y) = last?;
        if !self.primary && !self.panning {
            return None;
        }
        let pans = self.panning || self.modifiers.shift_key();
        Some(((x - from_x) as f32, (y - from_y) as f32, pans))
    }
}

/// Everything that only exists once there is a surface to draw into.
struct State {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    gfx: Gfx,
    preview: Preview,
    scene: Arc<Scene>,
    camera: Arc<CameraBinding>,
    /// A multisampled colour and depth pair, resolving into the surface at the end of the
    /// pass. Drawing into the surface directly fails validation on the first `set_pipeline`
    /// whenever the target asks for more than one sample.
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    /// What the camera orbits, and what sets how far out it sits.
    center: Vec3,
    radius: f32,
    /// Yaw and pitch about a target, an absolute distance so zooming does not inherit the
    /// scale of the scene, and a pan across the view plane.
    rig: Camera,
    pointer: Pointer,
    /// The file's own copy, kept because resolving a frame reads the blocks every tick.
    nif: Arc<nif::Nif>,
    /// The particle simulation, which carries state from one frame to the next.
    systems: Vec<nif::psys::System>,
    /// Where the controllers start and end, or `None` where the file has none. A file with a
    /// span loops over it; one without draws its resting pose.
    span: Option<(f32, f32)>,
    started: std::time::Instant,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attributes = Window::default_attributes().with_title("nif-wgpu");
        let window = Arc::new(event_loop.create_window(attributes).expect("a window"));
        self.window = Some(window.clone());
        self.state = Some(State::new(
            window,
            self.nif.clone(),
            &self.library,
            &self.shaders,
            self.anisotropy,
        ));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = self.state.as_mut() else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::ModifiersChanged(modifiers) => {
                state.pointer.modifiers = modifiers.state();
            }
            WindowEvent::MouseInput {
                button, state: it, ..
            } => {
                let down = it == ElementState::Pressed;
                match button {
                    MouseButton::Left => state.pointer.primary = down,
                    MouseButton::Middle | MouseButton::Right => state.pointer.panning = down,
                    _ => {}
                }
            }
            WindowEvent::CursorLeft { .. } => state.pointer.at = None,
            WindowEvent::CursorMoved { position, .. } => {
                if let Some((dx, dy, pans)) = state.pointer.drag(position.x, position.y) {
                    state.turn(dx, dy, pans);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // a wheel notch is a line rather than a distance, so it is scaled to roughly
                // what a UI toolkit would have reported for the same turn of the wheel
                let scroll = match delta {
                    MouseScrollDelta::LineDelta(_, lines) => lines * 50.0,
                    MouseScrollDelta::PixelDelta(pixels) => pixels.y as f32,
                };
                state.zoom(scroll);
            }
            WindowEvent::RedrawRequested => {
                state.draw();
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

impl State {
    fn new(
        window: Arc<Window>,
        shared: Arc<nif::Nif>,
        library: &TextureLibrary,
        shaders: &Shaders,
        anisotropy: u16,
    ) -> Self {
        let nif = shared.as_ref();
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).expect("a surface");
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .expect("an adapter");
        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
                .expect("a device");

        // The renderer writes gamma space values, so the surface has to be a linear format or
        // everything comes out too dark. That is the same reason its textures are `Rgba8Unorm`.
        let size = window.inner_size();
        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(|format| !format.is_srgb())
            .unwrap_or(capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let target = Target::new(format).with_samples(SAMPLES);
        let (mut gfx, preview) = Gfx::new(device, queue, target);
        // before the scene is built, since a sampler is bound alongside the texture it reads
        // and changing one afterwards means building the scene again
        gfx.set_anisotropy(anisotropy);
        let (scene, unhandled, partial) = gfx.build_scene(nif, library, shaders);
        for note in unhandled.iter().chain(partial.iter()) {
            eprintln!("{note}");
        }
        let camera = Arc::new(gfx.camera());
        let (color, depth) = attachments(&gfx.device, target, config.width, config.height);
        let (center, radius) = (scene.center, scene.radius.max(1.0));
        let span = nif::anim::span(&nif.blocks);
        let mut systems = nif::psys::systems(&nif.blocks);
        nif_wgpu::scene::place_emitters(nif, &mut systems);

        Self {
            surface,
            config,
            gfx,
            preview,
            scene: Arc::new(scene),
            camera,
            color,
            depth,
            center,
            radius,
            rig: Camera::default(),
            pointer: Pointer::default(),
            nif: shared,
            systems,
            span,
            started: std::time::Instant::now(),
        }
    }

    /// Where on the timeline this tick is. A file whose controllers cover a span loops over it;
    /// one with no controllers stays at rest.
    fn time(&self) -> Option<f32> {
        let (start, end) = self.span?;
        let elapsed = self.started.elapsed().as_secs_f32();
        Some(start + elapsed % (end - start).max(1e-6))
    }

    /// How far the rig sits from what it looks at. Absolute, so a shape framed close stays
    /// close when the scene around it is large.
    fn distance(&self) -> f32 {
        self.rig.distance.unwrap_or(self.radius * 2.5)
    }

    /// The rig's own axes, from which everything else in the view is derived.
    fn basis(&self) -> (Vec3, Vec3, Vec3) {
        let direction = Vec3::new(
            self.rig.yaw.cos() * self.rig.pitch.cos(),
            self.rig.yaw.sin() * self.rig.pitch.cos(),
            self.rig.pitch.sin(),
        );
        let forward = -direction;
        let right = forward.cross(Vec3::Z).normalize_or_zero();
        (direction, right, right.cross(forward).normalize_or_zero())
    }

    /// Orbits, or pans across the view plane at the rate the pointer moves over it.
    fn turn(&mut self, dx: f32, dy: f32, pans: bool) {
        if !pans {
            self.rig.yaw -= dx * 0.01;
            self.rig.pitch =
                (self.rig.pitch + dy * 0.01).clamp(-Camera::PITCH_LIMIT, Camera::PITCH_LIMIT);
            return;
        }
        let (_, right, up) = self.basis();
        let height = self.config.height.max(1) as f32;
        let per_pixel = 2.0 * self.distance() * (FOV.to_radians() * 0.5).tan() / height;
        self.rig.pan += (up * dy - right * dx) * per_pixel;
    }

    /// The floor is absolute, so a small shape inside a sprawling scene can still be approached.
    fn zoom(&mut self, scroll: f32) {
        if scroll == 0.0 {
            return;
        }
        let distance = self.distance() * (1.0 - scroll * 0.002);
        self.rig.distance = Some(distance.clamp(1e-3, self.radius * 1000.0));
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        self.surface.configure(&self.gfx.device, &self.config);
        let (color, depth) = attachments(
            &self.gfx.device,
            self.gfx.target,
            self.config.width,
            self.config.height,
        );
        self.color = color;
        self.depth = depth;
    }

    fn draw(&mut self) {
        use wgpu::CurrentSurfaceTexture as Acquired;
        let frame = match self.surface.get_current_texture() {
            Acquired::Success(frame) | Acquired::Suboptimal(frame) => frame,
            // the next redraw asks again, so a frame that cannot be acquired is skipped
            _ => return,
        };
        let target = frame.texture.create_view(&Default::default());

        // Everything that moves is resolved here: poses, skins, morphs, driven material and
        // shader values, and the particle simulation, which is why this takes the systems by
        // reference. A file with no controllers resolves to the resting pose.
        let resolved = match self.time() {
            Some(time) => Arc::new(resolve(
                &self.nif,
                Some(&self.scene),
                &mut self.systems,
                &Default::default(),
                Viewpoint {
                    time: Some(time),
                    camera: None,
                },
            )),
            None => Arc::new(Frame::default()),
        };

        let (direction, right, up) = self.basis();
        let distance = self.distance();
        let look_at = self.center + self.rig.pan;
        let eye = look_at + direction * distance;
        let view = look_at_mat4(
            eye - self.scene.origin,
            look_at - self.scene.origin,
            Vec3::Z,
        );

        // The near plane tracks the distance so precision stays where the camera looks. The far
        // plane cannot, since it has to clear the whole scene from wherever the eye now is.
        let aspect = self.config.width as f32 / self.config.height.max(1) as f32;
        let reach = (eye.distance(self.center) + self.scene.animated_radius) * 1.25;
        let projection = perspective(
            FOV.to_radians(),
            aspect,
            (distance * 0.01).max(1e-5),
            reach.max(1e-3),
        );
        let view_proj = projection * view;

        let uniform = camera_uniform(
            view_proj,
            eye - self.scene.origin,
            true,
            true,
            &Light::default(),
            &self.scene.lights,
            self.scene.origin,
        );
        self.gfx.write_camera(&self.camera, &uniform);

        let call = PreviewCall {
            instances: vec![Instance {
                scene: self.scene.clone(),
                frame: resolved,
                camera: self.camera.clone(),
            }],
            wireframe: false,
            grid: false,
            cull: true,
            selected: None,
            eye,
            right,
            up,
            lod_mode: LodMode::Auto,
            lod_distance: 0.0,
        };

        let mut encoder = self.gfx.device.create_command_encoder(&Default::default());
        call.prepare(&self.gfx.device, &self.gfx.queue, &mut self.preview);
        {
            let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("nif"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.color,
                    depth_slice: None,
                    resolve_target: Some(&target),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.06,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            call.paint(&mut pass.forget_lifetime(), &self.preview);
        }
        self.gfx.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

/// The colour and depth the pass draws into, made to match what the renderer was built for.
fn attachments(
    device: &wgpu::Device,
    target: Target,
    width: u32,
    height: u32,
) -> (wgpu::TextureView, wgpu::TextureView) {
    let make = |label, format| {
        device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: target.samples,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
            .create_view(&Default::default())
    };
    (
        make("nif colour", target.color),
        make("nif depth", target.depth),
    )
}
