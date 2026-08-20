use std::{collections::HashSet, io::Cursor, path::PathBuf, sync::Arc};

use eframe::egui::{self, text::LayoutJob, Color32, FontId, TextFormat, WidgetText};
use eframe::egui_wgpu;
use egui_dock::{DockArea, DockState, Style, TabViewer};
use egui_ltreeview::{Action, NodeBuilder, TreeView, TreeViewState};
use egui_phosphor::regular as icon;
use nif::glam::camera::rh::{proj::directx::perspective, view::look_at_mat4};
use nif::glam::Vec3;
use nif::{blocks::Block, Nif};

use crate::details::{self, Details};
use crate::library::TextureLibrary;
use crate::pick;
use crate::scene::{Camera, Gfx, LodMode, PreviewCall, Scene};

struct Loaded {
    path: PathBuf,
    nif: Nif,
    links: Vec<Vec<Link>>,
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
    lod_mode: LodMode,
    /// Used by LodMode::Manual, in the file's own units.
    lod_distance: f32,
    /// Where the last pick happened, so clicking the same spot cycles through what is behind.
    last_pick: Option<egui::Pos2>,
    /// Set when the selection changed outside the tree, so the tree can catch up.
    sync_tree: bool,
    /// Nodes to open or close before the tree is next drawn.
    openness: Vec<(usize, bool)>,
    /// Decoded images for the details pane, keyed by block.
    previews: Details,
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
    let [_, right] =
        dock.main_surface_mut()
            .split_right(egui_dock::NodeIndex::root(), 0.3, vec![Tab::Preview]);
    dock.main_surface_mut()
        .split_below(right, 0.7, vec![Tab::Details]);
    dock
}

pub struct Nifty {
    documents: Vec<Document>,
    active: usize,
    error: Option<String>,
    gfx: Option<Gfx>,
    /// Shared across documents: where to look for the textures NIFs reference by name.
    library: TextureLibrary,
    show_library: bool,
    root_input: String,
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
            lod_mode: LodMode::Auto,
            lod_distance: 0.0,
            last_pick: None,
            sync_tree: false,
            openness: Vec::new(),
            previews: Details::default(),
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
            library: TextureLibrary::default(),
            show_library: false,
            root_input: String::new(),
        }
    }

    /// Selects the first open document, so opening several lands on the one named first.
    pub fn focus_first(&mut self) {
        self.active = 0;
    }

    /// Adds a directory to search for the textures NIFs name.
    pub fn add_texture_root(&mut self, root: PathBuf) {
        if self.library.add_root(root) {
            self.rebuild_scenes();
        }
    }

    /// Re-resolves every open document's textures, for when the roots change.
    fn rebuild_scenes(&mut self) {
        let Some(gfx) = &self.gfx else {
            return;
        };
        for document in &mut self.documents {
            let Some(loaded) = &document.state.loaded else {
                continue;
            };
            document.state.scene = Some(Arc::new(gfx.build_scene(&loaded.nif, &self.library)));
            document.state.previews.clear();
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
                let scene = gfx.build_scene(&nif, &self.library);
                let shapes = scene.meshes.len();
                state.scene = Some(Arc::new(scene));
                format!("{} blocks, {} shapes", nif.blocks.len(), shapes)
            }
            None => format!("{} blocks", nif.blocks.len()),
        });
        state.loaded = Some(Loaded {
            path,
            links: link_table(&nif.blocks),
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
    // source textures are usually unnamed, so fall back to the file path
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

/// The generic ref lists read better without a label; every other field names itself.
const UNLABELLED: [&str; 3] = ["child_refs", "property_refs", "extra_data_refs"];

/// Every block a block points at, found by reflection rather than by naming each field. Walking
/// 103 types by hand is what kept losing refs: the texture chain, data_ref and controller_ref
/// were each invisible in the tree until someone noticed and added an arm.
fn linked(block: &Block) -> Vec<Link> {
    nif::reflect::refs(block)
        .into_iter()
        .filter_map(|found| {
            let slot = found
                .path
                .last()
                .filter(|name| !UNLABELLED.contains(&name.as_str()))
                .cloned();
            Some(Link {
                index: found.reference.index()?,
                slot,
            })
        })
        .collect()
}

/// Precomputed per file, since the tree asks for a block's links on every frame.
fn link_table(blocks: &[Block]) -> Vec<Vec<Link>> {
    blocks.iter().map(linked).collect()
}

struct Viewer<'a> {
    state: &'a mut State,
    gfx: Option<&'a Gfx>,
    library: &'a TextureLibrary,
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
                let roots: Vec<usize> = loaded.nif.roots().map(|(index, _)| index).collect();
                let tree = Tree {
                    blocks,
                    links: &loaded.links,
                    palette: &palette,
                };

                let mut all = None;
                ui.horizontal(|ui| {
                    if ui.button("expand all").clicked() {
                        all = Some(true);
                    }
                    if ui.button("collapse all").clicked() {
                        all = Some(false);
                    }
                });
                if let Some(open) = all {
                    self.state.openness = (0..blocks.len()).map(|index| (index, open)).collect();
                }

                // Openness and scroll are both settled before the tree draws. The offset is a
                // ScrollArea argument, so one measured from the laid out rows would always be a
                // pass behind the openness that changed them.
                let tree_id = ui.make_persistent_id("hierarchy");
                let openness = std::mem::take(&mut self.state.openness);
                let mut offset = None;
                if self.state.sync_tree || !openness.is_empty() {
                    let mut state = TreeViewState::<usize>::load(ui, tree_id).unwrap_or_default();
                    for (index, open) in &openness {
                        state.set_openness(*index, *open);
                    }
                    // a pick in the preview has to open the tree down to the block it landed on
                    if self.state.sync_tree {
                        state.set_selected(self.state.selected.into_iter().collect());
                        if let Some(target) = self.state.selected {
                            for ancestor in ancestors_of(&loaded.links, &roots, target) {
                                state.set_openness(ancestor, true);
                            }
                            let height = row_height(ui);
                            let viewport = ui.available_height();
                            offset = row_of(&loaded.links, &roots, &state, target)
                                .map(|row| ((row as f32 + 0.5) * height - viewport * 0.5).max(0.0));
                        }
                    }
                    state.store(ui, tree_id);
                }

                let mut requested = Vec::new();
                let mut area = egui::ScrollArea::both().auto_shrink(false);
                if let Some(offset) = offset {
                    area = area.vertical_scroll_offset(offset);
                }
                area.show(ui, |ui| {
                    let (_, actions) = TreeView::new(tree_id).show(ui, |builder| {
                        let mut path = HashSet::new();
                        for &index in &roots {
                            add_node(
                                builder,
                                &tree,
                                &Link::plain(index),
                                &mut path,
                                0,
                                &mut requested,
                            );
                        }
                    });
                    for action in actions {
                        if let Action::SetSelected(selected) = action {
                            clicked = selected.first().copied();
                        }
                    }
                });
                if !requested.is_empty() {
                    // a context menu names one node; the request covers its whole subtree
                    for (index, open) in requested {
                        for node in subtree_of(&loaded.links, index) {
                            self.state.openness.push((node, open));
                        }
                    }
                    discard(ui.ctx(), "hierarchy openness");
                }
                self.state.sync_tree = false;
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
            Tab::Details => match self.state.selected {
                None => {
                    ui.centered_and_justified(|ui| ui.label("no block selected"));
                }
                Some(index) => {
                    // a link in the value column selects the block it points at
                    if let Some(target) = details::show(
                        ui,
                        &mut self.state.previews,
                        &loaded.nif,
                        self.library,
                        index,
                    ) {
                        self.state.selected = Some(target);
                        self.state.sync_tree = true;
                    }
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

        if !scene.lods.is_empty() {
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("{} LOD nodes", scene.lods.len()));
                ui.selectable_value(&mut self.state.lod_mode, LodMode::Auto, "by camera");
                ui.selectable_value(&mut self.state.lod_mode, LodMode::Manual, "by distance");
                ui.selectable_value(&mut self.state.lod_mode, LodMode::All, "all levels");

                match self.state.lod_mode {
                    LodMode::Manual => {
                        // every node sits at its least detailed level past the last switch,
                        // so a slider that went further would do nothing
                        let far = scene
                            .lods
                            .values()
                            .map(|lod| lod.last_switch())
                            .filter(|distance| distance.is_finite())
                            .fold(scene.radius * 4.0, f32::max);
                        ui.add(
                            egui::Slider::new(&mut self.state.lod_distance, 0.0..=far)
                                .text("distance"),
                        );
                        let levels: Vec<String> = scene
                            .lods
                            .values()
                            .map(|lod| lod.level_at(self.state.lod_distance).to_string())
                            .collect();
                        ui.label(format!("level {}", levels.join(", ")));
                    }
                    LodMode::Auto => {
                        ui.weak("level follows the camera");
                    }
                    LodMode::All => {
                        let levels = scene
                            .lods
                            .values()
                            .map(|lod| lod.ranges.len())
                            .max()
                            .unwrap_or(0);
                        ui.weak(format!("up to {levels} levels drawn over each other"));
                    }
                }
            });
        }

        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());

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

        // clicking the same spot again selects the next hit behind the current one
        if response.clicked() {
            if let (Some(pointer), Some(loaded)) =
                (response.interact_pointer_pos(), &self.state.loaded)
            {
                // only what is drawn can be picked, so a hidden LOD level is not selectable
                let visible =
                    scene.visible_shapes(self.state.lod_mode, self.state.lod_distance, eye);
                let hits = pick::ray_through(view_proj, rect, pointer)
                    .map(|ray| pick::hits(&loaded.nif, &ray, &visible))
                    .unwrap_or_default();
                let repeat = self
                    .state
                    .last_pick
                    .is_some_and(|last| last.distance(pointer) < 4.0);
                let next = match (repeat, self.state.selected) {
                    (true, Some(current)) => hits
                        .iter()
                        .position(|hit| hit.block == current)
                        .map(|at| hits[(at + 1) % hits.len()].block),
                    _ => None,
                };
                self.state.selected = next.or_else(|| hits.first().map(|hit| hit.block));
                self.state.last_pick = Some(pointer);
                self.state.sync_tree = true;
                // the hierarchy draws before the preview, so it has already missed this pick
                discard(ui.ctx(), "pick");
            }
        }

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
                lod_mode: self.state.lod_mode,
                lod_distance: self.state.lod_distance,
                scene,
                wireframe: self.state.wireframe,
                cull: self.state.cull,
                selected: self.state.selected,
                eye,
            },
        ));
    }
}

/// The chain of nodes from a root down to `target`, following the same links the tree draws.
fn ancestors_of(links: &[Vec<Link>], roots: &[usize], target: usize) -> Vec<usize> {
    fn descend(
        links: &[Vec<Link>],
        index: usize,
        target: usize,
        path: &mut Vec<usize>,
        seen: &mut HashSet<usize>,
    ) -> bool {
        if index == target {
            return true;
        }
        if !seen.insert(index) {
            return false;
        }
        path.push(index);
        for child in links.get(index).into_iter().flatten() {
            if descend(links, child.index, target, path, seen) {
                return true;
            }
        }
        path.pop();
        seen.remove(&index);
        false
    }

    let mut path = Vec::new();
    let mut seen = HashSet::new();
    for &root in roots {
        if descend(links, root, target, &mut path, &mut seen) {
            break;
        }
        path.clear();
        seen.clear();
    }
    path
}

/// What the tree lays every row out at, from `interact_size` plus the spacing between rows.
fn row_height(ui: &egui::Ui) -> f32 {
    ui.spacing().interact_size.y + ui.spacing().item_spacing.y
}

/// Where `target` lands in the tree's row order, or None when a closed directory hides it.
///
/// This counts what `add_node` draws, so the two have to keep the same rules: a node with no
/// children is a leaf, a node already on the path is drawn as one, and only an open directory
/// contributes its children.
fn row_of(
    links: &[Vec<Link>],
    roots: &[usize],
    state: &TreeViewState<usize>,
    target: usize,
) -> Option<usize> {
    fn walk(
        links: &[Vec<Link>],
        state: &TreeViewState<usize>,
        index: usize,
        depth: usize,
        path: &mut HashSet<usize>,
        row: &mut usize,
        target: usize,
    ) -> Option<usize> {
        if index == target {
            return Some(*row);
        }
        *row += 1;

        let children = links.get(index).map(Vec::as_slice).unwrap_or_default();
        if children.is_empty() || !path.insert(index) {
            return None;
        }
        let mut found = None;
        if state.is_open(&index).unwrap_or(depth == 0) {
            for child in children {
                found = walk(links, state, child.index, depth + 1, path, row, target);
                if found.is_some() {
                    break;
                }
            }
        }
        path.remove(&index);
        found
    }

    let mut row = 0;
    let mut path = HashSet::new();
    roots
        .iter()
        .find_map(|&index| walk(links, state, index, 0, &mut path, &mut row, target))
}

/// What every node in the tree needs to draw itself.
struct Tree<'a> {
    blocks: &'a [Block],
    links: &'a [Vec<Link>],
    palette: &'a Palette,
}

/// Every node reachable from `index`, including itself.
fn subtree_of(links: &[Vec<Link>], index: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let mut stack = vec![index];
    while let Some(node) = stack.pop() {
        if !seen.insert(node) {
            continue;
        }
        out.push(node);
        for child in links.get(node).into_iter().flatten() {
            stack.push(child.index);
        }
    }
    out
}

/// Throw away the pass just laid out and run another, so the corrected one is what reaches the
/// screen. The repaint is not redundant: a discard can be declined, egui allows only one per
/// frame by default, and a pass that has still not settled would otherwise sit on screen until
/// the next input arrives.
fn discard(ctx: &egui::Context, reason: &'static str) {
    ctx.request_discard(reason);
    ctx.request_repaint();
}

fn add_node(
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, usize>,
    tree: &Tree<'_>,
    link: &Link,
    path: &mut HashSet<usize>,
    depth: usize,
    requested: &mut Vec<(usize, bool)>,
) {
    let index = link.index;
    let label = label_for(tree.blocks, index, link.slot.as_deref(), tree.palette);
    let glyph = tree.blocks.get(index).map(icon_for).unwrap_or(icon::CIRCLE);
    let children = tree.links.get(index).map(Vec::as_slice).unwrap_or_default();

    if children.is_empty() || !path.insert(index) {
        builder.node(NodeBuilder::leaf(index).label(label).icon(move |ui| {
            ui.label(glyph);
        }));
        return;
    }

    let mut menu = None;
    builder.node(
        NodeBuilder::dir(index)
            // the root opens so a new file is not a single closed row
            .default_open(depth == 0)
            .label(label)
            .icon(move |ui| {
                ui.label(glyph);
            })
            .context_menu(|ui| {
                if ui.button("expand all children").clicked() {
                    menu = Some(true);
                    ui.close();
                }
                if ui.button("collapse all children").clicked() {
                    menu = Some(false);
                    ui.close();
                }
            }),
    );
    if let Some(open) = menu {
        requested.push((index, open));
    }
    for child in children {
        add_node(builder, tree, child, path, depth + 1, requested);
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
            if path.is_dir() {
                if self.library.add_root(path) {
                    self.rebuild_scenes();
                }
            } else {
                self.open(path);
            }
        }
        if ui.ctx().input(|i| i.key_pressed(egui::Key::F)) {
            if let Some(document) = self.documents.get_mut(self.active) {
                document.focus_selected();
            }
        }

        if self.show_library {
            let mut library = std::mem::take(&mut self.library);
            let mut input = std::mem::take(&mut self.root_input);
            let mut open = true;
            let mut changed = false;

            egui::Window::new("texture directories")
                .open(&mut open)
                .default_width(560.0)
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "Textures are matched by file name only, ignoring the path, the \
                         extension and case. When the same name exists under several \
                         directories, the one highest in this list is used.",
                    );
                    ui.separator();

                    let mut remove = None;
                    let mut promote = None;
                    for (index, root) in library.roots().iter().enumerate() {
                        ui.horizontal(|ui| {
                            if ui.small_button(icon::X).clicked() {
                                remove = Some(index);
                            }
                            if ui
                                .add_enabled(index > 0, egui::Button::new(icon::ARROW_UP).small())
                                .clicked()
                            {
                                promote = Some(index);
                            }
                            ui.monospace(root.display().to_string());
                        });
                    }
                    if let Some(index) = remove {
                        library.remove_root(index);
                        changed = true;
                    }
                    if let Some(index) = promote {
                        library.promote_root(index);
                        changed = true;
                    }
                    if library.roots().is_empty() {
                        ui.weak("no directories yet - drop a folder onto the window, or paste one");
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        let entry = ui.text_edit_singleline(&mut input);
                        let submitted =
                            entry.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                        if (ui.button("add").clicked() || submitted) && !input.trim().is_empty() {
                            if library.add_root(PathBuf::from(input.trim())) {
                                changed = true;
                            }
                            input.clear();
                        }
                        if ui.button("rescan").clicked() {
                            library.rescan();
                            changed = true;
                        }
                    });

                    ui.separator();
                    ui.label(format!(
                        "{} textures indexed, {} name collisions",
                        library.indexed(),
                        library.collisions()
                    ));
                });

            self.library = library;
            self.root_input = input;
            self.show_library = open;
            if changed {
                self.rebuild_scenes();
            }
        }

        let Nifty {
            documents,
            active,
            error,
            gfx,
            library,
            show_library,
            root_input: _,
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

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let label = match library.roots().len() {
                        0 => "textures: none".to_string(),
                        n => format!("textures: {n} dirs, {} files", library.indexed()),
                    };
                    if ui.button(label).clicked() {
                        *show_library = true;
                    }
                });
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
                        library,
                    },
                );
        });
    }
}
