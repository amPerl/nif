//! Capturing the preview to image files, so a change can be judged from a directory of pictures
//! rather than from someone holding the mouse.
//!
//! The shots come from the window's own pixels, cropped to the rectangle the preview drew into,
//! rather than from a second offscreen pass. A second pass would need its own view matrix, and
//! every time this viewer has derived the same thing twice the two copies have drifted.

use std::path::{Path, PathBuf};

use eframe::egui;

/// Frames to let a newly opened document settle before its first shot: the dock has to lay out
/// and the scene has to be built before the preview draws anything worth keeping.
const SETTLE_FIRST: u32 = 4;

/// Frames between one shot and the next, which is one turn of the camera.
const SETTLE_NEXT: u32 = 1;

/// Frames to wait for a shot before giving up on it. A document whose preview tab is not on
/// screen never reports a rectangle, and without this the run would wait for it forever.
const PATIENCE: u32 = 600;

/// What `--capture` asks for, settled before there is a window to serve it.
#[derive(Clone, Debug)]
pub struct Request {
    pub dir: PathBuf,
    /// Degrees of yaw between one shot and the next, going all the way round.
    pub step: f32,
    /// Held for every shot, in degrees above the horizon.
    pub pitch: f32,
    /// The window to open. The preview is a part of it, so this sets how large the images are.
    pub size: [f32; 2],
    /// Where on the timeline to shoot, in the file's own seconds. Without this a capture only
    /// ever sees the resting pose, which shows nothing of an animated feature.
    pub time: Option<f32>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            dir: PathBuf::from("captures"),
            step: 45.0,
            // the pitch the viewer opens at, so a capture matches what opening the file shows
            pitch: nif_wgpu::scene::Camera::default().pitch.to_degrees(),
            size: [1280.0, 900.0],
            time: None,
        }
    }
}

impl Request {
    /// The yaws one document is shot from, starting at zero. A step that does not divide 360
    /// stops short of it rather than shooting the first angle twice.
    pub(crate) fn yaws(&self) -> Vec<f32> {
        let step = self.step.abs().clamp(1.0, 360.0);
        let count = (360.0 / step).ceil().max(1.0) as usize;
        (0..count).map(|turn| turn as f32 * step).collect()
    }
}

/// A shot asked for and not yet delivered. The rectangle is kept from the frame that was drawn,
/// since by the time the pixels arrive the layout could have moved.
struct Shot {
    path: PathBuf,
    rect: egui::Rect,
}

/// How to aim the preview for the shot being taken.
pub struct Aim {
    pub document: usize,
    /// Degrees.
    pub yaw: f32,
    pub pitch: f32,
    /// Seconds, or `None` to leave the timeline where it is.
    pub time: Option<f32>,
}

/// Where a capture run has got to. Each open document is walked through every yaw in turn, and
/// the window closes once the last image is written.
pub struct Capture {
    request: Request,
    document: usize,
    /// The yaws left for the document being shot, in the order they are taken.
    todo: Vec<f32>,
    settle: u32,
    inflight: Option<Shot>,
    /// Frames since this shot was aimed at, so a run that cannot make progress ends rather
    /// than hangs.
    waited: u32,
    written: usize,
    problems: Vec<String>,
}

impl Capture {
    pub fn new(request: Request) -> Self {
        let todo = request.yaws();
        Self {
            request,
            document: 0,
            todo,
            settle: SETTLE_FIRST,
            inflight: None,
            waited: 0,
            written: 0,
            problems: Vec::new(),
        }
    }

    /// How to aim this frame, or `None` while a shot is in flight and the camera has to hold
    /// still for it.
    pub fn aim(&self) -> Option<Aim> {
        if self.inflight.is_some() {
            return None;
        }
        Some(Aim {
            document: self.document,
            yaw: *self.todo.first()?,
            pitch: self.request.pitch,
            time: self.request.time,
        })
    }

    /// Saves whatever pixels have arrived and moves on to the next yaw. Returns true when the
    /// run is over, which is when the window should close.
    ///
    /// Called before the preview draws, so a shot delivered this frame frees the camera to turn
    /// in the same frame rather than costing an idle one.
    pub fn deliver(&mut self, ctx: &egui::Context, documents: usize) -> bool {
        if documents == 0 {
            self.problems.push("no file to capture".into());
            return true;
        }
        if let (Some(image), Some(shot)) = (arrived(ctx), self.inflight.take()) {
            match save(&shot, &image, ctx.pixels_per_point()) {
                Ok(()) => self.written += 1,
                Err(problem) => self.problems.push(problem),
            }
            self.next();
        }
        self.waited += 1;
        if self.waited > PATIENCE {
            self.problems
                .push(format!("gave up waiting for document {}", self.document));
            self.inflight = None;
            self.next();
        }
        if self.todo.is_empty() && self.inflight.is_none() {
            self.document += 1;
            self.todo = self.request.yaws();
            self.settle = SETTLE_FIRST;
        }
        self.document >= documents
    }

    /// Done with the yaw being shot, whether it produced an image or not.
    fn next(&mut self) {
        if !self.todo.is_empty() {
            self.todo.remove(0);
        }
        self.settle = SETTLE_NEXT;
        self.waited = 0;
    }

    /// Asks for the pixels of the preview that has just been drawn. `rect` is the region it drew
    /// into, in points, which is what the window's own image is cropped to.
    pub fn shoot(&mut self, ctx: &egui::Context, stem: &str, rect: egui::Rect) {
        if self.inflight.is_some() {
            return;
        }
        let Some(&yaw) = self.todo.first() else {
            return;
        };
        // a repaint every frame, since nothing else here asks for one and the run would stall
        ctx.request_repaint();
        if self.settle > 0 {
            self.settle -= 1;
            return;
        }
        // the time goes in the name too, or two runs of the same file overwrite each other
        let at = match self.request.time {
            Some(time) => format!("-t{time:.2}"),
            None => String::new(),
        };
        let path = self
            .request
            .dir
            .join(format!("{stem}-yaw{:03}{at}.png", yaw.round() as i64));
        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
        self.inflight = Some(Shot { path, rect });
    }

    /// What the run wrote, for whoever is reading the terminal rather than the directory.
    pub fn report(&self) {
        println!(
            "captured {} image{} into {}",
            self.written,
            match self.written {
                1 => "",
                _ => "s",
            },
            self.request.dir.display()
        );
        for problem in &self.problems {
            eprintln!("  {problem}");
        }
    }

    pub fn failed(&self) -> bool {
        !self.problems.is_empty()
    }
}

/// The screenshot this frame's input carries, if one has come back. The readback takes a frame
/// or two, so most frames have none.
fn arrived(ctx: &egui::Context) -> Option<std::sync::Arc<egui::ColorImage>> {
    ctx.input(|input| {
        input.events.iter().find_map(|event| match event {
            egui::Event::Screenshot { image, .. } => Some(image.clone()),
            _ => None,
        })
    })
}

fn save(shot: &Shot, image: &egui::ColorImage, pixels_per_point: f32) -> Result<(), String> {
    let (width, height, rgba) = crop(image, shot.rect, pixels_per_point);
    if width == 0 || height == 0 {
        return Err(format!("{}: the preview drew nothing", shot.path.display()));
    }
    if let Some(dir) = shot.path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    write_png(&shot.path, width, height, &rgba).map_err(|e| format!("{}: {e}", shot.path.display()))
}

/// The preview's own pixels out of the whole window's. Clamped rather than asserted, since the
/// rectangle is in points and the rounding into pixels can reach past the edge by one.
fn crop(image: &egui::ColorImage, rect: egui::Rect, pixels_per_point: f32) -> (u32, u32, Vec<u8>) {
    let (full_width, full_height) = (image.size[0], image.size[1]);
    let at = |value: f32, limit: usize| {
        ((value * pixels_per_point).round().max(0.0) as usize).min(limit)
    };
    let x0 = at(rect.min.x, full_width);
    let x1 = at(rect.max.x, full_width).max(x0);
    let y0 = at(rect.min.y, full_height);
    let y1 = at(rect.max.y, full_height).max(y0);

    let (width, height) = (x1 - x0, y1 - y0);
    let mut rgba = Vec::with_capacity(width * height * 4);
    for row in y0..y1 {
        for column in x0..x1 {
            let pixel = image.pixels[row * full_width + column];
            rgba.extend_from_slice(&pixel.to_array());
        }
    }
    (width as u32, height as u32, rgba)
}

fn write_png(path: &Path, width: u32, height: u32, rgba: &[u8]) -> Result<(), image::ImageError> {
    image::save_buffer(path, rgba, width, height, image::ExtendedColorType::Rgba8)
}
