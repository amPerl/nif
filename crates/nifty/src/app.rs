use std::{
    collections::{HashMap, HashSet},
    io::Cursor,
    path::PathBuf,
    sync::Arc,
};

use eframe::egui::{self, text::LayoutJob, Color32, FontId, TextFormat, WidgetText};
use eframe::egui_wgpu;
use egui_dock::{DockArea, DockState, Style, TabViewer};
use egui_ltreeview::{Action, NodeBuilder, TreeView, TreeViewState};
use egui_phosphor::regular as icon;
use nif::glam::camera::rh::{proj::directx::perspective, view::look_at_mat4};
use nif::glam::{Mat4, Vec3};
use nif::{blocks::Block, Nif};
use nif_wgpu::library::TextureLibrary;
use nif_wgpu::pick;
use nif_wgpu::scene::{Camera, Frame, Gfx, Light, LodMode, PreviewCall, Scene, Viewpoint};
use nif_wgpu::shaders::Shaders;
use nif_wgpu::Viewport;

use crate::capture::Capture;
use crate::details::{self, Details};

struct Loaded {
    path: PathBuf,
    /// When the file was last written as of the read that produced this, which is what auto
    /// reload compares against.
    ///
    /// A read that fails leaves this alone, so the poll keeps retrying. That is deliberate on
    /// Windows, where a file being written is often locked outright: the retry picks it up the
    /// moment the writer lets go, where recording the time we saw would leave the viewer stale
    /// until the next save.
    written: Option<std::time::SystemTime>,
    nif: Nif,
    links: Vec<Vec<Link>>,
    consumed: usize,
    size: usize,
    /// What every controller in the file covers, so the timeline has a range.
    span: Option<(f32, f32)>,
    /// Billboards have to be re-oriented whenever the camera moves, animation or not.
    billboards: bool,
    /// Something in the file can be culled, so the walk has to run even when nothing moves.
    hideable: bool,
    /// Whether anything in the file is skinned, since a skinned shape is placed by its bones
    /// every frame rather than by the transform the scene was built with.
    skinned: bool,
    /// Whether playback should start over at the end, or hold what the file settles on.
    repeats: bool,
    /// Particle state, which has to outlive a scene rebuild and so cannot live in the scene.
    systems: Vec<nif::psys::System>,
}

struct State {
    loaded: Option<Loaded>,
    selected: Option<usize>,
    scene: Option<Arc<Scene>>,
    /// The frame the last draw built. Framing a shape happens outside the draw and still has to
    /// ask where that shape is now, which only a frame knows.
    last_frame: Arc<Frame>,
    /// The rectangle the preview last drew into, in points. A capture crops the window's own
    /// pixels to it, so it has to be the rectangle the draw used rather than the panel's.
    preview_rect: Option<egui::Rect>,
    camera: Camera,
    wireframe: bool,
    /// Draw from the camera the file carries rather than the pan and orbit rig. Ignored by a
    /// file that has none.
    scene_camera: bool,
    cull: bool,
    colors: bool,
    textures: bool,
    grid: bool,
    lod_mode: LodMode,
    /// Used by LodMode::Manual, in the file's own units.
    lod_distance: f32,
    /// Where the timeline sits, in the file's own seconds.
    time: f32,
    /// Draw the transport under the preview. The clock runs either way, so hiding it leaves an
    /// animation playing rather than stopping it.
    show_timeline: bool,
    /// Watch this file and re-read it when it changes on disk.
    auto_reload: bool,
    /// Let the clock run past the file's span instead of starting over at it.
    ///
    /// The span is the longest single controller. Where a shorter one does not divide it,
    /// wrapping there catches that one part way through its own cycle and it jumps. The engine
    /// wraps nothing: each controller runs on its own clock, and they coincide again only at a
    /// common multiple of their periods. This reproduces that.
    unbounded: bool,
    playing: bool,
    /// Where the last pick happened, so clicking the same spot cycles through what is behind.
    last_pick: Option<egui::Pos2>,
    /// Blocks hidden by hand from the tree, which hide whatever sits under them the way the
    /// file's own cull flag does. Separate from the flag the file carries, so hiding something
    /// here says nothing about what the file asked for.
    hidden: HashSet<usize>,
    /// Set when the selection changed outside the tree, so the tree can catch up.
    sync_tree: bool,
    /// Nodes to open or close before the tree is next drawn.
    openness: Vec<(usize, bool)>,
    /// Decoded images for the details pane, keyed by block.
    previews: Details,
    /// This document's own view. Two previews on screen at once are two of these, since one
    /// buffer between them would leave both drawing whichever was written last.
    camera_binding: Option<Arc<nif_wgpu::scene::CameraBinding>>,
    /// Technique names this file asked for that no shader could draw, so the viewer can say the
    /// render is wrong rather than quietly showing the fixed function stand in.
    unhandled: Vec<String>,
    /// Techniques drawn but not in full, which have to be said as plainly as the ones nothing
    /// draws at all.
    partial: Vec<String>,
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
    /// Names this document for as long as it is open. The dock reorders and reparents tabs, so
    /// a position in the tree is not a name, and the panes inside a document need an egui id
    /// that survives being dragged somewhere else.
    id: usize,
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
        frame_selected(&mut self.state);
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
    /// The open files, as tabs. Nested docks: each document is one tab here and lays its own
    /// panes out inside itself, so two files can be split side by side.
    documents: DockState<Document>,
    next_id: usize,
    error: Option<String>,
    gfx: Option<Gfx>,
    /// Shared across documents: where to look for the textures NIFs reference by name.
    library: TextureLibrary,
    show_library: bool,
    root_input: String,
    /// The viewer's own light, which stands in for a file that carries none of its own. 92
    /// corpus files do carry lights, and those replace this.
    light: Light,
    show_light: bool,
    /// Built in shaders, plus any a user supplied from a directory.
    shaders: Shaders,
    show_shaders: bool,
    /// A `--capture` run, which turns each document through a circle and then closes the window.
    capture: Option<Capture>,
    /// When the files were last checked, in the same seconds egui counts.
    polled: f64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            loaded: None,
            selected: None,
            scene: None,
            last_frame: Arc::default(),
            preview_rect: None,
            camera: Camera::default(),
            wireframe: false,
            scene_camera: false,
            cull: true,
            colors: true,
            textures: true,
            grid: true,
            lod_mode: LodMode::Auto,
            lod_distance: 0.0,
            time: 0.0,
            show_timeline: true,
            auto_reload: false,
            unbounded: false,
            playing: false,
            last_pick: None,
            hidden: HashSet::new(),
            sync_tree: false,
            openness: Vec::new(),
            previews: Details::default(),
            camera_binding: None,
            unhandled: Vec::new(),
            partial: Vec::new(),
        }
    }
}

impl Nifty {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // wgpu reports validation failures through `log`, and nothing here installs a logger.
        // Set here rather than in the renderer, since where the errors go is the application's
        // to choose.
        if let Some(render_state) = cc.wgpu_render_state.as_ref() {
            render_state
                .device
                .on_uncaptured_error(std::sync::Arc::new(|error| {
                    eprintln!("wgpu error: {error}");
                }));
        }
        Self {
            documents: DockState::new(Vec::new()),
            next_id: 0,
            error: None,
            gfx: cc.wgpu_render_state.as_ref().map(Gfx::from_render_state),
            library: TextureLibrary::default(),
            show_library: false,
            root_input: String::new(),
            light: Light::default(),
            show_light: false,
            shaders: Shaders::default(),
            show_shaders: false,
            capture: None,
            polled: 0.0,
        }
    }

    /// Runs the viewer as a capture rather than as a window to work in: every open document is
    /// turned through a full circle, the preview is saved at each step, and the window closes.
    pub fn capture(&mut self, request: crate::capture::Request) {
        self.capture = Some(Capture::new(request));
    }

    /// Selects the first open document, so opening several lands on the one named first.
    pub fn focus_first(&mut self) {
        let first = self.documents.iter_all_tabs().next().map(|(path, _)| path);
        if let Some(path) = first {
            let _ = self.documents.set_active_tab(path);
        }
    }

    /// The document the bar reports on and a reload or a frame key acts upon.
    fn focused(&mut self) -> Option<&mut Document> {
        front(&mut self.documents)
    }

    /// The document with this id, wherever the dock has since moved it to.
    fn document(&mut self, id: usize) -> Option<&mut Document> {
        self.documents
            .iter_all_tabs_mut()
            .map(|(_, tab)| tab)
            .find(|tab| tab.id == id)
    }

    /// How many files are open. A capture walks them in this order.
    fn count(&self) -> usize {
        self.documents.iter_all_tabs().count()
    }

    /// Adds a directory to search. One root serves both purposes: the texture library indexes
    /// the images in it and the shader registry indexes any `<TechniqueName>.wgsl`, so a folder
    /// of shaders and a folder of textures are told apart by what is in them rather than by a
    /// flag.
    pub fn add_root(&mut self, root: PathBuf) {
        let shaders = self.shaders.add_root(root.clone());
        let textures = self.library.add_root(root);
        if shaders || textures {
            self.rebuild_scenes();
        }
    }

    /// Re-resolves every open document's textures, for when the roots change.
    fn rebuild_scenes(&mut self) {
        let Some(gfx) = &self.gfx else {
            return;
        };
        for (_, document) in self.documents.iter_all_tabs_mut() {
            let Some(loaded) = &document.state.loaded else {
                continue;
            };
            let (scene, unhandled, partial) =
                gfx.build_scene(&loaded.nif, &self.library, &self.shaders);
            document.state.scene = Some(Arc::new(scene));
            document.state.unhandled = unhandled;
            document.state.partial = partial;
            document.state.previews.clear();
        }
    }

    /// Each file opens as its own document rather than replacing the current one.
    pub fn open(&mut self, path: PathBuf) {
        let Some(state) = self.load(path) else {
            return;
        };
        self.next_id += 1;
        self.documents.push_to_focused_leaf(Document {
            id: self.next_id,
            state,
            dock: default_dock(),
        });
    }

    /// Reads the document at `index` again from the path it came from, keeping where the camera
    /// is and where the timeline sits. Both are kept because a reload is for looking at an edit
    /// to the same file, and returning to the default view would hide what changed.
    pub fn reload(&mut self, id: usize) {
        let Some(path) = self
            .document(id)
            .and_then(|d| d.state.loaded.as_ref())
            .map(|loaded| loaded.path.clone())
        else {
            return;
        };
        let Some(mut state) = self.load(path) else {
            return;
        };
        let Some(document) = self.document(id) else {
            return;
        };
        let was = &document.state;
        state.camera = was.camera;
        // the file may have been edited into a different span, so the old time is kept only
        // where the new one still reaches it
        state.time = match state.loaded.as_ref().and_then(|l| l.span) {
            Some((start, end)) => was.time.clamp(start, end),
            None => was.time,
        };
        state.wireframe = was.wireframe;
        state.scene_camera = was.scene_camera;
        state.cull = was.cull;
        state.colors = was.colors;
        state.textures = was.textures;
        state.grid = was.grid;
        state.lod_mode = was.lod_mode;
        state.lod_distance = was.lod_distance;
        state.playing = was.playing;
        state.auto_reload = was.auto_reload;
        state.show_timeline = was.show_timeline;
        // The selection is a block index, and an edited file can mean a different block sits at
        // it. Restored only when the block there still has the same type and name, so a reload
        // never silently moves the selection to something else.
        let same = |a: &State, b: &State| {
            let at = a.selected?;
            (identity(a, at) == identity(b, at)).then_some(at)
        };
        if let Some(at) = same(was, &state) {
            state.selected = Some(at);
            // the tree opens the ancestors and scrolls to it, the way a pick does
            state.sync_tree = true;
        }
        // Hidden blocks are indices too, so each is kept only where the block at it is still
        // the same one. Dropping the rest is better than hiding something nobody chose.
        state.hidden = was
            .hidden
            .iter()
            .copied()
            .filter(|at| identity(was, *at) == identity(&state, *at))
            .collect();
        document.state = state;
    }

    /// Re-reads any open file whose last written time has moved. One `metadata` call per open
    /// document, which is cheap enough to run on a timer.
    fn poll_for_changes(&mut self) {
        let changed: Vec<usize> = self
            .documents
            .iter_all_tabs()
            .filter_map(|(_, document)| {
                if !document.state.auto_reload {
                    return None;
                }
                let loaded = document.state.loaded.as_ref()?;
                let now = written_at(&loaded.path);
                (now.is_some() && now != loaded.written).then_some(document.id)
            })
            .collect();
        for id in changed {
            self.reload(id);
        }
    }

    /// Reads and parses one file into a fresh state, or reports why it could not be.
    fn load(&mut self, path: PathBuf) -> Option<State> {
        self.error = None;

        let bytes = match std::fs::read(&path) {
            Ok(bytes) => bytes,
            Err(e) => {
                self.error = Some(e.to_string());
                return None;
            }
        };

        let mut reader = Cursor::new(&bytes);
        let nif = match Nif::parse(&mut reader) {
            Ok(nif) => nif,
            Err(e) => {
                self.error = Some(first_line(&e.to_string()));
                return None;
            }
        };

        let mut state = State::default();
        if let Some(gfx) = &self.gfx {
            let (scene, unhandled, partial) = gfx.build_scene(&nif, &self.library, &self.shaders);
            state.scene = Some(Arc::new(scene));
            state.camera_binding = Some(Arc::new(gfx.camera()));
            state.unhandled = unhandled;
            state.partial = partial;
        }
        state.time = nif::anim::span(&nif.blocks).map_or(0.0, |(start, _)| start);
        let mut systems = nif::psys::systems(&nif.blocks);
        place_emitters(&nif, &mut systems);
        state.loaded = Some(Loaded {
            written: written_at(&path),
            path,
            links: link_table(&nif.blocks),
            span: nif::anim::span(&nif.blocks),
            billboards: nif
                .blocks
                .iter()
                .any(|block| matches!(block, Block::NiBillboardNode(_))),
            repeats: nif::anim::repeats(&nif.blocks),
            systems,
            hideable: nif.blocks.iter().any(|block| {
                matches!(block, Block::NiVisController(_))
                    || block.av_object().is_some_and(|av| av.is_hidden())
            }),
            skinned: nif
                .blocks
                .iter()
                .any(|block| matches!(block, Block::NiSkinInstance(_))),
            nif,
            consumed: reader.position() as usize,
            size: bytes.len(),
        });

        Some(state)
    }
}

/// How often auto reload looks at the files. Short enough that a save in another program shows
/// up promptly, long enough that the check costs nothing.
const POLL_SECONDS: f64 = 0.5;

/// When a file was last written, or `None` where it cannot be read at all.
fn written_at(path: &std::path::Path) -> Option<std::time::SystemTime> {
    std::fs::metadata(path)
        .and_then(|meta| meta.modified())
        .ok()
}

/// What identifies a block well enough to tell whether a reload is looking at the same one:
/// its type, and its name where it has one.
fn identity(state: &State, index: usize) -> Option<(&'static str, String)> {
    let block = state.loaded.as_ref()?.nif.blocks.get(index)?;
    let name = block
        .object_net()
        .map(|named| named.name.to_string_lossy().into_owned())
        .unwrap_or_default();
    Some((block.name(), name))
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
    light: &'a Light,
}

/// How both docks are drawn.
///
/// egui_dock fills a tab bar with `extreme_bg_color`, which is what a text field is filled with
/// rather than what chrome is: nearly black under a dark theme and pure white under a light one,
/// against a panel that is neither. Part of the way from there to the panel's own fill sets the
/// bar off from what it labels without either standing out. Mixed rather than named, since the
/// two ends swap round between a light theme and a dark one and a fixed grey would only suit one.
///
/// Not half way: an inactive tab is filled with exactly that mix, and matching it would leave
/// the tabs indistinguishable from the bar they sit in. The bar stays past them, so they read
/// as raised out of it the way they did before.
///
/// Every tab body is outlined in the same colour the separator between two of them is drawn in,
/// so two panes side by side are divided by three lines that all look alike. Only the separator
/// does anything: it is what the pointer grabs to resize them. The other two go.
///
/// A body is padded by the window margin, which read as the inside of a box while the outline
/// was drawn around it. With the outline gone there is nothing for it to be inside of, so it
/// reads as the pane floating clear of its own edges instead. Enough is kept that the content
/// does not touch the separator it sits against.
fn dock_style(ui: &egui::Ui) -> Style {
    let mut style = Style::from_egui(ui.style().as_ref());
    style.tab_bar.bg_fill = ui
        .visuals()
        .panel_fill
        .lerp_to_gamma(ui.visuals().extreme_bg_color, 0.75);
    style.tab.tab_body.stroke = egui::Stroke::NONE;
    style.tab.tab_body.inner_margin = egui::Margin::same(2);
    style
}

/// The document in front: the one the dock has focused, or the first open where it has focused
/// none. A nested dock can leave the outer one with no focused leaf, and the bar reporting on
/// nothing while files are open reads as though none were.
fn front(documents: &mut DockState<Document>) -> Option<&mut Document> {
    let id = documents
        .find_active_focused()
        .map(|(_, tab)| tab.id)
        .or_else(|| documents.iter_all_tabs().next().map(|(_, tab)| tab.id))?;
    documents
        .iter_all_tabs_mut()
        .map(|(_, tab)| tab)
        .find(|tab| tab.id == id)
}

/// One open file as a tab of its own: the toolbar for it, and its panes laid out underneath.
struct Desk<'a> {
    gfx: Option<&'a Gfx>,
    library: &'a TextureLibrary,
    light: &'a Light,
}

impl Desk<'_> {
    /// What is true of one open file, and what is true of the viewer, on one row. Drawn inside
    /// the document it is about, so nothing here is reachable until a file is open.
    /// Whatever the file turned out not to be drawn faithfully as. Only drawn when there is
    /// something to say, so a file the viewer can draw in full gives its panes the whole height.
    fn warnings(&mut self, ui: &mut egui::Ui, document: &Document) -> bool {
        let unhandled = document.state.unhandled.as_slice();
        let partial = document.state.partial.as_slice();
        if unhandled.is_empty() && partial.is_empty() {
            return false;
        }
        // the dock below is flush with the document's edges, and this row would be too
        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(4, 0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // a technique nothing can draw renders as the fixed function stand in, which
                    // looks like an answer. Say so rather than let it pass for one.
                    if !unhandled.is_empty() {
                        ui.colored_label(
                            egui::Color32::from_rgb(230, 170, 70),
                            format!("unhandled shader: {}", unhandled.join(", ")),
                        )
                        .on_hover_text(
                            "drawn with the fixed function stand in, which is wrong for these",
                        );
                    }
                    if !partial.is_empty() {
                        ui.colored_label(
                            egui::Color32::from_rgb(200, 190, 120),
                            format!("partly drawn: {}", partial.join(", ")),
                        )
                        .on_hover_text("the technique draws, but not everything it asks for");
                    }
                });
            });
        true
    }
}

impl TabViewer for Desk<'_> {
    type Tab = Document;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title().into()
    }

    /// The document's own id rather than its title, since two files can share a name and the
    /// dock would then treat them as the same tab.
    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(("document", tab.id))
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        // the dock below draws no edge of its own, so without this whatever sits above it and
        // the tabs under it would share one unbroken surface
        if self.warnings(ui, tab) {
            ui.separator();
        }
        DockArea::new(&mut tab.dock)
            .id(egui::Id::new(("panes", tab.id)))
            // Collapsing a leaf hides its body and leaves the bar behind, which is not
            // something to want of a pane whose whole purpose is what is in it. Closing every
            // pane at once leaves a document with nothing in it and no way to ask for one back.
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .style(dock_style(ui))
            .show_inside(
                ui,
                &mut Viewer {
                    state: &mut tab.state,
                    gfx: self.gfx,
                    library: self.library,
                    light: self.light,
                },
            );
    }
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
                    hidden: &self.state.hidden,
                };

                let mut all = None;
                let mut reveal = false;
                let concealed = tree.hidden.len();
                ui.horizontal(|ui| {
                    if ui.button("expand all").clicked() {
                        all = Some(true);
                    }
                    if ui.button("collapse all").clicked() {
                        all = Some(false);
                    }
                    // Shown only when there is something to show, and it says how many. A node
                    // hidden inside a closed parent is otherwise nowhere on screen, so without
                    // this the only way back is to remember where it was.
                    if concealed > 0
                        && ui
                            .button(format!("{} show {concealed} hidden", icon::EYE))
                            .clicked()
                    {
                        reveal = true;
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
                let mut toggled = Vec::new();
                // Vertical only: a name wider than the pane is cut rather than scrolled to.
                // Sideways scrolling put the eye on each row past the right edge of what was on
                // screen, which is where it is least use.
                let mut area = egui::ScrollArea::vertical().auto_shrink(false);
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
                                &mut toggled,
                            );
                        }
                    });
                    for action in actions {
                        if let Action::SetSelected(selected) = action {
                            clicked = selected.first().copied();
                        }
                    }
                });
                if reveal {
                    self.state.hidden.clear();
                }
                for index in toggled {
                    if !self.state.hidden.insert(index) {
                        self.state.hidden.remove(&index);
                    }
                }
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
    /// Where the shapes are and which of them are culled, for the frame about to be drawn.
    /// Skipped entirely when the file holds nothing that moves or hides.
    fn frame(&mut self, viewpoint: Viewpoint) -> Arc<Frame> {
        let Some(loaded) = &self.state.loaded else {
            return Arc::default();
        };
        // a particle system is animation even when nothing else in the file moves, and a block
        // hidden by hand has to be resolved whether or not anything else in the file does
        if viewpoint.is_static()
            && !loaded.hideable
            && !loaded.skinned
            && loaded.systems.is_empty()
            && self.state.hidden.is_empty()
        {
            return Arc::default();
        }
        let mut frame = Frame::default();
        let mut concealed = Concealed::default();
        for visit in viewpoint.walk(&loaded.nif) {
            // every block, not only the shapes: a bone is a node the skinned shape does not
            // own, and placing one means reaching it here
            frame
                .poses
                .insert(visit.index, Mat4::from(&visit.transform));
            // asked of every block for the same reason: what was hidden by hand is usually a
            // node, and a node is not what gets drawn
            let by_hand = concealed.visit(visit.index, visit.depth, &self.state.hidden);
            if visit.block.av_object().is_none() {
                continue;
            }
            if visit.hidden || by_hand {
                frame.hidden.insert(visit.index);
            }
        }
        // A skinned shape is placed by its bones whether or not anything animates, so its
        // geometry is resolved every frame rather than only when the clock runs. The scene
        // built the resting pose into the buffer, and this replaces it once bones move.
        let scene = self.state.scene.iter();
        for mesh in scene.flat_map(|scene| scene.meshes.iter()) {
            if !mesh.skinned {
                continue;
            }
            let Some(geometry) = loaded.nif.blocks.get(mesh.shape_block).and_then(Block::geometry)
            else {
                continue;
            };
            let skinned = nif::skin::deform(&loaded.nif.blocks, geometry, |index| {
                frame.poses.get(&index).copied()
            });
            if let Some(skinned) = skinned {
                frame.deformed.insert(
                    mesh.shape_block,
                    nif_wgpu::scene::Deformed {
                        positions: skinned.positions,
                        normals: skinned.normals,
                    },
                );
            }
        }
        // an alpha controller hangs off the material rather than the shape, and one material
        // can be shared, so these are collected by material block
        if let Some(time) = viewpoint.time {
            for (index, block) in loaded.nif.blocks.iter().enumerate() {
                let Block::NiMaterialProperty(material) = block else {
                    continue;
                };
                if let Some(alpha) = nif::anim::alpha_at(&loaded.nif.blocks, material, time) {
                    // a quadratic track overshoots its keys, and files do drive alpha negative.
                    // The fixed function pipeline clamped the material colour, so clamp here.
                    frame.alpha.insert(index, alpha.clamp(0.0, 1.0));
                }
                // one channel of the material colour, clamped for the same reason
                if let Some((channel, value)) =
                    nif::anim::material_color_at(&loaded.nif.blocks, material, time)
                {
                    frame.material_color.insert(
                        index,
                        (
                            channel,
                            [
                                value.x.clamp(0.0, 1.0),
                                value.y.clamp(0.0, 1.0),
                                value.z.clamp(0.0, 1.0),
                                1.0,
                            ],
                        ),
                    );
                }
            }
            // a geometry morpher rewrites the shape's vertices rather than moving the shape,
            // so it is resolved per frame like a pose and handed to the renderer the same way
            let scene = self.state.scene.iter();
            for mesh in scene.flat_map(|scene| scene.meshes.iter()) {
                let Some(geometry) = loaded
                    .nif
                    .blocks
                    .get(mesh.shape_block)
                    .and_then(Block::geometry)
                else {
                    continue;
                };
                if let Some(positions) = nif::anim::morph_at(&loaded.nif.blocks, geometry, time) {
                    // bending a surface leaves its resting shading behind, so the normals are
                    // rebuilt from the moved vertices wherever the morpher asks for it
                    let normals =
                        nif::anim::morph_normals(&loaded.nif.blocks, geometry, &positions);
                    frame.deformed.insert(
                        mesh.shape_block,
                        nif_wgpu::scene::Deformed { positions, normals },
                    );
                }
            }
            // an attribute can be driven over time, and a shader reads it from the same model
            // uniform either way, so only the lane the controller names is replaced
            let scene = self.state.scene.iter();
            for mesh in scene.flat_map(|scene| scene.meshes.iter()) {
                let (names, resolved) = mesh.attributes;
                if names.iter().all(|name| name.is_empty()) {
                    continue;
                }
                let Some(geometry) = loaded
                    .nif
                    .blocks
                    .get(mesh.shape_block)
                    .and_then(Block::av_object)
                else {
                    continue;
                };
                let mut params = resolved;
                let mut driven = false;
                for (lane, name) in names.iter().enumerate() {
                    if name.is_empty() {
                        continue;
                    }
                    if let Some(value) =
                        nif::anim::float_extra_data_at(&loaded.nif.blocks, geometry, name, time)
                    {
                        params[lane] = value;
                        driven = true;
                    }
                }
                if driven {
                    frame.params.insert(mesh.shape_block, params);
                }
            }
            // a texture transform controller drives one member of one slot's transform, so a
            // property can be the target of several at once and they are resolved together.
            // Walked per shape rather than per property, since which slots a shape binds is its
            // shader's choice.
            // Both kinds, since a flip controller reaches a particle system's sprite exactly as
            // it reaches a shape's map, and a quarter of them target one.
            let shapes = self
                .state
                .scene
                .iter()
                .flat_map(|scene| scene.meshes.iter())
                .map(|mesh| {
                    (
                        mesh.shape_block,
                        mesh.texturing_block,
                        mesh.bound,
                        mesh.uv_pins,
                    )
                });
            let systems = self
                .state
                .scene
                .iter()
                .flat_map(|scene| scene.particles.iter())
                .map(|mesh| {
                    (
                        mesh.block,
                        mesh.texturing_block,
                        nif_wgpu::scene::DEFAULT_SLOTS,
                        [None; nif_wgpu::scene::BOUND_SLOTS],
                    )
                });
            for (shape_block, texturing_block, bound, uv_pins) in
                shapes.chain(systems).collect::<Vec<_>>()
            {
                let property = match texturing_block.and_then(|i| loaded.nif.blocks.get(i)) {
                    Some(Block::NiTexturingProperty(property)) => property,
                    _ => continue,
                };
                frame.uv.insert(
                    shape_block,
                    nif_wgpu::scene::slot_uv_rows(
                        &loaded.nif.blocks,
                        Some(property),
                        bound,
                        uv_pins,
                        time,
                    ),
                );
                // every slot a flip controller is ever seen to drive, not just the base one:
                // a property flipped on two at once is the common case
                let mut flipped = nif_wgpu::scene::FlipState::default();
                for slot in nif_wgpu::scene::FLIPPABLE {
                    let source =
                        nif::anim::flip_source_at(&loaded.nif.blocks, property, slot, time)
                            .and_then(|r| r.index());
                    if let Some(source) = source {
                        flipped.set(slot, source);
                    }
                }
                if let (false, Some(block)) = (flipped.is_empty(), texturing_block) {
                    frame.flip.insert(block, flipped);
                }
            }
        }

        // the simulation carries state, so it is advanced here and the result handed to the
        // renderer, which keeps the drawing side free of anything that has to persist
        if let Some(loaded) = self.state.loaded.as_mut() {
            let time = viewpoint.time.unwrap_or(0.0);
            for system in &mut loaded.systems {
                system.seek(&loaded.nif.blocks, time);
                frame
                    .particles
                    .insert(system.block, system.particles().to_vec());
            }
        }
        Arc::new(frame)
    }

    /// The transport. Returns where the timeline sits, or None when nothing animates.
    fn timeline(&mut self, ui: &mut egui::Ui) -> Option<f32> {
        let loaded = self.state.loaded.as_ref()?;
        let (start, end) = loaded.span?;

        let repeats = loaded.repeats;
        // animating forever has nothing to hold or wrap at, so a file that plays once is
        // left alone
        let unbounded = self.state.unbounded && repeats;
        if self.state.playing {
            // stable_dt rather than dt, so one slow frame does not jump the animation
            self.state.time += ui.input(|i| i.stable_dt).min(0.1);
            if self.state.time > end && !unbounded {
                // a file whose controllers all clamp is played once, and holds what it ends on.
                // Starting over would be a loop the file never asked for.
                if repeats {
                    self.state.time = start + (self.state.time - start) % (end - start).max(1e-6);
                } else {
                    self.state.time = end;
                    self.state.playing = false;
                }
            }
            ui.ctx().request_repaint();
        }

        if !self.state.show_timeline {
            return Some(self.state.time);
        }

        ui.horizontal_wrapped(|ui| {
            let label = if self.state.playing { "pause" } else { "play" };
            if ui.button(label).clicked() {
                self.state.playing = !self.state.playing;
            }
            if ui.button("stop").clicked() {
                self.state.playing = false;
                self.state.time = start;
            }
            // the slider grows with the clock when it animates forever, so the handle still
            // tracks the time instead of resting at the end
            let reach = match unbounded {
                true => end.max(self.state.time),
                false => end,
            };
            ui.add(
                egui::Slider::new(&mut self.state.time, start..=reach)
                    .text("seconds")
                    .drag_value_speed(0.01),
            );
            ui.add_enabled(
                repeats,
                egui::Checkbox::new(&mut self.state.unbounded, "animate forever"),
            )
            .on_hover_text(
                "let the clock run past the span instead of starting over at it, the way the \
                 game does. The span is the longest controller, and the shorter ones are caught \
                 part way through when it wraps",
            );
            ui.weak(format!(
                "{:.2} s span, {}",
                end - start,
                if repeats { "repeats" } else { "plays once" }
            ));
        });

        Some(self.state.time)
    }

    fn preview(&mut self, ui: &mut egui::Ui) {
        // an empty preview still fills a rectangle, and a capture photographs that rather than
        // waiting for a draw that is never going to come
        self.state.preview_rect = Some(ui.available_rect_before_wrap());
        // the camera comes with the scene: both are built when the file loads and neither is
        // any use without the other
        let (Some(scene), Some(gfx), Some(binding)) = (
            self.state.scene.clone(),
            self.gfx,
            self.state.camera_binding.clone(),
        ) else {
            ui.centered_and_justified(|ui| ui.label("nothing to draw"));
            return;
        };
        // a particle system draws without any stored geometry, so it counts as something to draw
        if scene.meshes.is_empty() && scene.particles.is_empty() {
            ui.centered_and_justified(|ui| ui.label("no drawable geometry"));
            return;
        }

        ui.horizontal_wrapped(|ui| {
            // Five toggles in a row is most of the width for something rarely touched. Held
            // open on a click, since these are usually changed a few at a time and a menu that
            // shut after each one would have to be opened five times.
            egui::containers::menu::MenuButton::from_button(
                egui::Button::new("visibility").right_text(icon::CARET_DOWN),
            )
            .config(
                egui::containers::menu::MenuConfig::new()
                    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside),
            )
            .ui(ui, |ui| {
                ui.checkbox(&mut self.state.wireframe, "wireframe");
                ui.checkbox(&mut self.state.cull, "cull backfaces");
                ui.checkbox(&mut self.state.colors, "material colours");
                ui.checkbox(&mut self.state.textures, "textures");
                ui.checkbox(&mut self.state.grid, "grid");
            });
            // Where the view is pointed and what it is pointed through, which are the same
            // question. Each of these settles it, so the menu shuts on a click.
            egui::containers::menu::MenuButton::from_button(
                egui::Button::new("camera").right_text(icon::CARET_DOWN),
            )
            .ui(ui, |ui| {
                if ui.button("reset view").clicked() {
                    self.state.camera = Camera::default();
                }
                if ui
                    .add(egui::Button::new("frame selected").shortcut_text("F"))
                    .clicked()
                {
                    frame_selected(self.state);
                }
                // a file carries at most one camera, so this is a choice between two rather
                // than a list. Left out entirely for a file that carries none.
                if scene.camera.is_some() {
                    ui.separator();
                    ui.selectable_value(&mut self.state.scene_camera, false, "orbit");
                    ui.selectable_value(&mut self.state.scene_camera, true, "scene camera");
                }
            });
            // the shape of the file's own camera, while the view is through it
            if let (Some(cam), true) = (scene.camera, self.state.scene_camera) {
                ui.label(format!(
                    "{:.0} deg at {:.2}:1",
                    cam.fov.to_degrees(),
                    cam.aspect
                ));
            }
            ui.checkbox(&mut self.state.show_timeline, "animation controls");
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

        let time = self.timeline(ui);

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
            camera.pitch = (camera.pitch + response.drag_delta().y * 0.01)
                .clamp(-Camera::PITCH_LIMIT, Camera::PITCH_LIMIT);
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
        let mut eye = target + direction * distance;

        // A file's own camera replaces the rig outright: where it sits, which way it points and
        // the shape of its frustum all come from the file. Its own near and far are kept rather
        // than fitted to the scene, since the point of looking through it is to see what the
        // game framed.
        let through = self.state.scene_camera.then_some(scene.camera).flatten();
        let posed = through.and_then(|cam| {
            let loaded = self.state.loaded.as_ref()?;
            let mut walk = match time {
                Some(at) => loaded.nif.walk().at_time(at),
                None => loaded.nif.walk(),
            };
            let world = walk
                .find(|visit| visit.index == cam.block)
                .map(|visit| Mat4::from(&visit.transform))?;
            Some((cam, cam.view(world)))
        });
        // the scene is drawn near zero, so the view is built there too. Bounds, LOD distances
        // and the pick ray all stay in the file's own space
        let view = match posed {
            Some((_, (at, forward, up))) => {
                eye = at;
                look_at_mat4(at - scene.origin, at + forward - scene.origin, up)
            }
            None => look_at_mat4(eye - scene.origin, target - scene.origin, Vec3::Z),
        };
        // the near plane tracks the distance so precision stays where the camera looks. The far
        // plane cannot: it has to clear the floor, which is sized to the scene. Both terms are
        // measured from the eye rather than from the world origin, and the scene term uses the
        // swept bounds so an animation carrying a shape outside its resting box is not clipped
        let reach = (eye.distance(scene.grid.center) + scene.grid.half)
            .max(eye.distance(scene.center) + scene.animated_radius)
            * 1.25;
        // the file's camera is drawn into a rectangle of its own shape, so nothing is stretched
        // into an aspect the game never used
        let rect = match posed {
            Some((cam, _)) => letterbox(rect, cam.aspect),
            None => rect,
        };
        let projection = match posed {
            Some((cam, _)) => perspective(cam.fov, cam.aspect, cam.near, cam.far),
            None => perspective(
                fov,
                rect.width() / rect.height(),
                (distance * 0.01).max(1e-5),
                reach,
            ),
        };
        let view_proj = projection * view;

        // the view matrix rows are the camera's own axes in world space, which is what a
        // billboard turns to match
        let billboards = self.state.loaded.as_ref().is_some_and(|l| l.billboards);
        let viewpoint = Viewpoint {
            time,
            camera: billboards.then(|| nif::billboard::Camera {
                location: eye.into(),
                right: view.row(0).truncate().into(),
                up: view.row(1).truncate().into(),
                direction: (-view.row(2).truncate()).into(),
            }),
        };
        let frame = self.frame(viewpoint);
        // kept so framing a shape, which happens outside the draw, asks the same frame the draw
        // used rather than the resting scene
        self.state.last_frame = frame.clone();

        // clicking the same spot again selects the next hit behind the current one
        if response.clicked() {
            if let (Some(pointer), Some(loaded)) =
                (response.interact_pointer_pos(), &self.state.loaded)
            {
                // only what is drawn can be picked, so a hidden LOD level is not selectable
                let mut visible = scene.visible_shapes(
                    self.state.lod_mode,
                    self.state.lod_distance,
                    eye,
                    &frame.poses,
                );
                visible.retain(|shape| !frame.hidden.contains(shape));
                let hits = pick::ray_through(
                    view_proj,
                    scene.origin,
                    Viewport::from(rect),
                    [pointer.x, pointer.y],
                )
                .map(|ray| {
                    // the same axes the renderer spans a quad with, so a particle is picked
                    // as the square it draws rather than a sphere around it
                    let axes = (view.row(0).truncate(), view.row(1).truncate());
                    pick::hits(&loaded.nif, &ray, &visible, viewpoint, &frame, axes)
                })
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

        // a file's own ambient replaces the viewer's, and its own lights replace the key light
        let mut light_now = *self.light;
        if let Some(ambient) = scene.ambient {
            light_now.ambient = ambient;
        }
        // A light can sit on a node something animates, and its dimmer can be driven too, so
        // the resting values only stand while the clock does. Resolved against the same walk
        // the shapes are posed by, or the light would lag the thing carrying it.
        let blocks = self.state.loaded.as_ref().map(|l| &l.nif.blocks);
        let lights_now = match (viewpoint.time, blocks) {
            (Some(time), Some(blocks)) if !scene.light_blocks.is_empty() => scene
                .light_blocks
                .iter()
                .zip(scene.lights.iter())
                .map(|(block, resting)| {
                    let Some(light) = blocks.get(*block) else {
                        return *resting;
                    };
                    let world = frame
                        .poses
                        .get(block)
                        .copied()
                        .unwrap_or_else(|| Mat4::from_translation(resting.position));
                    let dimmer = light
                        .av_object()
                        .and_then(|av| nif::anim::dimmer_at(blocks, av, time));
                    nif::light::resolve(light, world, dimmer).unwrap_or(*resting)
                })
                .collect(),
            _ => scene.lights.clone(),
        };
        // A light under something hidden stops lighting, the way geometry under it stops
        // drawing. Without this an eye on a light block would show and do nothing.
        let lights_now: Vec<nif::light::Lit> = match frame.hidden.is_empty() {
            true => lights_now,
            false => lights_now
                .into_iter()
                .zip(scene.light_blocks.iter())
                .filter(|(_, block)| !frame.hidden.contains(block))
                .map(|(lit, _)| lit)
                .collect(),
        };
        let uniform = nif_wgpu::scene::camera_uniform(
            view_proj,
            eye - scene.origin,
            self.state.colors,
            self.state.textures,
            &light_now,
            &lights_now,
            scene.origin,
        );
        gfx.write_camera(&binding, &uniform);

        self.state.preview_rect = Some(rect);
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            PreviewCall {
                camera: binding,
                frame,
                lod_mode: self.state.lod_mode,
                lod_distance: self.state.lod_distance,
                scene,
                wireframe: self.state.wireframe,
                grid: self.state.grid,
                cull: self.state.cull,
                selected: self.state.selected,
                eye,
                right: view.row(0).truncate(),
                up: view.row(1).truncate(),
            },
        ));
    }
}

/// The largest rectangle of the given shape that fits inside `within`, centred in it. The bars
/// left over are what the preview does not draw into.
fn letterbox(within: egui::Rect, aspect: f32) -> egui::Rect {
    if aspect <= 0.0 || within.width() <= 0.0 || within.height() <= 0.0 {
        return within;
    }
    let (w, h) = match within.width() / within.height() > aspect {
        // wider than it should be, so the height decides
        true => (within.height() * aspect, within.height()),
        false => (within.width(), within.width() / aspect),
    };
    egui::Rect::from_center_size(within.center(), egui::vec2(w, h))
}

#[cfg(test)]
mod concealed_tests {
    use super::Concealed;
    use std::collections::HashSet;

    /// One depth first walk, as (block, depth) pairs in the order the walk reaches them.
    fn walked(nodes: &[(usize, usize)], hidden: &[usize]) -> Vec<usize> {
        let hidden: HashSet<usize> = hidden.iter().copied().collect();
        let mut concealed = Concealed::default();
        nodes
            .iter()
            .filter(|(index, depth)| concealed.visit(*index, *depth, &hidden))
            .map(|(index, _)| *index)
            .collect()
    }

    /// 0 holds 1, which holds 2 and 3; 4 is 1's sibling and holds 5.
    const TREE: [(usize, usize); 6] = [(0, 0), (1, 1), (2, 2), (3, 2), (4, 1), (5, 2)];

    #[test]
    fn hiding_a_node_hides_what_is_under_it() {
        assert_eq!(walked(&TREE, &[1]), vec![1, 2, 3]);
    }

    #[test]
    fn a_subtree_ends_where_the_depth_returns() {
        // 4 and 5 are past 1's subtree, so hiding 1 leaves them alone
        let out = walked(&TREE, &[1]);
        assert!(!out.contains(&4) && !out.contains(&5));
    }

    #[test]
    fn two_hidden_siblings_each_hide_their_own() {
        assert_eq!(walked(&TREE, &[1, 4]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn a_hidden_node_inside_a_hidden_node_does_not_end_early() {
        // 2 sits inside 1, so leaving 2 must not reveal 3, which is still inside 1
        assert_eq!(walked(&TREE, &[1, 2]), vec![1, 2, 3]);
    }

    #[test]
    fn hiding_the_root_hides_the_file() {
        assert_eq!(walked(&TREE, &[0]), vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn hiding_nothing_hides_nothing() {
        assert!(walked(&TREE, &[]).is_empty());
    }
}

#[cfg(test)]
mod camera_tests {
    use super::letterbox;
    use eframe::egui;

    fn rect(w: f32, h: f32) -> egui::Rect {
        egui::Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(w, h))
    }

    /// The file's camera draws into a rectangle of its own shape, so what the game framed is not
    /// stretched into the panel's aspect. The bars land on whichever pair of sides is spare.
    #[test]
    fn a_letterbox_keeps_the_shape_and_fits_inside() {
        // a panel wider than the camera leaves bars at the sides
        let inner = letterbox(rect(400.0, 100.0), 1.0);
        assert!((inner.width() - inner.height()).abs() < 1e-3);
        assert!((inner.height() - 100.0).abs() < 1e-3, "height was given away");

        // and one taller than the camera leaves them above and below
        let inner = letterbox(rect(100.0, 400.0), 1.0);
        assert!((inner.width() - 100.0).abs() < 1e-3);

        // the common frustum here is about four to three, and it keeps that
        let inner = letterbox(rect(800.0, 800.0), 1.32);
        assert!((inner.width() / inner.height() - 1.32).abs() < 1e-3);
        assert!(inner.width() <= 800.0 && inner.height() <= 800.0);

        // always centred in what it was given, and never larger than it
        let outer = rect(640.0, 480.0);
        for aspect in [0.5f32, 1.0, 1.32, 2.5] {
            let inner = letterbox(outer, aspect);
            assert!((inner.center() - outer.center()).length() < 1e-3);
            assert!(inner.width() <= outer.width() + 1e-3);
            assert!(inner.height() <= outer.height() + 1e-3);
        }

        // a nonsense aspect gives the rectangle back rather than an empty one
        assert_eq!(letterbox(outer, 0.0), outer);
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
    hidden: &'a HashSet<usize>,
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

/// Points the camera at the selected shape, or back at the whole scene where nothing is
/// selected. Reached from the preview's own menu and from the key that does the same.
fn frame_selected(state: &mut State) {
    let Some(scene) = &state.scene else {
        return;
    };
    let mesh = state.selected.and_then(|index| {
        scene
            .meshes
            .iter()
            .find(|m| m.shape_block == index || m.data_block == index)
    });

    match mesh {
        Some(mesh) => {
            // where it is at the moment being drawn, not where the file leaves it
            state.camera.pan = state.last_frame.center_of(mesh) - scene.center;
            state.camera.distance = Some(mesh.radius * 2.5);
        }
        None => {
            state.camera.pan = Vec3::ZERO;
            state.camera.distance = None;
        }
    }
}

/// Whether a depth first walk is inside a block hidden by hand.
///
/// Hiding a node hides everything under it, which is what the file's own cull flag does. The
/// walk reaches a block before any of its children and never returns to a depth it has left,
/// so the depth of the block that was hidden is enough to say where its subtree ends.
#[derive(Default)]
struct Concealed {
    /// The depth of the outermost hidden block the walk has entered and not yet left.
    at: Option<usize>,
}

impl Concealed {
    /// Called once per visit, in walk order. Answers whether this block is hidden, by itself
    /// or by something above it.
    fn visit(&mut self, index: usize, depth: usize, hidden: &HashSet<usize>) -> bool {
        // left first, then entered: a hidden block can be the next sibling of a hidden block,
        // and testing in the other order would let the first one's depth swallow the second
        if self.at.is_some_and(|entered| depth <= entered) {
            self.at = None;
        }
        if self.at.is_none() && hidden.contains(&index) {
            self.at = Some(depth);
        }
        self.at.is_some()
    }
}

/// A node's label, and the eye that hides what the label names.
///
/// Only an `NiAVObject` can be hidden, so nothing else is offered one. The eye for a shown node
/// appears under the pointer and nowhere else: nearly every node is shown, and drawing an eye on
/// all of them would put a column of identical icons down the tree. A hidden node keeps its eye
/// whatever the pointer does, since that is the only thing saying it is hidden.
fn node_label(ui: &mut egui::Ui, label: &LayoutJob, hidden: bool, hideable: bool, flip: &mut bool) {
    // The scroll bar draws over the right edge of the row, so the eye is held clear of where it
    // lands. Taken from the style rather than fixed, since the bar is a different width when it
    // is a solid one, and asking for the allocated width instead returns nothing for a floating
    // bar: that kind allocates no space and draws over the content.
    let bar = {
        let scroll = &ui.spacing().scroll;
        scroll.bar_inner_margin + scroll.bar_width + scroll.bar_outer_margin
    };
    // The tree lays its rows out to the widest one it has ever been asked to hold and never
    // narrows again, so a row reaches past the right of what is on screen once the pane is made
    // smaller. Both the name and the eye are placed against the visible edge rather than against
    // the row, or the eye lands where it cannot be seen or clicked.
    let edge = ui.clip_rect().right() - bar;
    let eye = match hideable {
        true => ui.spacing().icon_width + ui.spacing().button_padding.x * 2.0,
        false => 0.0,
    };
    // A name with no room left for it is cut rather than drawn under the eye. The whole of it is
    // still readable on hover.
    let mut job = label.clone();
    job.wrap.max_width = (edge - eye - ui.cursor().min.x).max(0.0);
    job.wrap.max_rows = 1;
    job.wrap.overflow_character = Some('\u{2026}');
    let galley = ui.painter().layout_job(job);
    let cut = galley.elided;
    let response = ui.add(egui::Label::new(galley).selectable(false));
    if cut {
        response.on_hover_text(label.text.clone());
    }
    if !hideable {
        return;
    }
    // A row is a band across the whole tree, so the pointer's height decides which row it is
    // over and the tree's own clip rect decides whether it is over the tree at all. Testing the
    // label's rectangle instead would lose the pointer over the icon to the left of it.
    let row = ui.max_rect();
    let hovered = ui
        .ctx()
        .pointer_hover_pos()
        .is_some_and(|at| at.y >= row.top() && at.y <= row.bottom() && ui.clip_rect().contains(at));
    if !hidden && !hovered {
        return;
    }
    let (glyph, hint) = match hidden {
        true => (icon::EYE_SLASH, "show this and everything under it"),
        false => (icon::EYE, "hide this and everything under it"),
    };
    let at = egui::Rect::from_min_max(
        egui::pos2(edge - eye, row.top()),
        egui::pos2(edge, row.bottom()),
    );
    if ui
        .put(at, egui::Button::new(glyph).frame(false))
        .on_hover_text(hint)
        .clicked()
    {
        *flip = true;
    }
}

fn add_node(
    builder: &mut egui_ltreeview::TreeViewBuilder<'_, usize>,
    tree: &Tree<'_>,
    link: &Link,
    path: &mut HashSet<usize>,
    depth: usize,
    requested: &mut Vec<(usize, bool)>,
    toggled: &mut Vec<usize>,
) {
    let index = link.index;
    let label = label_for(tree.blocks, index, link.slot.as_deref(), tree.palette);
    let glyph = tree.blocks.get(index).map(icon_for).unwrap_or(icon::CIRCLE);
    let children = tree.links.get(index).map(Vec::as_slice).unwrap_or_default();
    let hideable = tree.blocks.get(index).and_then(Block::av_object).is_some();
    let hidden = tree.hidden.contains(&index);
    let mut flip = false;

    if children.is_empty() || !path.insert(index) {
        builder.node(
            NodeBuilder::leaf(index)
                .label_ui(|ui| node_label(ui, &label, hidden, hideable, &mut flip))
                .icon(move |ui| {
                    ui.label(glyph);
                }),
        );
        if flip {
            toggled.push(index);
        }
        return;
    }

    let mut menu = None;
    builder.node(
        NodeBuilder::dir(index)
            // the root opens so a new file is not a single closed row
            .default_open(depth == 0)
            .label_ui(|ui| node_label(ui, &label, hidden, hideable, &mut flip))
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
    if flip {
        toggled.push(index);
    }
    for child in children {
        add_node(builder, tree, child, path, depth + 1, requested, toggled);
    }
    builder.close_dir();
    path.remove(&index);
}

impl eframe::App for Nifty {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // A capture aims before anything draws, so the yaw it sets is the yaw that gets painted
        // and then photographed. Delivering first frees the camera to turn in the same frame the
        // previous shot arrives in.
        let open = self.count();
        if let Some(capture) = &mut self.capture {
            if capture.deliver(ui.ctx(), open) {
                capture.report();
                let failed = capture.failed();
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                self.capture = None;
                if failed {
                    std::process::exit(1);
                }
            } else if let Some(aim) = capture.aim() {
                let at = self
                    .documents
                    .iter_all_tabs()
                    .nth(aim.document)
                    .map(|(path, _)| path);
                if let Some(path) = at {
                    let _ = self.documents.set_active_tab(path);
                }
                if let Some((_, document)) = self.documents.iter_all_tabs_mut().nth(aim.document) {
                    if let Some(time) = aim.time {
                        document.state.time = time;
                    }
                    document.state.camera.yaw = aim.yaw.to_radians();
                    document.state.camera.pitch = aim
                        .pitch
                        .to_radians()
                        .clamp(-Camera::PITCH_LIMIT, Camera::PITCH_LIMIT);
                }
            }
        }
        for path in ui.ctx().input(|i| {
            i.raw
                .dropped_files
                .iter()
                .filter_map(|f| f.path.clone())
                .collect::<Vec<_>>()
        }) {
            if path.is_dir() {
                self.add_root(path);
            } else {
                self.open(path);
            }
        }
        // one timer for the lot, since a poll is one metadata call per file that wants one
        let watching = self
            .documents
            .iter_all_tabs()
            .any(|(_, document)| document.state.auto_reload);
        if watching {
            let now = ui.ctx().input(|i| i.time);
            if now - self.polled >= POLL_SECONDS {
                self.polled = now;
                self.poll_for_changes();
            }
            // egui only draws on input, so without this the poll would stop as soon as the
            // window stopped receiving any, which is when a file is being edited elsewhere
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_secs_f64(POLL_SECONDS));
        }
        if ui.ctx().input(|i| i.key_pressed(egui::Key::F)) {
            if let Some(document) = self.focused() {
                document.focus_selected();
            }
        }

        if self.show_shaders {
            let mut open = true;
            egui::Window::new("shaders")
                .open(&mut open)
                .default_width(460.0)
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "Shaders are WGSL fragments written against the contract in prelude.wgsl.                          Drop a directory on the window and any <TechniqueName>.wgsl in it is                          picked up, overriding a built in of the same name.",
                    );
                    ui.separator();
                    egui::Grid::new("shader list").striped(true).show(ui, |ui| {
                        for (name, origin) in self.shaders.names() {
                            ui.label(name);
                            match origin {
                                nif_wgpu::shaders::Origin::BuiltIn => {
                                    ui.weak("built in");
                                }
                                nif_wgpu::shaders::Origin::Directory(path) => {
                                    ui.label(path.display().to_string());
                                }
                            }
                            ui.end_row();
                        }
                    });
                    // a shader can name a texture the file never mentions, and more than one
                    // file of that name can be installed, so which one won has to be visible
                    let named: Vec<(&str, &str)> = self.shaders.named_textures().collect();
                    if !named.is_empty() {
                        ui.separator();
                        ui.label("textures named by a shader rather than by the file:");
                        egui::Grid::new("named textures").striped(true).show(ui, |ui| {
                            for (shader, file) in named {
                                ui.label(shader);
                                ui.label(file);
                                match self.library.resolve(file) {
                                    Some(path) => {
                                        ui.label(path.display().to_string());
                                    }
                                    None => {
                                        ui.colored_label(
                                            egui::Color32::from_rgb(230, 170, 70),
                                            "not found in any texture directory",
                                        );
                                    }
                                }
                                ui.end_row();
                            }
                        });
                    }

                    // an attribute is bound from extra data on the shape rather than from
                    // anything the file names, so which names a shader looks for is otherwise
                    // invisible
                    let attributes: Vec<(&str, &str, f32)> = self.shaders.attributes().collect();
                    if !attributes.is_empty() {
                        ui.separator();
                        ui.label("attributes a shape can supply as float extra data:");
                        egui::Grid::new("shader attributes").striped(true).show(
                            ui,
                            |ui| {
                                for (shader, attribute, default) in attributes {
                                    ui.label(shader);
                                    ui.label(attribute);
                                    ui.weak(format!("default {default}"));
                                    ui.end_row();
                                }
                            },
                        );
                    }

                    if self.shaders.roots().is_empty() {
                        ui.weak("no shader directories added");
                    } else {
                        ui.separator();
                        for root in self.shaders.roots() {
                            ui.weak(root.display().to_string());
                        }
                    }
                });
            self.show_shaders = open;
        }

        if self.show_light {
            let mut open = true;
            let mut light = self.light;
            egui::Window::new("light")
                .open(&mut open)
                .default_width(300.0)
                .show(ui.ctx(), |ui| {
                    ui.label("A NIF carries no scene lighting, so this light is the viewer's own.");
                    ui.label("Every shape reads it, including the custom shaders.");
                    ui.separator();

                    let colour = |ui: &mut egui::Ui, name: &str, value: &mut Vec3| {
                        ui.horizontal(|ui| {
                            let mut rgb = [value.x, value.y, value.z];
                            ui.label(format!("{name:<9}"));
                            if ui.color_edit_button_rgb(&mut rgb).changed() {
                                *value = Vec3::from(rgb);
                            }
                            let mut scale = value.max_element();
                            if ui
                                .add(egui::Slider::new(&mut scale, 0.0..=2.0).text("level"))
                                .changed()
                            {
                                let unit = value.normalize_or(Vec3::ONE.normalize());
                                *value = unit * (scale / unit.max_element());
                            }
                        });
                    };
                    colour(ui, "ambient", &mut light.ambient);
                    colour(ui, "diffuse", &mut light.diffuse);
                    colour(ui, "specular", &mut light.specular);

                    ui.separator();
                    // the direction of travel, so the readout matches the engine's convention
                    let mut azimuth = light.direction.y.atan2(light.direction.x).to_degrees();
                    let mut elevation = light.direction.z.asin().to_degrees();
                    let turned = ui
                        .add(egui::Slider::new(&mut azimuth, -180.0..=180.0).text("azimuth"))
                        .changed()
                        | ui.add(egui::Slider::new(&mut elevation, -89.0..=89.0).text("elevation"))
                            .changed();
                    if turned {
                        let (az, el) = (azimuth.to_radians(), elevation.to_radians());
                        light.direction =
                            Vec3::new(az.cos() * el.cos(), az.sin() * el.cos(), el.sin());
                    }
                    ui.add(egui::Slider::new(&mut light.fill, 0.0..=1.0).text("rim"));

                    ui.separator();
                    if ui.button("reset").clicked() {
                        light = Light::default();
                    }
                });
            self.light = light;
            self.show_light = open;
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

        // Drawn before the fields are taken apart, since opening a file needs the whole app.
        // A top panel has to be added before the central one either way.
        let mut chosen = Vec::new();
        let mut reload = None;
        egui::Panel::top("menu").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        ui.close();
                        chosen = rfd::FileDialog::new()
                            .add_filter("NIF", &["nif"])
                            .pick_files()
                            .unwrap_or_default();
                    }
                    ui.separator();
                    // Both act on the file in front, which is the one the dock last focused.
                    // With nothing open there is nothing for them to act on, so they are shown
                    // as unavailable rather than left out: a menu that changes shape is harder
                    // to learn than one that greys out.
                    match front(&mut self.documents) {
                        Some(document) => {
                            if ui
                                .button("Reload")
                                .on_hover_text(
                                    "read this file again, keeping the camera and the time",
                                )
                                .clicked()
                            {
                                reload = Some(document.id);
                                ui.close();
                            }
                            ui.checkbox(&mut document.state.auto_reload, "Auto reload")
                                .on_hover_text("re-read this file when it changes on disk");
                        }
                        None => {
                            ui.add_enabled(false, egui::Button::new("Reload"));
                            ui.add_enabled(false, egui::Checkbox::new(&mut false, "Auto reload"));
                        }
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                // These are the viewer's, not one file's: every document resolves against the
                // same texture roots, the same shaders and the same stand in light.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let label = match self.library.roots().len() {
                        0 => "textures: none".to_string(),
                        n => format!("textures: {n} dirs, {} files", self.library.indexed()),
                    };
                    if ui.button(label).clicked() {
                        self.show_library = true;
                    }
                    if ui.button("light").clicked() {
                        self.show_light = true;
                    }
                    if ui
                        .button(format!("shaders: {}", self.shaders.count()))
                        .clicked()
                    {
                        self.show_shaders = true;
                    }
                    // a failed read belongs to the viewer too, since the file it names never
                    // became a document to say it in
                    if let Some(error) = &self.error {
                        ui.colored_label(egui::Color32::from_rgb(220, 120, 90), error.as_str());
                    }
                });
            });
        });
        if let Some(id) = reload {
            self.reload(id);
        }
        for path in chosen {
            self.open(path);
        }

        // Only the fields the panels below need. The menu row and the settings windows are
        // drawn while the whole app is still reachable, so what they use is not taken apart here.
        let Nifty {
            documents,
            next_id: _,
            error: _,
            gfx,
            library,
            show_library: _,
            root_input: _,
            light,
            show_light: _,
            shaders: _,
            show_shaders: _,
            capture,
            polled: _,
        } = self;

        let panel = egui::Frame::central_panel(ui.style().as_ref()).inner_margin(0);
        egui::CentralPanel::default().frame(panel).show(ui, |ui| {
            // nothing open means nothing to act on, so the window says the one thing it can
            if documents.iter_all_tabs().next().is_none() {
                ui.centered_and_justified(|ui| ui.label("drop a .nif here"));
                return;
            }
            let mut desk = Desk {
                gfx: gfx.as_ref(),
                library,
                light,
            };
            let mut style = dock_style(ui);
            // A document's body holds a dock rather than content, so padding it only pushes the
            // inner tab bar off the outer one. What is drawn directly in it pads itself.
            style.tab.tab_body.inner_margin = egui::Margin::ZERO;
            DockArea::new(documents)
                .id(egui::Id::new("documents"))
                .show_leaf_collapse_buttons(false)
                .show_leaf_close_all_buttons(false)
                .style(style)
                .show_inside(ui, &mut desk);
        });

        // the preview has drawn by now, so both the pixels and the rectangle they are in exist
        if let (Some(capture), Some(document)) = (capture, front(documents)) {
            if let Some(rect) = document.state.preview_rect {
                let stem = document
                    .state
                    .loaded
                    .as_ref()
                    .and_then(|loaded| loaded.path.file_stem())
                    .map(|stem| stem.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "untitled".into());
                capture.shoot(ui.ctx(), &stem, rect);
            }
        }
    }
}

/// Hands each system the transforms its emitters place against.
///
/// An emitter places into the space of the object it names, not the system's, and in this corpus
/// every emitter names one. The simulation does not walk the graph, so the walk happens here and
/// the result is a matrix per named object taking it into its system's space.
fn place_emitters(nif: &Nif, systems: &mut [nif::psys::System]) {
    if systems.is_empty() {
        return;
    }
    let mut world: HashMap<usize, Mat4> = HashMap::new();
    for visit in nif.walk() {
        world.insert(visit.index, Mat4::from(&visit.transform));
    }
    for system in systems.iter_mut() {
        let world_space = matches!(
            nif.blocks.get(system.block),
            Some(Block::NiParticleSystem(psys)) if psys.world_space
        );
        let Some(into_system) = world
            .get(&system.block)
            .map(|pose| nif_wgpu::scene::particle_space(*pose, world_space).inverse())
        else {
            continue;
        };
        let spaces = nif::psys::System::emitter_objects(&nif.blocks, system.block)
            .into_iter()
            .filter_map(|object| Some((object, into_system * *world.get(&object)?)))
            .collect();
        system.place_against(spaces);
    }
}
