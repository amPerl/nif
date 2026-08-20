//! Field by field view of the selected block.
//!
//! Every row is a field of the block, in declaration order. A row's value is plain text unless
//! there is something better to show for it, in which case only that value cell changes. Types
//! with no row list yet fall back to debug formatting.

use std::collections::HashMap;

use eframe::egui;
use nif::blocks::{
    Block, BumpMap, LightMode, MaterialData, NiAvObject, NiGeometry, NiGeometryData, NiPalette,
    NiPixelData, StencilDrawMode, TexDesc, TexTransform, VertMode,
};
use nif::common::{ByteColor4, Color3};
use nif::Nif;

use crate::library::TextureLibrary;
use crate::texture::decode_texture;

/// What to draw in a row's value column. Every variant is the field's own data.
enum Value {
    Text(String),
    /// Another block, shown as its index and type.
    Link(Option<usize>),
    Swatches(Vec<ByteColor4>),
    /// A swatch beside the numbers it was drawn from.
    Colour([f32; 4]),
    /// A blob that must never be printed in full.
    Bytes(usize),
    /// An array too long to list, shown as its length and what it holds.
    Items(usize, &'static str),
}

/// Anything nifty worked out rather than read. It shares a row with the field it came from, so
/// there are no rows that do not correspond to a field, and it is dimmed to stay distinguishable.
enum Derived {
    Note(String),
    /// Pixels decoded from the named block.
    Image {
        block: usize,
        missing: String,
    },
}

struct Row {
    name: &'static str,
    value: Value,
    derived: Vec<Derived>,
}

/// A divider announcing where a base struct or a nested field's own fields begin. Type names are
/// CamelCase and field names are snake_case, which is enough to tell the two apart.
enum Entry {
    Section(&'static str),
    Field(Row),
}

fn text(name: &'static str, value: impl ToString) -> Entry {
    field(name, Value::Text(value.to_string()))
}

fn field(name: &'static str, value: Value) -> Entry {
    Entry::Field(Row {
        name,
        value,
        derived: Vec::new(),
    })
}

fn with(entry: Entry, derived: Vec<Derived>) -> Entry {
    match entry {
        Entry::Field(row) => Entry::Field(Row { derived, ..row }),
        other => other,
    }
}

/// The name, extra data and controller every NiObjectNET carries.
fn object_net(base: &nif::blocks::NiObjectNET) -> Vec<Entry> {
    let mut rows = vec![
        Entry::Section("NiObjectNET"),
        text("name", base.name.to_string_lossy()),
    ];
    for reference in &base.extra_data_refs {
        rows.push(field("extra_data_ref", Value::Link(reference.index())));
    }
    rows.push(field(
        "controller_ref",
        Value::Link(base.controller_ref.index()),
    ));
    rows
}

/// Decoded images keyed by the block they came from, so a texture is uploaded once.
#[derive(Default)]
pub struct Previews {
    images: HashMap<usize, Option<egui::TextureHandle>>,
}

impl Previews {
    pub fn clear(&mut self) {
        self.images.clear();
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
    previews: &mut Previews,
    nif: &Nif,
    library: &TextureLibrary,
    index: usize,
) -> Option<usize> {
    let Some(block) = nif.blocks.get(index) else {
        ui.centered_and_justified(|ui| ui.label("no block selected"));
        return None;
    };

    let Some(rows) = rows(block, nif, library, index) else {
        egui::ScrollArea::both()
            .auto_shrink(false)
            .show(ui, |ui| ui.monospace(fallback(block)));
        return None;
    };

    let mut follow = None;
    egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
        egui::Grid::new("fields")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                for entry in &rows {
                    match entry {
                        Entry::Section(name) => {
                            ui.label(egui::RichText::new(*name).weak());
                            ui.separator();
                        }
                        Entry::Field(row) => {
                            ui.label(row.name);
                            ui.vertical(|ui| {
                                if let Some(target) = value(ui, nif, &row.value) {
                                    follow = Some(target);
                                }
                                for item in &row.derived {
                                    derived(ui, previews, nif, library, item);
                                }
                            });
                        }
                    }
                    ui.end_row();
                }
            });
    });
    follow
}

/// Draws one value cell. Returns a block index when the user asked to follow a link.
fn value(ui: &mut egui::Ui, nif: &Nif, value: &Value) -> Option<usize> {
    match value {
        Value::Text(text) => {
            ui.monospace(text);
            None
        }
        Value::Bytes(len) => {
            ui.weak(format!("[{len} bytes]"));
            None
        }
        Value::Items(len, unit) => {
            ui.weak(format!("[{len} {unit}]"));
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
    previews: &mut Previews,
    nif: &Nif,
    library: &TextureLibrary,
    item: &Derived,
) {
    match item {
        Derived::Note(note) => {
            ui.weak(note);
        }
        Derived::Image { block, missing } => match image(ui, previews, nif, library, *block) {
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
    previews: &mut Previews,
    nif: &Nif,
    library: &TextureLibrary,
    index: usize,
) -> Option<egui::TextureHandle> {
    let block = nif.blocks.get(index)?;
    let handle = match block {
        Block::NiPixelData(pixels) => {
            let palette = match pixels.palette_ref.get(&nif.blocks) {
                Some(Block::NiPalette(palette)) => Some(palette),
                _ => None,
            };
            previews.get(ui, index, || decode_texture(pixels, palette))
        }
        Block::NiSourceTexture(source) => {
            let requested = source.file_name.to_string_lossy().into_owned();
            previews.get(ui, index, || library.load(&requested))
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

fn rows(block: &Block, nif: &Nif, library: &TextureLibrary, index: usize) -> Option<Vec<Entry>> {
    match block {
        Block::NiPixelData(pixels) => Some(pixel_data(pixels, index)),
        Block::NiPalette(palette) => Some(self::palette(palette)),
        Block::NiSourceTexture(_) => Some(source_texture(nif, library, index)),
        Block::NiMaterialProperty(material) => Some(material_property(material)),
        Block::NiTexturingProperty(texturing) => Some(texturing_property(texturing)),
        Block::NiTriShape(shape) => Some(geometry("NiTriShape", &shape.base)),
        Block::NiTriStrips(strips) => Some(geometry("NiTriStrips", &strips.base)),
        Block::NiTriShapeData(data) => {
            let mut rows = geometry_data(&data.base.base);
            rows.push(Entry::Section("NiTriBasedGeomData"));
            rows.push(text("num_triangles", data.base.num_triangles));
            rows.push(Entry::Section("NiTriShapeData"));
            rows.push(match &data.triangles {
                Some(triangles) => field("triangles", Value::Items(triangles.len(), "triangles")),
                None => text("triangles", "None"),
            });
            rows.push(field(
                "match_groups",
                Value::Items(data.match_groups.len(), "groups"),
            ));
            Some(rows)
        }
        Block::NiTriStripsData(data) => {
            let mut rows = geometry_data(&data.base.base);
            rows.push(Entry::Section("NiTriBasedGeomData"));
            rows.push(text("num_triangles", data.base.num_triangles));
            rows.push(Entry::Section("NiTriStripsData"));
            for length in &data.strip_lengths {
                rows.push(text("strip_length", length));
            }
            rows.push(match &data.points {
                Some(points) => with(
                    field("points", Value::Items(points.len(), "points")),
                    vec![Derived::Note(format!(
                        "{} triangles after dropping degenerates",
                        data.triangles().count()
                    ))],
                ),
                None => text("points", "None"),
            });
            Some(rows)
        }
        Block::NiAlphaProperty(alpha) => Some(alpha_property(alpha)),
        Block::NiZBufferProperty(z) => Some(z_buffer_property(z)),
        Block::NiStencilProperty(stencil) => Some(stencil_property(stencil)),
        Block::NiVertexColorProperty(colour) => Some(vertex_color_property(colour)),
        Block::NiSpecularProperty(specular) => Some(flag_property(
            "NiSpecularProperty",
            &specular.base,
            specular.flags,
            vec![enabled(specular.is_enabled())],
        )),
        Block::NiWireframeProperty(wireframe) => Some(flag_property(
            "NiWireframeProperty",
            &wireframe.base,
            wireframe.flags,
            vec![enabled(wireframe.is_enabled())],
        )),
        Block::NiDitherProperty(dither) => Some(enum_property(
            "NiDitherProperty",
            &dither.base,
            format!("{:?}", dither.flags),
        )),
        Block::NiShadeProperty(shade) => Some(enum_property(
            "NiShadeProperty",
            &shade.base,
            format!("{:?}", shade.flags),
        )),
        Block::NiStringExtraData(data) => Some(extra_data(
            "NiStringExtraData",
            &data.name,
            vec![text("value", data.value.to_string_lossy())],
        )),
        Block::NiIntegerExtraData(data) => Some(extra_data(
            "NiIntegerExtraData",
            &data.name,
            vec![text("value", data.value)],
        )),
        Block::NiBooleanExtraData(data) => Some(extra_data(
            "NiBooleanExtraData",
            &data.name,
            vec![text("value", data.value)],
        )),
        Block::NiFloatExtraData(data) => Some(extra_data(
            "NiFloatExtraData",
            &data.name,
            vec![text("value", data.value)],
        )),
        Block::NiColorExtraData(data) => Some(extra_data(
            "NiColorExtraData",
            &data.name,
            vec![field(
                "data",
                Value::Colour([data.data.r, data.data.g, data.data.b, data.data.a]),
            )],
        )),
        Block::NiFloatsExtraData(data) => Some(extra_data(
            "NiFloatsExtraData",
            &data.name,
            data.data.iter().map(|v| text("data", v)).collect(),
        )),
        Block::NiIntegersExtraData(data) => Some(extra_data(
            "NiIntegersExtraData",
            &data.name,
            data.data.iter().map(|v| text("data", v)).collect(),
        )),
        _ => None,
    }
}

/// Everything a NiAVObject carries, including the flag word whose low bit hides the subtree.
fn av_object(base: &NiAvObject) -> Vec<Entry> {
    let mut rows = object_net(&base.base);
    rows.push(Entry::Section("NiAVObject"));
    rows.push(with(
        text("flags", format!("{:#06x}", base.flags)),
        vec![Derived::Note(if base.is_hidden() {
            "hidden, this node and its children are skipped".into()
        } else {
            "visible".into()
        })],
    ));
    rows.push(text(
        "translation",
        format!(
            "{} {} {}",
            base.translation.x, base.translation.y, base.translation.z
        ),
    ));
    let m = &base.rotation.column_major;
    rows.push(text(
        "rotation",
        format!(
            "{} {} {} / {} {} {} / {} {} {}",
            m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8]
        ),
    ));
    rows.push(text("scale", base.scale));
    for reference in &base.property_refs {
        rows.push(field("property_ref", Value::Link(reference.index())));
    }
    rows.push(field(
        "collision_ref",
        Value::Link(base.collision_ref.index()),
    ));
    rows
}

fn geometry(kind: &'static str, base: &NiGeometry) -> Vec<Entry> {
    let mut rows = av_object(&base.base);
    rows.push(Entry::Section("NiGeometry"));
    rows.push(field("data_ref", Value::Link(base.data_ref.index())));
    rows.push(field(
        "skin_instance_ref",
        Value::Link(base.skin_instance_ref.index()),
    ));
    match &base.material_data {
        MaterialData::None => rows.push(text("material_data", "None")),
        MaterialData::Shader(shader) | MaterialData::Invalid { shader, .. } => {
            rows.push(match &base.material_data {
                MaterialData::Invalid { flag, .. } => {
                    text("material_data", format!("Invalid flag {flag}"))
                }
                _ => text("material_data", "Shader"),
            });
            rows.push(text("shader_name", shader.name.to_string_lossy()));
            rows.push(field(
                "shader_extra_data_ref",
                Value::Link(shader.extra_data_ref.index()),
            ));
        }
    }
    rows.push(Entry::Section(kind));
    rows
}

/// The vertex arrays are parallel and long, so they are summarised rather than listed.
fn geometry_data(data: &NiGeometryData) -> Vec<Entry> {
    let mut rows = vec![
        Entry::Section("NiGeometryData"),
        text("group_id", data.group_id),
        text("keep_flags", data.keep_flags),
        text("compress_flags", data.compress_flags),
    ];
    rows.push(match &data.vertices {
        Some(vertices) => field("vertices", Value::Items(vertices.len(), "vertices")),
        None => text("vertices", "None"),
    });
    rows.push(with(
        text("data_flags", format!("{:#06x}", data.data_flags)),
        vec![Derived::Note(format!(
            "{} uv sets, havok material {}, nbt method {}",
            data.uv_set_count(),
            data.havok_material(),
            data.nbt_method()
        ))],
    ));
    for (name, array) in [
        ("normals", &data.normals),
        ("tangents", &data.tangents),
        ("binormals", &data.binormals),
    ] {
        rows.push(match array {
            Some(values) => field(name, Value::Items(values.len(), "vectors")),
            None => text(name, "None"),
        });
    }
    rows.push(text(
        "center",
        format!("{} {} {}", data.center.x, data.center.y, data.center.z),
    ));
    rows.push(text("radius", data.radius));
    rows.push(match &data.vertex_colors {
        Some(colours) => field("vertex_colors", Value::Items(colours.len(), "colours")),
        None => text("vertex_colors", "None"),
    });
    for set in &data.uv_sets {
        rows.push(field("uv_set", Value::Items(set.uvs.len(), "coords")));
    }
    rows.push(text(
        "consistency_flags",
        format!("{:#06x}", data.consistency_flags),
    ));
    rows.push(field(
        "additional_data_ref",
        Value::Link(data.additional_data_ref.index()),
    ));
    rows
}

fn enabled(on: bool) -> Derived {
    Derived::Note(if on { "enabled" } else { "disabled" }.into())
}

/// A `flags` word shown as it is, with what the crate's accessors make of it alongside.
fn flag_property(
    kind: &'static str,
    base: &nif::blocks::NiObjectNET,
    flags: u16,
    decoded: Vec<Derived>,
) -> Vec<Entry> {
    let mut rows = object_net(base);
    rows.push(Entry::Section(kind));
    rows.push(with(text("flags", format!("{flags:#06x}")), decoded));
    rows
}

/// Some properties spend their flags word on a plain enum rather than on bits.
fn enum_property(kind: &'static str, base: &nif::blocks::NiObjectNET, flags: String) -> Vec<Entry> {
    let mut rows = object_net(base);
    rows.push(Entry::Section(kind));
    rows.push(text("flags", flags));
    rows
}

fn alpha_property(alpha: &nif::blocks::NiAlphaProperty) -> Vec<Entry> {
    let mut decoded = vec![Derived::Note(format!(
        "blending {}, {:?} to {:?}",
        if alpha.alpha_blend() { "on" } else { "off" },
        alpha.source_blend_mode(),
        alpha.destination_blend_mode()
    ))];
    decoded.push(Derived::Note(if alpha.alpha_test() {
        format!("test {:?} against threshold", alpha.test_func())
    } else {
        "test off".into()
    }));
    if alpha.no_sorter() {
        decoded.push(Derived::Note("no sorter".into()));
    }
    if alpha.clone_unique() {
        decoded.push(Derived::Note("clone unique".into()));
    }
    if alpha.editor_alpha_threshold() {
        decoded.push(Derived::Note("editor alpha threshold".into()));
    }

    let mut rows = flag_property("NiAlphaProperty", &alpha.base, alpha.flags, decoded);
    rows.push(text("threshold", alpha.threshold));
    rows
}

fn z_buffer_property(z: &nif::blocks::NiZBufferProperty) -> Vec<Entry> {
    let decoded = vec![Derived::Note(format!(
        "test {}, write {}",
        z.depth_test(),
        z.depth_write()
    ))];
    let mut rows = flag_property("NiZBufferProperty", &z.base, z.flags, decoded);
    rows.push(text("function", format!("{:?}", z.function)));
    rows
}

fn vertex_color_property(colour: &nif::blocks::NiVertexColorProperty) -> Vec<Entry> {
    let mut rows = object_net(&colour.base);
    rows.push(Entry::Section("NiVertexColorProperty"));
    rows.push(text("flags", format!("{:#06x}", colour.flags)));
    // these select which D3D material channel the vertex colour feeds, so they are worth spelling out
    rows.push(with(
        text("vertex_mode", format!("{:?}", colour.vertex_mode)),
        vec![Derived::Note(
            match colour.vertex_mode {
                VertMode::SourceIgnore => {
                    "ambient, diffuse and emissive all come from the material"
                }
                VertMode::SourceEmissive => "emissive comes from the vertex colour",
                VertMode::SourceAmbientDiffuse => "ambient and diffuse come from the vertex colour",
                VertMode::Unknown(_) => "unrecognised",
            }
            .into(),
        )],
    ));
    rows.push(with(
        text("lighting_mode", format!("{:?}", colour.lighting_mode)),
        vec![Derived::Note(
            match colour.lighting_mode {
                LightMode::Emissive => "no lights are applied",
                LightMode::EmissiveAmbientDiffuse => "lights are applied",
                LightMode::Unknown(_) => "unrecognised",
            }
            .into(),
        )],
    ));
    rows
}

fn stencil_property(stencil: &nif::blocks::NiStencilProperty) -> Vec<Entry> {
    let mut rows = object_net(&stencil.base);
    rows.push(Entry::Section("NiStencilProperty"));
    rows.push(text("stencil_enabled", stencil.stencil_enabled));
    rows.push(text(
        "stencil_function",
        format!("{:?}", stencil.stencil_function),
    ));
    rows.push(text("stencil_ref", stencil.stencil_ref));
    rows.push(text(
        "stencil_mask",
        format!("{:#010x}", stencil.stencil_mask),
    ));
    rows.push(text("fail_action", format!("{:?}", stencil.fail_action)));
    rows.push(text("zfail_action", format!("{:?}", stencil.zfail_action)));
    rows.push(text("pass_action", format!("{:?}", stencil.pass_action)));
    // this is what decides face culling, not the alpha property
    rows.push(with(
        text("draw_mode", format!("{:?}", stencil.draw_mode)),
        vec![Derived::Note(
            match stencil.draw_mode {
                StencilDrawMode::CcwOrBoth | StencilDrawMode::Ccw => "back faces culled",
                StencilDrawMode::Cw => "front faces culled",
                StencilDrawMode::Both => "nothing culled, drawn two sided",
            }
            .into(),
        )],
    ));
    rows
}

/// The extra data types are flat: a name, then whatever they carry. They share no base struct in
/// this crate, so the section is the type itself.
fn extra_data(kind: &'static str, name: &nif::blocks::NiString, values: Vec<Entry>) -> Vec<Entry> {
    let mut rows = vec![Entry::Section(kind), text("name", name.to_string_lossy())];
    rows.extend(values);
    rows
}

fn colour(name: &'static str, value: &Color3) -> Entry {
    field(name, Value::Colour([value.r, value.g, value.b, 1.0]))
}

fn material_property(material: &nif::blocks::NiMaterialProperty) -> Vec<Entry> {
    let mut rows = object_net(&material.base);
    rows.push(Entry::Section("NiMaterialProperty"));
    rows.push(colour("color_ambient", &material.color_ambient));
    rows.push(colour("color_diffuse", &material.color_diffuse));
    rows.push(colour("color_specular", &material.color_specular));
    rows.push(colour("color_emissive", &material.color_emissive));
    rows.push(text("glossiness", material.glossiness));
    rows.push(text("alpha", material.alpha));
    rows
}

/// One texture slot. The transform is an enum, so its own fields only exist when it is present.
fn tex_desc(name: &'static str, desc: &TexDesc, rows: &mut Vec<Entry>) {
    rows.push(Entry::Section(name));
    rows.push(field("source_ref", Value::Link(desc.source_ref.index())));
    rows.push(text("clamp_mode", format!("{:?}", desc.clamp_mode)));
    rows.push(text("filter_mode", format!("{:?}", desc.filter_mode)));
    rows.push(text("uv_set", desc.uv_set));
    match &desc.transform {
        TexTransform::None => rows.push(text("transform", "None")),
        TexTransform::Present(transform) | TexTransform::Invalid { transform, .. } => {
            if let TexTransform::Invalid { flag, .. } = &desc.transform {
                rows.push(text("transform", format!("Invalid flag {flag}")));
            } else {
                rows.push(text("transform", "Present"));
            }
            rows.push(text(
                "translation",
                format!("{} {}", transform.translation.u, transform.translation.v),
            ));
            rows.push(text(
                "tiling",
                format!("{} {}", transform.tiling.u, transform.tiling.v),
            ));
            rows.push(text("w_rotation", transform.w_rotation));
            rows.push(text("transform_type", transform.transform_type));
            rows.push(text(
                "center_offset",
                format!(
                    "{} {}",
                    transform.center_offset.u, transform.center_offset.v
                ),
            ));
        }
    }
}

fn texturing_property(texturing: &nif::blocks::NiTexturingProperty) -> Vec<Entry> {
    let mut rows = object_net(&texturing.base);
    rows.push(Entry::Section("NiTexturingProperty"));
    rows.push(text("apply_mode", format!("{:?}", texturing.apply_mode)));
    rows.push(text("texture_count", texturing.texture_count));

    let slots: [(&'static str, Option<&TexDesc>); 9] = [
        ("base_texture", texturing.base_texture.as_deref()),
        ("dark_texture", texturing.dark_texture.as_deref()),
        ("detail_texture", texturing.detail_texture.as_deref()),
        ("gloss_texture", texturing.gloss_texture.as_deref()),
        ("glow_texture", texturing.glow_texture.as_deref()),
        ("decal0_texture", texturing.decal0_texture.as_deref()),
        ("decal1_texture", texturing.decal1_texture.as_deref()),
        ("decal2_texture", texturing.decal2_texture.as_deref()),
        ("decal3_texture", texturing.decal3_texture.as_deref()),
    ];
    for (name, desc) in slots {
        match desc {
            Some(desc) => tex_desc(name, desc, &mut rows),
            None => rows.push(text(name, "None")),
        }
    }

    match &texturing.bump_map {
        None => rows.push(text("bump_map", "absent")),
        Some(BumpMap::None) => rows.push(text("bump_map", "None")),
        Some(bump) => {
            if let Some(data) = bump.get() {
                rows.push(text("bump_map", "Present"));
                tex_desc("bump_map.texture", &data.texture, &mut rows);
                rows.push(Entry::Section("bump_map"));
                rows.push(text("luma_scale", data.luma_scale));
                rows.push(text("luma_offset", data.luma_offset));
                rows.push(text(
                    "matrix",
                    format!(
                        "m11 {} m21 {} m12 {} m22 {}",
                        data.matrix.m11, data.matrix.m21, data.matrix.m12, data.matrix.m22
                    ),
                ));
            }
        }
    }

    for entry in &texturing.shader_textures {
        match entry.get() {
            None => rows.push(text("shader_texture", "None")),
            Some(map) => {
                tex_desc("shader_texture", &map.map, &mut rows);
                rows.push(text("map_id", map.map_id));
            }
        }
    }
    rows
}

fn pixel_data(pixels: &NiPixelData, index: usize) -> Vec<Entry> {
    let format = &pixels.base;
    let mut rows = vec![
        Entry::Section("NiPixelFormat"),
        text("pixel_format", format!("{:?}", format.pixel_format)),
        text("bits_per_pixel", format.bits_per_pixel),
        text("renderer_hint", format.renderer_hint),
        text("extra_data", format.extra_data),
        text("flags", format.flags),
        text("tiling", format.tiling),
    ];
    for (i, channel) in format.channels.iter().enumerate() {
        rows.push(text(
            "channel",
            format!(
                "{i}: kind {} convention {} bits {} signed {}",
                channel.kind, channel.convention, channel.bits_per_channel, channel.is_signed
            ),
        ));
    }
    rows.push(Entry::Section("NiPixelData"));
    rows.push(field(
        "palette_ref",
        Value::Link(pixels.palette_ref.index()),
    ));
    rows.push(text("bytes_per_pixel", pixels.bytes_per_pixel));
    for mip in &pixels.mipmaps {
        rows.push(text(
            "mipmap",
            format!("{} x {} at offset {}", mip.width, mip.height, mip.offset),
        ));
    }
    for (i, face) in pixels.pixel_data.iter().enumerate() {
        let row = field(
            if i == 0 { "pixel_data" } else { "face" },
            Value::Bytes(face.data.len()),
        );
        rows.push(if i == 0 {
            with(
                row,
                vec![Derived::Image {
                    block: index,
                    missing: "cannot decode this format".into(),
                }],
            )
        } else {
            row
        });
    }
    rows
}

fn palette(palette: &NiPalette) -> Vec<Entry> {
    vec![
        Entry::Section("NiPalette"),
        text("has_alpha", palette.has_alpha),
        field("palette", Value::Swatches(palette.palette.clone())),
    ]
}

fn source_texture(nif: &Nif, library: &TextureLibrary, index: usize) -> Vec<Entry> {
    let Some(Block::NiSourceTexture(source)) = nif.blocks.get(index) else {
        return Vec::new();
    };
    let requested = source.file_name.to_string_lossy().into_owned();
    let mut rows = object_net(&source.base);
    rows.push(Entry::Section("NiSourceTexture"));
    rows.push(text("use_external", source.use_external));

    let mut name_derived = Vec::new();
    if source.use_external {
        name_derived.push(Derived::Note(match library.resolve(&requested) {
            Some(path) => format!("resolves to {}", path.display()),
            None => "not found in any texture directory".into(),
        }));
        name_derived.push(Derived::Image {
            block: index,
            missing: "add a texture directory containing this file".into(),
        });
    }
    rows.push(with(text("file_name", &requested), name_derived));
    if source.use_external {
        if let Some(link) = source.unknown_link_ref {
            rows.push(text("unknown_link_ref", link));
        }
    } else {
        let mut pixels = field("pixel_data_ref", Value::Link(source.pixel_data_ref.index()));
        if let Some(block) = source.pixel_data_ref.index() {
            pixels = with(
                pixels,
                vec![Derived::Image {
                    block,
                    missing: "cannot decode this format".into(),
                }],
            );
        }
        rows.push(pixels);
    }
    rows.push(text("pixel_layout", format!("{:?}", source.pixel_layout)));
    rows.push(text("mipmap_format", format!("{:?}", source.mipmap_format)));
    rows.push(text("alpha_format", format!("{:?}", source.alpha_format)));
    rows.push(text("is_static", source.is_static));
    rows.push(text("direct_render", source.direct_render));

    rows
}

fn fallback(block: &Block) -> String {
    let text = format!("{block:#?}");
    match text.char_indices().nth(200_000) {
        Some((cut, _)) => format!("{}\n...truncated", &text[..cut]),
        None => text,
    }
}
