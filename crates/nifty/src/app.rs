use std::{collections::HashSet, io::Cursor, path::PathBuf, sync::Arc};

use eframe::egui::{self, text::LayoutJob, Color32, FontId, TextFormat, WidgetText};
use eframe::egui_wgpu;
use egui_dock::{DockArea, DockState, Style, TabViewer};
use egui_ltreeview::{Action, NodeBuilder, TreeView};
use egui_phosphor::regular as icon;
use nif::glam::camera::rh::{proj::directx::perspective, view::look_at_mat4};
use nif::glam::Vec3;
use nif::{blocks::Block, Nif};

use crate::scene::{Camera, Gfx, PreviewCall, Scene};

struct Loaded {
    path: PathBuf,
    nif: Nif,
    consumed: usize,
    size: usize,
}

struct State {
    loaded: Option<Loaded>,
    status: Option<String>,
    selected: Option<usize>,
    scene: Option<Arc<Scene>>,
    camera: Camera,
    wireframe: bool,
    cull: bool,
    colors: bool,
    textures: bool,
}

#[derive(Debug, PartialEq)]
enum Tab {
    Hierarchy,
    Blocks,
    Details,
    Preview,
    Header,
}

/// One open file: its own views, selection and camera.
struct Document {
    state: State,
    dock: DockState<Tab>,
}

impl Document {
    fn title(&self) -> String {
        match &self.state.loaded {
            Some(loaded) => loaded
                .path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| loaded.path.display().to_string()),
            None => "untitled".into(),
        }
    }

    /// Frame the selected shape, or the whole scene when the selection has no geometry.
    fn focus_selected(&mut self) {
        let Some(scene) = &self.state.scene else {
            return;
        };
        let mesh = self.state.selected.and_then(|index| {
            scene
                .meshes
                .iter()
                .find(|m| m.shape_block == index || m.data_block == index)
        });

        match mesh {
            Some(mesh) => {
                self.state.camera.pan = mesh.center - scene.center;
                self.state.camera.distance = Some(mesh.radius * 2.5);
            }
            None => {
                self.state.camera.pan = Vec3::ZERO;
                self.state.camera.distance = None;
            }
        }
    }
}

fn default_dock() -> DockState<Tab> {
    let mut dock = DockState::new(vec![Tab::Hierarchy, Tab::Header, Tab::Blocks]);
    dock.main_surface_mut().split_right(
        egui_dock::NodeIndex::root(),
        0.4,
        vec![Tab::Preview, Tab::Details],
    );
    dock
}

pub struct Nifty {
    documents: Vec<Document>,
    active: usize,
    error: Option<String>,
    gfx: Option<Gfx>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            loaded: None,
            status: None,
            selected: None,
            scene: None,
            camera: Camera::default(),
            wireframe: false,
            cull: true,
            colors: true,
            textures: true,
        }
    }
}

impl Nifty {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            documents: Vec::new(),
            active: 0,
            error: None,
            gfx: cc.wgpu_render_state.as_ref().map(Gfx::new),
        }
    }

    /// Each file opens as its own document rather than replacing the current one.
    pub fn open(&mut self, path: PathBuf) {
        self.error = None;

        let bytes = match std::fs::read(&path) {
            Ok(bytes) => bytes,
            Err(e) => {
                self.error = Some(e.to_string());
                return;
            }
        };

        let mut reader = Cursor::new(&bytes);
        let nif = match Nif::parse(&mut reader) {
            Ok(nif) => nif,
            Err(e) => {
                self.error = Some(first_line(&e.to_string()));
                return;
            }
        };

        let mut state = State::default();
        state.status = Some(match &self.gfx {
            Some(gfx) => {
                let scene = gfx.build_scene(&nif);
                let shapes = scene.meshes.len();
                state.scene = Some(Arc::new(scene));
                format!("{} blocks, {} shapes", nif.blocks.len(), shapes)
            }
            None => format!("{} blocks", nif.blocks.len()),
        });
        state.loaded = Some(Loaded {
            path,
            nif,
            consumed: reader.position() as usize,
            size: bytes.len(),
        });

        self.documents.push(Document {
            state,
            dock: default_dock(),
        });
        self.active = self.documents.len() - 1;
    }
}

fn first_line(text: &str) -> String {
    text.lines()
        .map(|l| l.trim())
        .find(|l| !l.is_empty() && !l.contains("Backtrace"))
        .unwrap_or("unknown error")
        .chars()
        .take(200)
        .collect()
}

struct Palette {
    dim: Color32,
    plain: Color32,
    accent: Color32,
    font: FontId,
}

impl Palette {
    fn of(ui: &egui::Ui) -> Self {
        Self {
            dim: ui.visuals().weak_text_color(),
            plain: ui.visuals().text_color(),
            accent: ui.visuals().hyperlink_color,
            font: egui::TextStyle::Body.resolve(ui.style()),
        }
    }
}

fn label_for(blocks: &[Block], index: usize, slot: Option<&str>, palette: &Palette) -> LayoutJob {
    let mut job = LayoutJob::default();
    let mut push = |text: &str, color: Color32| {
        job.append(
            text,
            0.0,
            TextFormat {
                font_id: palette.font.clone(),
                color,
                ..Default::default()
            },
        );
    };

    let Some(block) = blocks.get(index) else {
        push(&format!("{index} out of range"), palette.dim);
        return job;
    };

    push(&format!("{index}  "), palette.dim);
    if let Some(slot) = slot {
        push(&format!("{slot}  "), palette.dim);
    }
    push(block.name(), palette.plain);

    let mut name = block
        .object_net()
        .map(|o| o.name.to_string_lossy().into_owned())
        .unwrap_or_default();
    // source textures are almost always unnamed, and the path is what identifies them
    if name.is_empty() {
        if let Block::NiSourceTexture(texture) = block {
            name = texture.file_name.to_string_lossy().into_owned();
        }
    }
    if !name.is_empty() {
        push("  ", palette.dim);
        push(&name, palette.accent);
    }
    job
}

fn icon_for(block: &Block) -> &'static str {
    let name = block.name();
    match block {
        Block::NiPixelData(_) | Block::NiSourceTexture(_) | Block::NiSourceCubeMap(_) => {
            icon::IMAGE
        }
        Block::NiPalette(_) => icon::PALETTE,
        Block::NiCamera(_) => icon::VIDEO_CAMERA,
        Block::NiFlipController(_) => icon::FILM_STRIP,
        _ if name.ends_with("Data") => icon::GRID_FOUR,
        _ if name.ends_with("Property") => icon::PAINT_BRUSH,
        _ if name.ends_with("ExtraData") => icon::TAG,
        _ if name.ends_with("Controller") || name.ends_with("Ctlr") => icon::PLAY_CIRCLE,
        _ if name.ends_with("Interpolator") => icon::CHART_LINE,
        _ if name.ends_with("Light") => icon::LIGHTBULB,
        _ if name.ends_with("Node") => icon::CUBE,
        _ if name.starts_with("NiPSys") || name.contains("Particle") => icon::SPARKLE,
        _ if name.starts_with("NiTri") => icon::TRIANGLE,
        _ => icon::CIRCLE,
    }
}

struct Link {
    index: usize,
    slot: Option<String>,
}

impl Link {
    fn plain(index: usize) -> Self {
        Self { index, slot: None }
    }
}

fn linked(block: &Block) -> Vec<Link> {
    let mut out = Vec::new();
    for refs in [
        block.child_refs(),
        block.property_refs(),
        block.extra_data_refs(),
    ]
    .into_iter()
    .flatten()
    {
        out.extend(refs.iter().filter_map(|r| r.index()).map(Link::plain));
    }

    // the texture chain hangs off typed fields rather than the generic ref lists, so none of
    // it reaches the tree without these
    match block {
        Block::NiTexturingProperty(property) => {
            out.extend(property.textures().filter_map(|(slot, desc)| {
                Some(Link {
                    index: desc.source_ref.index()?,
                    slot: Some(slot.to_string()),
                })
            }));
        }
        Block::NiSourceTexture(texture) => {
            out.extend(texture.pixel_data_ref.index().map(Link::plain));
        }
        Block::NiPixelData(pixels) => {
            out.extend(pixels.palette_ref.index().map(Link::plain));
        }
        _ => {}
    }
    out
}

fn describe(block: &Block) -> String {
    if let Block::NiPixelData(pixels) = block {
        let mips: Vec<String> = pixels
            .mipmaps
            .iter()
            .map(|m| format!("  {}x{} @ {}", m.width, m.height, m.offset))
            .collect();
        let faces: Vec<String> = pixels
            .pixel_data
            .iter()
            .enumerate()
            .map(|(i, f)| format!("  face {i}: {} bytes", f.data.len()))
            .collect();
        return format!(
            "NiPixelData\nbytes_per_pixel: {}\nmipmaps:\n{}\nfaces:\n{}",
            pixels.bytes_per_pixel,
            mips.join("\n"),
            faces.join("\n")
        );
    }

    let text = format!("{block:#?}");
    match text.char_indices().nth(200_000) {
        Some((cut, _)) => format!("{}\n…truncated", &text[..cut]),
        None => text,
    }
}

struct Viewer<'a> {
    state: &'a mut State,
    gfx: Option<&'a Gfx>,
}

impl TabViewer for Viewer<'_> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        match tab {
            Tab::Hierarchy => "Hierarchy",
            Tab::Blocks => "Blocks",
            Tab::Details => "Details",
            Tab::Preview => "Preview",
            Tab::Header => "Header",
        }
        .into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if let Tab::Preview = tab {
            self.preview(ui);
            return;
        }

        let Some(loaded) = &self.state.loaded else {
            ui.centered_and_justified(|ui| ui.label("no file open"));
            return;
        };
        let blocks = &loaded.nif.blocks;

        match tab {
            Tab::Hierarchy => {
                let mut clicked = None;
                let palette = Palette::of(ui);
                egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                    let (_, actions) =
                        TreeView::new(ui.make_persistent_id("hierarchy")).show(ui, |builder| {
                            let mut path = HashSet::new();
                            for (index, _) in loaded.nif.roots() {
                                add_node(builder, blocks, &Link::plain(index), &mut path, &palette);
                            }
                        });
                    for action in actions {
                        if let Action::SetSelected(selected) = action {
                            clicked = selected.first().copied();
                        }
                    }
                });
                if clicked.is_some() {
                    self.state.selected = clicked;
                }
            }
            Tab::Blocks => {
                let mut clicked = None;
                let palette = Palette::of(ui);
                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        for (index, block) in blocks.iter().enumerate() {
                            let selected = self.state.selected == Some(index);
                            ui.horizontal(|ui| {
                                ui.label(icon_for(block));
                                if ui
                                    .selectable_label(
                                        selected,
                                        label_for(blocks, index, None, &palette),
                                    )
                                    .clicked()
                                {
                                    clicked = Some(index);
                                }
                            });
                        }
                    });
                if clicked.is_some() {
                    self.state.selected = clicked;
                }
            }
            Tab::Details => match self.state.selected.and_then(|i| blocks.get(i)) {
                None => {
                    ui.centered_and_justified(|ui| ui.label("no block selected"));
                }
                Some(block) => {
                    egui::ScrollArea::both()
                        .auto_shrink(false)
                        .show(ui, |ui| ui.monospace(describe(block)));
                }
            },
            Tab::Preview => {}
            Tab::Header => {
                let header = &loaded.nif.header;
                egui::Grid::new("header").num_columns(2).show(ui, |ui| {
                    ui.label("file");
                    ui.monospace(loaded.path.display().to_string());
                    ui.end_row();
                    ui.label("version");
                    ui.monospace(format!("{:#010x}", header.version));
                    ui.end_row();
                    ui.label("user version");
                    ui.monospace(header.user_version.to_string());
                    ui.end_row();
                    ui.label("endian");
                    ui.monospace(format!("{:?}", header.endian_type));
                    ui.end_row();
                    ui.label("blocks");
                    ui.monospace(blocks.len().to_string());
                    ui.end_row();
                    ui.label("block types");
                    ui.monospace(header.block_types().len().to_string());
                    ui.end_row();
                    ui.label("roots");
                    ui.monospace(loaded.nif.footer.root_refs.len().to_string());
                    ui.end_row();
                    ui.label("bytes");
                    ui.monospace(format!("{} read of {}", loaded.consumed, loaded.size));
                    ui.end_row();
                });
                ui.separator();
                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        for (i, t) in header.block_types().iter().enumerate() {
                            ui.monospace(format!("{i:3}  {}", t.to_string_lossy()));
                        }
                    });
            }
        }
    }
}

impl Viewer<'_> {
    fn preview(&mut self, ui: &mut egui::Ui) {
        let (Some(scene), Some(gfx)) = (self.state.scene.clone(), self.gfx) else {
            ui.centered_and_justified(|ui| ui.label("nothing to draw"));
            return;
        };
        if scene.meshes.is_empty() {
            ui.centered_and_justified(|ui| ui.label("no drawable geometry"));
            return;
        }

        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut self.state.wireframe, "wireframe");
            ui.checkbox(&mut self.state.cull, "cull backfaces");
            ui.checkbox(&mut self.state.colors, "material colours");
            ui.checkbox(&mut self.state.textures, "textures");
            if ui.button("reset view").clicked() {
                self.state.camera = Camera::default();
            }
            ui.separator();
            ui.label(format!(
                "{} shapes, radius {:.1}  ·  F frames the selection",
                scene.meshes.len(),
                scene.radius
            ));
        });

        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

        // egui clips the callback rect to the panel but sets the GPU viewport from it, so
        // the projection has to use the clipped rect or the image squishes when the panel
        // is narrower than the content above it
        let rect = rect.intersect(ui.clip_rect());
        if rect.width() < 1.0 || rect.height() < 1.0 {
            return;
        }

        let camera = &mut self.state.camera;
        let shift = ui.input(|i| i.modifiers.shift);
        let panning = response.dragged_by(egui::PointerButton::Middle)
            || response.dragged_by(egui::PointerButton::Secondary)
            || (shift && response.dragged_by(egui::PointerButton::Primary));

        if response.dragged() && !panning {
            camera.yaw -= response.drag_delta().x * 0.01;
            camera.pitch = (camera.pitch + response.drag_delta().y * 0.01).clamp(-1.5, 1.5);
        }
        // aspect goes in the projection, so the whole panel is used
        let fov = 60f32.to_radians();
        let mut distance = camera.distance.unwrap_or(scene.radius * 2.5);

        if response.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                // the floor is absolute, so a small shape inside a sprawling scene
                // can still be approached
                distance = (distance * (1.0 - scroll * 0.002)).clamp(1e-3, scene.radius * 1000.0);
                camera.distance = Some(distance);
            }
        }
        let direction = Vec3::new(
            camera.yaw.cos() * camera.pitch.cos(),
            camera.yaw.sin() * camera.pitch.cos(),
            camera.pitch.sin(),
        );

        // pan across the view plane, at the rate the cursor moves over it
        let forward = -direction;
        let right = forward.cross(Vec3::Z).normalize_or_zero();
        let up = right.cross(forward).normalize_or_zero();
        if panning {
            let per_pixel = 2.0 * distance * (fov * 0.5).tan() / rect.height();
            let delta = response.drag_delta();
            camera.pan += (up * delta.y - right * delta.x) * per_pixel;
        }

        // NIF is Z-up; only the camera rig needs to know that
        let target = scene.center + camera.pan;
        let view = look_at_mat4(target + direction * distance, target, Vec3::Z);
        // near and far track the distance, so precision stays put as you close in
        let projection = perspective(
            fov,
            rect.width() / rect.height(),
            (distance * 0.01).max(1e-5),
            distance * 50.0,
        );
        let eye = target + direction * distance;
        let view_proj = projection * view;

        let mut uniform = [0f32; 24];
        uniform[..16].copy_from_slice(&view_proj.to_cols_array());
        uniform[16..19].copy_from_slice(&eye.to_array());
        uniform[20] = if self.state.colors { 1.0 } else { 0.0 };
        uniform[21] = if self.state.textures { 1.0 } else { 0.0 };
        gfx.render_state
            .queue
            .write_buffer(&gfx.camera_buffer, 0, bytemuck::cast_slice(&uniform));

        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            PreviewCall {
                scene,
                wireframe: self.state.wireframe,
                cull: self.state.cull,
                selected: self.state.selected,
                eye,
            },
        ));
    }
}

fn add_node(
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, usize>,
    blocks: &[Block],
    link: &Link,
    path: &mut HashSet<usize>,
    palette: &Palette,
) {
    let index = link.index;
    let label = label_for(blocks, index, link.slot.as_deref(), palette);
    let glyph = blocks.get(index).map(icon_for).unwrap_or(icon::CIRCLE);
    let children = blocks.get(index).map(linked).unwrap_or_default();

    if children.is_empty() || !path.insert(index) {
        builder.node(NodeBuilder::leaf(index).label(label).icon(move |ui| {
            ui.label(glyph);
        }));
        return;
    }

    builder.node(NodeBuilder::dir(index).label(label).icon(move |ui| {
        ui.label(glyph);
    }));
    for child in &children {
        add_node(builder, blocks, child, path, palette);
    }
    builder.close_dir();
    path.remove(&index);
}

impl eframe::App for Nifty {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        for path in ui.ctx().input(|i| {
            i.raw
                .dropped_files
                .iter()
                .filter_map(|f| f.path.clone())
                .collect::<Vec<_>>()
        }) {
            self.open(path);
        }
        if ui.ctx().input(|i| i.key_pressed(egui::Key::F)) {
            if let Some(document) = self.documents.get_mut(self.active) {
                document.focus_selected();
            }
        }

        let Nifty {
            documents,
            active,
            error,
            gfx,
        } = self;

        egui::Panel::top("bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                if documents.is_empty() {
                    ui.label("drop a .nif here");
                }

                let mut close = None;
                for (index, document) in documents.iter().enumerate() {
                    if ui
                        .selectable_label(index == *active, document.title())
                        .clicked()
                    {
                        *active = index;
                    }
                    if ui.small_button(icon::X).clicked() {
                        close = Some(index);
                    }
                    ui.separator();
                }
                if let Some(index) = close {
                    documents.remove(index);
                    *active = (*active).min(documents.len().saturating_sub(1));
                }

                if let Some(status) = documents.get(*active).and_then(|d| d.state.status.as_ref()) {
                    ui.label(status);
                }
                if let Some(error) = error {
                    ui.colored_label(egui::Color32::from_rgb(220, 120, 90), error.as_str());
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            let Some(document) = documents.get_mut(*active) else {
                ui.centered_and_justified(|ui| ui.label("no file open"));
                return;
            };
            DockArea::new(&mut document.dock)
                .id(egui::Id::new("dock").with(*active))
                .style(Style::from_egui(ui.style().as_ref()))
                .show_inside(
                    ui,
                    &mut Viewer {
                        state: &mut document.state,
                        gfx: gfx.as_ref(),
                    },
                );
        });
    }
}
