//! Field by field view of the selected block.
//!
//! The rows come from reflection, so every field of every block appears without anyone listing
//! them. A row's value is rendered from its type. Anything nifty worked out rather than read is
//! attached to the row it came from and dimmed.

use std::collections::{HashMap, HashSet};

use eframe::egui;
use egui_phosphor::regular as icon;
use facet_reflect::Peek;
use nif::blocks::{Block, LightMode, StencilDrawMode, VertMode};
use nif::common::{BlockRef, ByteColor4, Color3, Color4};
use nif::Nif;

use crate::library::TextureLibrary;
use crate::texture::decode_texture;

/// Structs small enough to read on one line rather than expand into their own section.
const INLINE: [&str; 6] = [
    "Vector3",
    "Matrix33",
    "Matrix22",
    "TexCoord",
    "Quaternion",
    "Tbc",
];

enum Value {
    Text(String),
    /// A list with nothing in it, which has nothing to expand.
    Empty,
    Link(Option<usize>),
    Colour([f32; 4]),
    Swatches(Vec<ByteColor4>),
    Bytes(usize),
}

/// Worked out rather than read. Shares a row with the field it came from.
enum Derived {
    Note(String),
    Image { block: usize, missing: String },
}

struct Row {
    name: String,
    value: Value,
    derived: Vec<Derived>,
    depth: usize,
}

enum Entry {
    Section {
        name: String,
        depth: usize,
    },
    /// A list header. Its elements follow only while it is expanded.
    Collection {
        name: String,
        depth: usize,
        len: usize,
        key: String,
    },
    Field(Row),
}

/// How far each nesting level shifts the name column.
const INDENT: f32 = 14.0;

#[derive(Default)]
pub struct Details {
    images: HashMap<usize, Option<egui::TextureHandle>>,
    /// Keys of the collections the user has opened. Everything starts collapsed.
    expanded: HashSet<String>,
}

impl Details {
    pub fn clear(&mut self) {
        self.images.clear();
        self.expanded.clear();
    }

    fn get(
        &mut self,
        ui: &egui::Ui,
        index: usize,
        decode: impl FnOnce() -> Option<(u32, u32, Vec<u8>)>,
    ) -> Option<&egui::TextureHandle> {
        self.images
            .entry(index)
            .or_insert_with(|| {
                let (width, height, rgba) = decode()?;
                let size = [width as usize, height as usize];
                let image = egui::ColorImage::from_rgba_unmultiplied(size, &rgba);
                Some(ui.ctx().load_texture(
                    format!("block-{index}"),
                    image,
                    egui::TextureOptions::LINEAR,
                ))
            })
            .as_ref()
    }
}

/// Returns a block index when a link in the value column was followed.
pub fn show(
    ui: &mut egui::Ui,
    state: &mut Details,
    nif: &Nif,
    library: &TextureLibrary,
    index: usize,
) -> Option<usize> {
    let Some(block) = nif.blocks.get(index) else {
        ui.centered_and_justified(|ui| ui.label("no block selected"));
        return None;
    };

    let rows = rows(block, nif, library, index, &state.expanded);
    let mut follow = None;
    let mut toggle = None;
    egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
        egui::Grid::new("fields")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                for entry in &rows {
                    match entry {
                        Entry::Section { name, depth } => {
                            ui.horizontal(|ui| {
                                ui.add_space(*depth as f32 * INDENT);
                                ui.label(egui::RichText::new(name).weak());
                            });
                            ui.separator();
                        }
                        Entry::Collection {
                            name,
                            depth,
                            len,
                            key,
                        } => {
                            let open = state.expanded.contains(key);
                            ui.horizontal(|ui| {
                                ui.add_space(*depth as f32 * INDENT);
                                let arrow = if open {
                                    icon::CARET_DOWN
                                } else {
                                    icon::CARET_RIGHT
                                };
                                if ui.button(format!("{arrow} {name}")).clicked() {
                                    toggle = Some(key.clone());
                                }
                            });
                            ui.weak(format!("[{len} items]"));
                        }
                        Entry::Field(row) => {
                            ui.horizontal(|ui| {
                                ui.add_space(row.depth as f32 * INDENT);
                                ui.label(&row.name);
                            });
                            ui.vertical(|ui| {
                                if let Some(target) = value(ui, nif, &row.value) {
                                    follow = Some(target);
                                }
                                for item in &row.derived {
                                    derived(ui, state, nif, library, item);
                                }
                            });
                        }
                    }
                    ui.end_row();
                }
            });
    });
    if let Some(key) = toggle {
        if !state.expanded.remove(&key) {
            state.expanded.insert(key);
        }
    }
    follow
}

fn value(ui: &mut egui::Ui, nif: &Nif, value: &Value) -> Option<usize> {
    match value {
        Value::Text(text) => {
            ui.monospace(text);
            None
        }
        Value::Empty => {
            ui.weak("[empty]");
            None
        }
        Value::Bytes(len) => {
            ui.weak(format!("[{len} bytes]"));
            None
        }
        Value::Link(None) => {
            ui.weak("none");
            None
        }
        Value::Link(Some(target)) => {
            let name = nif.blocks.get(*target).map_or("out of range", Block::name);
            ui.link(format!("{target}  {name}"))
                .clicked()
                .then_some(*target)
        }
        Value::Swatches(entries) => {
            swatches(ui, entries);
            None
        }
        Value::Colour(rgba) => {
            let [r, g, b, a] = *rgba;
            ui.horizontal(|ui| {
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
                ui.painter().rect_filled(
                    rect,
                    2.0,
                    egui::Color32::from_rgb(byte(r), byte(g), byte(b)),
                );
                ui.monospace(format!("{r:.3} {g:.3} {b:.3} {a:.3}"));
            });
            None
        }
    }
}

fn derived(
    ui: &mut egui::Ui,
    state: &mut Details,
    nif: &Nif,
    library: &TextureLibrary,
    item: &Derived,
) {
    match item {
        Derived::Note(note) => {
            ui.weak(note);
        }
        Derived::Image { block, missing } => match image(ui, state, nif, library, *block) {
            Some(handle) => {
                ui.add(
                    egui::Image::new(&handle)
                        .max_size(egui::vec2(256.0, 256.0))
                        .maintain_aspect_ratio(true),
                );
            }
            None => {
                ui.weak(missing.as_str());
            }
        },
    }
}

fn image(
    ui: &mut egui::Ui,
    state: &mut Details,
    nif: &Nif,
    library: &TextureLibrary,
    index: usize,
) -> Option<egui::TextureHandle> {
    let handle = match nif.blocks.get(index)? {
        Block::NiPixelData(pixels) => {
            let palette = match pixels.palette_ref.get(&nif.blocks) {
                Some(Block::NiPalette(palette)) => Some(palette),
                _ => None,
            };
            state.get(ui, index, || decode_texture(pixels, palette))
        }
        Block::NiSourceTexture(source) => {
            let requested = source.file_name.to_string_lossy().into_owned();
            state.get(ui, index, || library.load(&requested))
        }
        _ => None,
    };
    handle.cloned()
}

fn swatches(ui: &mut egui::Ui, entries: &[ByteColor4]) {
    let size = egui::vec2(12.0, 12.0);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(1.0, 1.0);
        for entry in entries {
            let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
            let colour = egui::Color32::from_rgba_unmultiplied(entry.r, entry.g, entry.b, entry.a);
            ui.painter().rect_filled(rect, 1.0, colour);
            response.on_hover_text(format!("{} {} {} {}", entry.r, entry.g, entry.b, entry.a));
        }
    });
}

struct Walk<'a> {
    block: &'a Block,
    nif: &'a Nif,
    library: &'a TextureLibrary,
    index: usize,
    expanded: &'a HashSet<String>,
    out: Vec<Entry>,
}

fn rows(
    block: &Block,
    nif: &Nif,
    library: &TextureLibrary,
    index: usize,
    expanded: &HashSet<String>,
) -> Vec<Entry> {
    let mut walk = Walk {
        block,
        nif,
        library,
        index,
        expanded,
        out: Vec::new(),
    };
    // Block is an enum wrapping the concrete type
    if let Ok(variant) = Peek::new(block).into_enum() {
        if let Ok(Some(inner)) = variant.field(0) {
            walk.structure(inner, 0, "");
        }
    }
    walk.out
}

impl Walk<'_> {
    fn structure(&mut self, peek: Peek<'_, '_>, depth: usize, path: &str) {
        let peek = unwrap_pointer(peek);
        let Ok(structure) = peek.into_struct() else {
            return;
        };
        let owner = peek.shape().type_identifier;
        let mut headed = false;
        for (position, field) in structure.ty().fields.iter().enumerate() {
            let Ok(value) = structure.field(position) else {
                continue;
            };
            // a base contributes its own fields under its own heading, ahead of this type's,
            // and at the same depth because the whole chain is one block's fields
            if field.name == "base" && is_struct(value) {
                self.structure(value, depth, path);
                continue;
            }
            if !headed {
                self.section(owner, depth);
                headed = true;
            }
            self.field(owner, field.name, value, depth, path);
        }
        if !headed {
            self.section(owner, depth);
        }
    }

    fn section(&mut self, name: &str, depth: usize) {
        self.out.push(Entry::Section {
            name: name.to_string(),
            depth,
        });
    }

    fn field(&mut self, owner: &str, name: &str, peek: Peek<'_, '_>, depth: usize, path: &str) {
        let peek = unwrap_pointer(peek);
        let here = format!("{path}/{name}");

        if let Ok(option) = peek.into_option() {
            match option.value() {
                Some(inner) => self.field(owner, name, inner, depth, path),
                None => self.push(owner, name, Value::Text("None".into()), depth),
            }
            return;
        }
        if let Some(value) = scalar(peek) {
            self.push(owner, name, value, depth);
            return;
        }
        if peek.into_list_like().is_ok() {
            self.list(owner, name, peek, depth, &here);
            return;
        }
        if is_struct(peek) && !INLINE.contains(&peek.shape().type_identifier) {
            self.section(name, depth);
            let inner_owner = peek.shape().type_identifier;
            if let Ok(structure) = peek.into_struct() {
                for (position, inner) in structure.ty().fields.iter().enumerate() {
                    if let Ok(value) = structure.field(position) {
                        self.field(inner_owner, inner.name, value, depth + 1, &here);
                    }
                }
            }
            return;
        }
        self.push(owner, name, Value::Text(format!("{peek:?}")), depth);
    }

    fn list(&mut self, owner: &str, name: &str, peek: Peek<'_, '_>, depth: usize, path: &str) {
        // these read better as one value than as a list of numbers
        if let Ok(bytes) = peek.get::<Vec<u8>>() {
            self.push(owner, name, Value::Bytes(bytes.len()), depth);
            return;
        }
        if let Ok(palette) = peek.get::<Vec<ByteColor4>>() {
            self.push(owner, name, Value::Swatches(palette.clone()), depth);
            return;
        }
        let Ok(list) = peek.into_list_like() else {
            return;
        };
        let len = list.len();
        if len == 0 {
            self.push(owner, name, Value::Empty, depth);
            return;
        }
        let key = format!("{}{path}", self.index);
        self.out.push(Entry::Collection {
            name: name.to_string(),
            depth,
            len,
            key: key.clone(),
        });
        if !self.expanded.contains(&key) {
            return;
        }
        // elements carry their index, so repeated sections are told apart
        for (position, item) in list.iter().enumerate() {
            let name = format!("{name}[{position}]");
            self.field(
                owner,
                &name,
                item,
                depth + 1,
                &format!("{path}[{position}]"),
            );
        }
    }

    fn push(&mut self, owner: &str, name: &str, value: Value, depth: usize) {
        self.out.push(Entry::Field(Row {
            name: name.to_string(),
            value,
            derived: self.derived(owner, name),
            depth,
        }));
    }

    /// What the crate's accessors make of a field, keyed by the struct that declares it.
    fn derived(&self, owner: &str, name: &str) -> Vec<Derived> {
        let block = self.block;
        match (owner, name) {
            ("NiAvObject", "flags") => {
                let hidden = block.av_object().is_some_and(|o| o.is_hidden());
                note(if hidden {
                    "hidden, this node and its children are skipped"
                } else {
                    "visible"
                })
            }
            ("NiAlphaProperty", "flags") => match block {
                Block::NiAlphaProperty(alpha) => vec![
                    Derived::Note(format!(
                        "blending {}, {:?} to {:?}",
                        if alpha.alpha_blend() { "on" } else { "off" },
                        alpha.source_blend_mode(),
                        alpha.destination_blend_mode()
                    )),
                    Derived::Note(if alpha.alpha_test() {
                        format!("test {:?} against threshold", alpha.test_func())
                    } else {
                        "test off".into()
                    }),
                ],
                _ => Vec::new(),
            },
            ("NiZBufferProperty", "flags") => match block {
                Block::NiZBufferProperty(z) => vec![Derived::Note(format!(
                    "test {}, write {}",
                    z.depth_test(),
                    z.depth_write()
                ))],
                _ => Vec::new(),
            },
            ("NiSpecularProperty", "flags") => match block {
                Block::NiSpecularProperty(p) => note(enabled(p.is_enabled())),
                _ => Vec::new(),
            },
            ("NiWireframeProperty", "flags") => match block {
                Block::NiWireframeProperty(p) => note(enabled(p.is_enabled())),
                _ => Vec::new(),
            },
            ("NiVertexColorProperty", "vertex_mode") => match block {
                Block::NiVertexColorProperty(p) => note(match p.vertex_mode {
                    VertMode::SourceIgnore => {
                        "ambient, diffuse and emissive come from the material"
                    }
                    VertMode::SourceEmissive => "emissive comes from the vertex colour",
                    VertMode::SourceAmbientDiffuse => {
                        "ambient and diffuse come from the vertex colour"
                    }
                    VertMode::Unknown(_) => "unrecognised",
                }),
                _ => Vec::new(),
            },
            ("NiVertexColorProperty", "lighting_mode") => match block {
                Block::NiVertexColorProperty(p) => note(match p.lighting_mode {
                    LightMode::Emissive => "no lights are applied",
                    LightMode::EmissiveAmbientDiffuse => "lights are applied",
                    LightMode::Unknown(_) => "unrecognised",
                }),
                _ => Vec::new(),
            },
            ("NiStencilProperty", "draw_mode") => match block {
                Block::NiStencilProperty(p) => note(match p.draw_mode {
                    StencilDrawMode::CcwOrBoth | StencilDrawMode::Ccw => "back faces culled",
                    StencilDrawMode::Cw => "front faces culled",
                    StencilDrawMode::Both => "nothing culled, drawn two sided",
                }),
                _ => Vec::new(),
            },
            ("NiTriStripsData", "points") => match block {
                Block::NiTriStripsData(data) => vec![Derived::Note(format!(
                    "{} triangles after dropping degenerates",
                    data.triangles().count()
                ))],
                _ => Vec::new(),
            },
            ("NiPixelData", "pixel_data") => vec![Derived::Image {
                block: self.index,
                missing: "cannot decode this format".into(),
            }],
            ("NiSourceTexture", "file_name") => match block {
                Block::NiSourceTexture(source) if source.use_external => {
                    let requested = source.file_name.to_string_lossy().into_owned();
                    vec![
                        Derived::Note(match self.library.resolve(&requested) {
                            Some(path) => format!("resolves to {}", path.display()),
                            None => "not found in any texture directory".into(),
                        }),
                        Derived::Image {
                            block: self.index,
                            missing: "add a texture directory containing this file".into(),
                        },
                    ]
                }
                _ => Vec::new(),
            },
            ("NiSourceTexture", "pixel_data_ref") => match block {
                Block::NiSourceTexture(source) => source
                    .pixel_data_ref
                    .index()
                    .filter(|target| self.nif.blocks.get(*target).is_some())
                    .map(|block| {
                        vec![Derived::Image {
                            block,
                            missing: "cannot decode this format".into(),
                        }]
                    })
                    .unwrap_or_default(),
                _ => Vec::new(),
            },
            _ => Vec::new(),
        }
    }
}

fn note(text: &str) -> Vec<Derived> {
    vec![Derived::Note(text.to_string())]
}

fn enabled(on: bool) -> &'static str {
    if on {
        "enabled"
    } else {
        "disabled"
    }
}

/// Types with a better rendering than debug formatting.
fn scalar(peek: Peek<'_, '_>) -> Option<Value> {
    if let Ok(reference) = peek.get::<BlockRef>() {
        return Some(Value::Link(reference.index()));
    }
    if let Ok(colour) = peek.get::<Color3>() {
        return Some(Value::Colour([colour.r, colour.g, colour.b, 1.0]));
    }
    if let Ok(colour) = peek.get::<Color4>() {
        return Some(Value::Colour([colour.r, colour.g, colour.b, colour.a]));
    }
    if let Ok(colour) = peek.get::<ByteColor4>() {
        let byte = |v: u8| f32::from(v) / 255.0;
        return Some(Value::Colour([
            byte(colour.r),
            byte(colour.g),
            byte(colour.b),
            byte(colour.a),
        ]));
    }
    if let Ok(string) = peek.get::<nif::blocks::NiString>() {
        return Some(Value::Text(string.to_string_lossy().into_owned()));
    }
    None
}

fn is_struct(peek: Peek<'_, '_>) -> bool {
    unwrap_pointer(peek).into_struct().is_ok()
}

fn unwrap_pointer<'m, 'f>(peek: Peek<'m, 'f>) -> Peek<'m, 'f> {
    match peek.into_pointer() {
        Ok(pointer) => pointer.borrow_inner().unwrap_or(peek),
        Err(_) => peek,
    }
}
