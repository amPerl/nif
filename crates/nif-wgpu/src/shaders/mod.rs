//! The shaders this crate can reproduce, keyed by the technique name a `NiGeometry` stores.
//!
//! A shader is a WGSL fragment entry point written against the contract in `prelude.wgsl`, which
//! is prepended to every one of them. Built ins are compiled in; a user can supply more by
//! pointing at a directory, the way texture roots already work, so a shader this build has never
//! heard of still draws.
//!
//! Only shaders whose source settles what they do belong here. A technique with no entry is
//! reported as unhandled rather than approximated.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use nif::blocks::TextureSlot;

/// Declarations every shader is written against, prepended to each one before compiling.
const PRELUDE: &str = include_str!("prelude.wgsl");

/// Where a pass's vertex displacement is spliced into the contract. It has to land above the
/// vertex stage that calls it, since WGSL has no forward declarations.
const HOOK: &str = "// <displacement>";

/// What a pass that moves no vertex gets in its place.
const NO_DISPLACEMENT: &str = include_str!("no_displacement.wgsl");

/// How many texture slots one shape binds. Which maps those hold is the shader's choice.
pub const SLOTS: usize = 4;

/// A slot a shader leaves unread has to contribute nothing, and what that means depends on how
/// the slot is combined.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Absent {
    /// Multiplied in, so nothing means white.
    White,
    /// Added on, so nothing means black.
    Black,
    /// Doubled by a `Modulate2x`, so nothing means a half.
    Half,
}

impl Absent {
    pub fn texel(&self) -> [u8; 4] {
        match self {
            Absent::White => [255, 255, 255, 255],
            Absent::Black => [0, 0, 0, 255],
            Absent::Half => [128, 128, 128, 255],
        }
    }
}

/// Where a slot's texture comes from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    /// A map the file names in one of its own texture slots.
    Slot(TextureSlot),
    /// A file the shader names itself, resolved through the texture library the way any external
    /// reference is. Nothing in the NIF says which file is meant, and more than one of that name
    /// can be installed, so root order decides which is found.
    Named(&'static str),
    /// A texture the shader declares as an attribute. The shape points it at one of its own
    /// shader maps by carrying an integer extra data called `<attribute>Index`, and the file
    /// the shader declares is only the fallback for a shape that does not.
    Attribute {
        /// The extra data whose value is the shader map to read.
        index: &'static str,
        /// What to resolve by name where the shape names no map.
        file: &'static str,
    },
    /// The same, for a texture the technique binds to a shader map by index rather than naming a
    /// file. The slot is the index the technique itself declares, used where the shape carries
    /// no override of its own.
    IndexedSlot {
        index: &'static str,
        slot: TextureSlot,
    },
}

/// Sampling a shader pins for a slot, since a pass sets its own sampler state and that beats the
/// map's own `TexClampMode`. Point sampling is how a toon ramp gets hard bands, not a blend.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sampling {
    pub address: (wgpu::AddressMode, wgpu::AddressMode),
    pub filter: wgpu::FilterMode,
}

impl Sampling {
    pub fn clamped(filter: wgpu::FilterMode) -> Sampling {
        Sampling {
            address: (
                wgpu::AddressMode::ClampToEdge,
                wgpu::AddressMode::ClampToEdge,
            ),
            filter,
        }
    }
}

/// What a shader overrides about the draw state its properties would otherwise ask for. A pass
/// in a `.fx` or `.NSF` can set its own blend and depth, and that beats the file's own
/// `NiAlphaProperty`, which is how `ActionSpecularBand` gets an additive pass.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct RenderState {
    /// `Some(None)` disables blending, `Some(Some(..))` forces one, `None` leaves the file's own.
    pub blend: Option<Option<(wgpu::BlendFactor, wgpu::BlendFactor)>>,
    pub depth_write: Option<bool>,
    /// Whether the alpha test the file asks for still applies.
    pub alpha_test: Option<bool>,
    /// `Some(None)` draws both faces, `Some(Some(face))` culls that one, `None` leaves the
    /// file's own stencil property to decide. An outline pass culls the front, so only the far
    /// side of its expanded shell survives and the shell reads as a rim.
    pub cull: Option<Option<wgpu::Face>>,
}

/// One draw a shader makes. Most shaders are a single pass; an outline draws an expanded shell
/// first and the surface over it, and the two want different state and different maps.
pub struct Pass {
    /// The fragment source, without the prelude.
    pub source: String,
    /// A pass that moves the vertex supplies its own `displace`, which the contract's vertex
    /// stage calls before projecting. None leaves the position where the geometry put it.
    pub vertex: Option<String>,
    /// Where each of the four bindings gets its texture.
    pub slots: [Option<Source>; SLOTS],
    /// What each unread slot stands in with.
    pub absent: [Absent; SLOTS],
    pub state: RenderState,
    /// Sampling a shader pins for a slot. `None` leaves the map's own clamp mode and linear.
    pub address: [Option<Sampling>; SLOTS],
    /// The uv set a slot reads, where the technique decides it rather than the map.
    ///
    /// A `TexDesc` naming its own set is a fixed function notion: the stage asks for whichever
    /// set the map points at. A technique with its own vertex shader has no such stage, and
    /// reads whatever `TEXCOORD` its source declares, so the map's answer is not consulted at
    /// all. `None` leaves it to the map, which is right for every technique built on stages.
    pub uv_set: [Option<u32>; SLOTS],
}

impl Default for Pass {
    fn default() -> Self {
        Pass {
            source: String::new(),
            vertex: None,
            slots: [None; SLOTS],
            absent: [Absent::White; SLOTS],
            state: RenderState::default(),
            address: [None; SLOTS],
            uv_set: [None; SLOTS],
        }
    }
}

/// One shader a scene can draw with.
pub struct Shader {
    pub name: String,
    /// What it draws, in order. Never empty.
    pub passes: Vec<Pass>,
    /// Whatever the shader wants told, reaching it as `model.params`. These are the declared
    /// defaults of its own attributes, used where a shape supplies nothing.
    pub params: [f32; 4],
    /// The attribute each lane of `params` carries, parallel to it. A shape overrides one by
    /// carrying an extra data block of that name, so the name is what does the binding. An
    /// empty name is a lane no attribute reaches.
    pub param_names: [&'static str; 4],
    /// A colour attribute, reaching the shader as `model.attribute_color`. It takes a whole vec4
    /// where a float takes a lane, and no technique here declares more than one that it reads.
    pub color: [f32; 4],
    /// The extra data name that overrides it, empty where the shader declares no colour.
    pub color_name: &'static str,
    /// Where it came from, for the UI to say so.
    pub origin: Origin,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Origin {
    BuiltIn,
    Directory(PathBuf),
}

impl Pass {
    /// The whole module source for one pass: the contract, the displacement the pass asks for
    /// and its own fragment.
    pub fn module_source(&self) -> String {
        let (head, tail) = PRELUDE
            .split_once(HOOK)
            .expect("the contract says where a displacement goes");
        let displace = self.vertex.as_deref().unwrap_or(NO_DISPLACEMENT);
        [head, displace, tail, &self.source].join("\n")
    }
}

impl Shader {
    /// A shader that draws once, which is all of them but the outline family.
    fn single(name: &str, pass: Pass) -> Shader {
        Shader {
            name: name.into(),
            passes: vec![pass],
            params: [0.0; 4],
            param_names: [""; 4],
            color: [1.0; 4],
            color_name: "",
            origin: Origin::BuiltIn,
        }
    }
}

/// The fixed function stand in, used by every shape whose geometry names no shader.
fn fixed() -> Shader {
    Shader::single(
        "fixed function",
        Pass {
            source: include_str!("fixed.wgsl").into(),
            slots: [
                Some(Source::Slot(TextureSlot::Base)),
                Some(Source::Slot(TextureSlot::Dark)),
                Some(Source::Slot(TextureSlot::Glow)),
                None,
            ],
            // dark multiplies, glow adds
            absent: [Absent::White, Absent::White, Absent::Black, Absent::White],
            ..Pass::default()
        },
    )
}

/// The toon shading pass, which two techniques draw after an outline and one draws alone.
fn toon_shading_pass() -> Pass {
    Pass {
        source: include_str!("ToonShading.wgsl").into(),
        // the base map comes from the file; the ramp is named by the shader itself and is
        // resolved through the texture library, so root order decides which copy wins
        slots: [
            Some(Source::Slot(TextureSlot::Base)),
            Some(Source::Attribute {
                index: "ToonRampIndex",
                file: "ToonRamp.bmp",
            }),
            None,
            None,
        ],
        // clamped and point sampled, which is what keeps the bands hard
        address: [
            None,
            Some(Sampling::clamped(wgpu::FilterMode::Nearest)),
            None,
            None,
        ],
        ..Pass::default()
    }
}

/// An outline drawn as a hull: the surface expanded along its own normal with the front faces
/// culled, so only the far side of the shell survives and reads as a rim, and the surface then
/// draws over it. The original has no pixel shader and passes the colour in as the vertex
/// diffuse, which is what this fragment stands in for.
fn toon_outline_pass() -> Pass {
    Pass {
        source: include_str!("ToonOutline.wgsl").into(),
        vertex: Some(include_str!("ToonOutline.vertex.wgsl").into()),
        // it samples nothing
        slots: [None; SLOTS],
        absent: [Absent::White; SLOTS],
        state: RenderState {
            blend: None,
            alpha_test: None,
            depth_write: Some(true),
            cull: Some(Some(wgpu::Face::Front)),
        },
        address: [None; SLOTS],
        uv_set: [None; SLOTS],
    }
}

/// The two techniques that outline. Their own descriptions differ in nothing this renderer can
/// see: the same two passes over the same two programs.
fn outlined_toon(name: &str) -> Shader {
    // the shading pass here sets its own cull and depth, which standalone ToonShading does not:
    // there the file's own properties decide. Same program, different state around it.
    let surface = Pass {
        state: RenderState {
            blend: None,
            alpha_test: None,
            depth_write: Some(true),
            cull: Some(Some(wgpu::Face::Back)),
        },
        ..toon_shading_pass()
    };
    Shader {
        name: name.into(),
        passes: vec![toon_outline_pass(), surface],
        // outlineThickness, at the value the source declares. outlineColor is a colour
        // attribute, which nothing binds yet, and black is what it declares.
        params: [0.1, 0.0, 0.0, 0.0],
        param_names: ["outlineThickness", "", "", ""],
        // the rim, at the colour the technique declares
        color: [0.0, 0.0, 0.0, 1.0],
        color_name: "outlineColor",
        origin: Origin::BuiltIn,
    }
}

fn built_ins() -> Vec<Shader> {
    vec![
        Shader::single(
            "ActionGameTree",
            Pass {
                source: include_str!("ActionGameTree.wgsl").into(),
                // reads the base slot alone, and carries no shader map
                slots: [Some(Source::Slot(TextureSlot::Base)), None, None, None],
                // the technique sets no blend or alpha state, so the file's own properties stand
                ..Pass::default()
            },
        ),
        Shader {
            name: "OilyFilm".into(),
            passes: vec![Pass {
                source: include_str!("OilyFilm.wgsl").into(),
                // the interference ramp and the warp map are the shader's own attributes, and
                // the shape names which of its shader maps each reads
                slots: [
                    Some(Source::Slot(TextureSlot::Base)),
                    Some(Source::Attribute {
                        index: "filmRampIndex",
                        file: "thinFilmRamp.bmp",
                    }),
                    Some(Source::Attribute {
                        index: "warpEffectIndex",
                        file: "WarpEffects.tga",
                    }),
                    None,
                ],
                // the base multiplies the diffuse, and the other two are added
                absent: [Absent::White, Absent::Black, Absent::Black, Absent::White],
                address: [
                    None,
                    Some(Sampling::clamped(wgpu::FilterMode::Linear)),
                    Some(Sampling::clamped(wgpu::FilterMode::Linear)),
                    None,
                ],
                ..Pass::default()
            }],
            // WarpAlpha then Exponent, at the values the source declares. A shape carrying a
            // float of either name overrides it, and a low WarpAlpha is what fades the surface.
            params: [1.0, 48.0, 0.0, 0.0],
            param_names: ["WarpAlpha", "Exponent", "", ""],
            color: [1.0; 4],
            color_name: "",
            origin: Origin::BuiltIn,
        },
        Shader::single("ToonShading", toon_shading_pass()),
        Shader {
            name: "ToonShadingWithOutline".into(),
            // one pass: it outlines per pixel rather than by drawing a shell, so it shares
            // nothing with the cartoon techniques but the ramp and the attribute names
            passes: vec![Pass {
                source: include_str!("ToonShadingWithOutline.wgsl").into(),
                ..toon_shading_pass()
            }],
            // outlineThickness, at the value the technique declares. outlineColor is a colour
            // attribute, which nothing binds yet, and black is what it declares.
            params: [0.1, 0.0, 0.0, 0.0],
            param_names: ["outlineThickness", "", "", ""],
            color: [0.0, 0.0, 0.0, 1.0],
            color_name: "outlineColor",
            origin: Origin::BuiltIn,
        },
        outlined_toon("ActionGameCartoon"),
        outlined_toon("JiCartoon"),
        Shader {
            name: "ActionSpecularBand".into(),
            passes: vec![Pass {
                source: include_str!("ActionSpecularBand.wgsl").into(),
                // the gloss map is the base slot, and there is no shader map
                slots: [Some(Source::Slot(TextureSlot::Base)), None, None, None],
                // the pass is additive and draws over whatever already shaded the surface. It
                // sets no alpha test or depth state, so the file's own properties govern those.
                state: RenderState {
                    blend: Some(Some((wgpu::BlendFactor::One, wgpu::BlendFactor::One))),
                    alpha_test: None,
                    depth_write: None,
                    cull: None,
                },
                // the sampler mirrors in u, which is the shader's state rather than the map's
                address: [
                    Some(Sampling {
                        address: (wgpu::AddressMode::MirrorRepeat, wgpu::AddressMode::Repeat),
                        filter: wgpu::FilterMode::Linear,
                    }),
                    None,
                    None,
                    None,
                ],
                ..Pass::default()
            }],
            // the exponent the source names Reflection. A shape usually carries its own, and a
            // smaller one spreads the band across the panel rather than pinning it to a point.
            params: [100.0, 0.0, 0.0, 0.0],
            param_names: ["Reflection", "", "", ""],
            color: [1.0; 4],
            color_name: "",
            origin: Origin::BuiltIn,
        },
        Shader {
            name: "ActionGameCartoonFX".into(),
            passes: vec![Pass {
                source: include_str!("ActionGameCartoonFX.wgsl").into(),
                // the decal is the base slot. A ramp is often left in shader map 0 from the
                // deprecated outline path, and this technique does not sample it.
                slots: [Some(Source::Slot(TextureSlot::Base)), None, None, None],
                ..Pass::default()
            }],
            params: [0.0; 4],
            param_names: [""; 4],
            // the body colour the decal is composited over, at the source's declared white
            color: [1.0; 4],
            color_name: "MaterialColor",
            origin: Origin::BuiltIn,
        },
        Shader::single(
            "AGCar2",
            Pass {
                source: include_str!("AGCar2.wgsl").into(),
                // the body decal is the base slot, and the technique binds the window decal and
                // the mask to shader maps by index, which the shape can move
                slots: [
                    Some(Source::Slot(TextureSlot::Base)),
                    Some(Source::IndexedSlot {
                        index: "DecalTex2Index",
                        slot: TextureSlot::Shader(0),
                    }),
                    Some(Source::IndexedSlot {
                        index: "MaskTex0Index",
                        slot: TextureSlot::Shader(1),
                    }),
                    None,
                ],
                // a shape with no mask is all body, which the red and green being zero would
                // otherwise turn entirely into glass
                absent: [Absent::White, Absent::White, Absent::White, Absent::White],
                // its vertex shader wires TEXCOORD0 to the decal and TEXCOORD1 to both the mask
                // and the window decal, whatever set the maps themselves name
                uv_set: [Some(0), Some(1), Some(1), None],
                ..Pass::default()
            },
        ),
        Shader::single(
            "VCAlphaTextureBlender",
            Pass {
                source: include_str!("VCAlphaTextureBlender.wgsl").into(),
                slots: [
                    Some(Source::Slot(TextureSlot::Shader(0))),
                    Some(Source::Slot(TextureSlot::Shader(1))),
                    Some(Source::Slot(TextureSlot::Shader(2))),
                    None,
                ],
                // the detail stage is Modulate2x, so an absent detail map is a half not a white
                absent: [Absent::White, Absent::White, Absent::Half, Absent::White],
                // the vertex alpha is a blend weight rather than an opacity, so both go off
                state: RenderState {
                    blend: Some(None),
                    alpha_test: Some(false),
                    depth_write: None,
                    cull: None,
                },
                ..Pass::default()
            },
        ),
    ]
}

/// Every shader available to draw with, plus where a user can put more.
pub struct Shaders {
    fixed: Shader,
    by_name: BTreeMap<String, Shader>,
    roots: Vec<PathBuf>,
}

impl Default for Shaders {
    fn default() -> Self {
        let mut by_name = BTreeMap::new();
        for shader in built_ins() {
            by_name.insert(shader.name.clone(), shader);
        }
        Shaders {
            fixed: fixed(),
            by_name,
            roots: Vec::new(),
        }
    }
}

impl Shaders {
    pub fn fixed(&self) -> &Shader {
        &self.fixed
    }

    /// The shader for a technique name, or None when nothing can draw it. An empty name is not a
    /// custom shader at all, so it takes the fixed function path.
    pub fn get(&self, name: &str) -> Option<&Shader> {
        if name.is_empty() {
            return Some(&self.fixed);
        }
        self.by_name.get(name)
    }

    /// How many shaders can be drawn with, the fixed function stand in aside.
    pub fn count(&self) -> usize {
        self.by_name.len()
    }

    pub fn roots(&self) -> &[PathBuf] {
        &self.roots
    }

    pub fn names(&self) -> impl Iterator<Item = (&str, &Origin)> {
        self.by_name
            .values()
            .map(|shader| (shader.name.as_str(), &shader.origin))
    }

    /// Every texture a shader names for itself. These do not appear anywhere in a NIF, so
    /// without listing them there is no way to see which copy the library picked.
    pub fn named_textures(&self) -> impl Iterator<Item = (&str, &'static str)> {
        self.by_name.values().flat_map(|shader| {
            shader
                .passes
                .iter()
                .flat_map(|pass| pass.slots.iter())
                .filter_map(move |slot| match slot {
                    Some(Source::Named(file)) | Some(Source::Attribute { file, .. }) => {
                        Some((shader.name.as_str(), *file))
                    }
                    _ => None,
                })
        })
    }

    /// Every attribute a shader declares, with the default it falls back to. A shape overrides
    /// one by carrying extra data of that name, and nothing else in the viewer says which names
    /// a shader is looking for.
    pub fn attributes(&self) -> impl Iterator<Item = (&str, &'static str, f32)> {
        self.by_name.values().flat_map(|shader| {
            shader
                .param_names
                .iter()
                .enumerate()
                .filter(|(_, name)| !name.is_empty())
                .map(move |(lane, name)| (shader.name.as_str(), *name, shader.params[lane]))
        })
    }

    /// Indexes `<root>/<TechniqueName>.wgsl`. A later root wins, so a user's own copy overrides
    /// a built in of the same name, which is what makes this worth pointing at a directory.
    /// Returns whether anything changed.
    pub fn add_root(&mut self, root: PathBuf) -> bool {
        if self.roots.contains(&root) {
            return false;
        }
        let found = self.index(&root);
        self.roots.push(root);
        found > 0
    }

    fn index(&mut self, root: &Path) -> usize {
        let Ok(entries) = std::fs::read_dir(root) else {
            return 0;
        };
        let mut found = 0;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("wgsl") {
                continue;
            }
            let Some(name) = path.file_stem().map(|n| n.to_string_lossy().into_owned()) else {
                continue;
            };
            let Ok(source) = std::fs::read_to_string(&path) else {
                continue;
            };
            // A supplied shader inherits the slot mapping and the attributes of the built in it
            // replaces, since nothing in a WGSL file says which map it wants. An unknown name
            // gets the shader maps, which is what a custom shader reads.
            //
            // It replaces the whole shader, passes included, so a file dropped in over an
            // outlining technique draws once rather than twice. Nothing in a `.wgsl` can say
            // otherwise yet, and a manifest is what that would take.
            let existing = self.by_name.get(&name);
            let (slots, absent, state, address, uv_set) = match existing {
                Some(existing) => {
                    let pass = &existing.passes[0];
                    (
                        pass.slots,
                        pass.absent,
                        pass.state,
                        pass.address,
                        pass.uv_set,
                    )
                }
                None => (
                    [
                        Some(Source::Slot(TextureSlot::Shader(0))),
                        Some(Source::Slot(TextureSlot::Shader(1))),
                        Some(Source::Slot(TextureSlot::Shader(2))),
                        Some(Source::Slot(TextureSlot::Base)),
                    ],
                    [Absent::White; SLOTS],
                    RenderState::default(),
                    [None; SLOTS],
                    [None; SLOTS],
                ),
            };
            let (params, param_names, color, color_name) = match existing {
                Some(existing) => (
                    existing.params,
                    existing.param_names,
                    existing.color,
                    existing.color_name,
                ),
                None => ([0.0; 4], [""; 4], [1.0; 4], ""),
            };
            self.by_name.insert(
                name.clone(),
                Shader {
                    name,
                    passes: vec![Pass {
                        source,
                        vertex: None,
                        slots,
                        absent,
                        state,
                        address,
                        uv_set,
                    }],
                    params,
                    param_names,
                    color,
                    color_name,
                    origin: Origin::Directory(path),
                },
            );
            found += 1;
        }
        found
    }
}
