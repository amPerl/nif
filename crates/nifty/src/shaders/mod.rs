//! The shaders nifty can reproduce, keyed by the technique name a `NiGeometry` stores.
//!
//! A shader is a WGSL fragment entry point written against the contract in `prelude.wgsl`, which
//! is prepended to every one of them. Built ins are compiled in; a user can supply more by
//! pointing at a directory, the way texture roots already work, so a shader this build has never
//! heard of still draws.
//!
//! Only shaders whose source settles what they do belong here. A technique with no entry is
//! reported as unhandled rather than approximated.

use eframe::egui_wgpu::wgpu;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use nif::blocks::TextureSlot;

/// Declarations every shader is written against, prepended to each one before compiling.
const PRELUDE: &str = include_str!("prelude.wgsl");

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
}

/// One shader nifty can draw with.
pub struct Shader {
    pub name: String,
    /// The fragment source, without the prelude.
    pub source: String,
    /// Where each of the four bindings gets its texture.
    pub slots: [Option<Source>; SLOTS],
    /// What each unread slot stands in with.
    pub absent: [Absent; SLOTS],
    pub state: RenderState,
    /// Sampling a shader pins for a slot. `None` leaves the map's own clamp mode and linear.
    pub address: [Option<Sampling>; SLOTS],
    /// Whatever the shader wants told, reaching it as `model.params`. These are the declared
    /// defaults of its own attributes: a file supplies its own only through shader extra data,
    /// which nothing in this game carries.
    pub params: [f32; 4],
    /// Where it came from, for the UI to say so.
    pub origin: Origin,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Origin {
    BuiltIn,
    Directory(PathBuf),
}

impl Shader {
    /// The whole module source, which is the contract followed by this shader's own fragment.
    pub fn module_source(&self) -> String {
        format!("{PRELUDE}\n{}", self.source)
    }
}

/// The fixed function stand in, used by every shape whose geometry names no shader.
fn fixed() -> Shader {
    Shader {
        name: "fixed function".into(),
        source: include_str!("fixed.wgsl").into(),
        slots: [
            Some(Source::Slot(TextureSlot::Base)),
            Some(Source::Slot(TextureSlot::Dark)),
            Some(Source::Slot(TextureSlot::Glow)),
            None,
        ],
        // dark multiplies, glow adds
        absent: [Absent::White, Absent::White, Absent::Black, Absent::White],
        state: RenderState::default(),
        address: [None; SLOTS],
        params: [0.0; 4],
        origin: Origin::BuiltIn,
    }
}

fn built_ins() -> Vec<Shader> {
    vec![
        Shader {
            name: "ActionGameTree".into(),
            source: include_str!("ActionGameTree.wgsl").into(),
            // reads the base slot alone, and carries no shader map
            slots: [Some(Source::Slot(TextureSlot::Base)), None, None, None],
            absent: [Absent::White; SLOTS],
            // the technique sets no blend or alpha state, so the file's own properties stand
            state: RenderState::default(),
            address: [None; SLOTS],
            params: [0.0; 4],
            origin: Origin::BuiltIn,
        },
        Shader {
            name: "OilyFilm".into(),
            source: include_str!("OilyFilm.wgsl").into(),
            // the interference ramp and the warp map are the shader's own attributes, at shader
            // map 0 and 1; a shape that supplies neither falls back to their declared files
            slots: [
                Some(Source::Slot(TextureSlot::Base)),
                Some(Source::Slot(TextureSlot::Shader(0))),
                Some(Source::Slot(TextureSlot::Shader(1))),
                None,
            ],
            // the base multiplies the diffuse, and the other two are added
            absent: [Absent::White, Absent::Black, Absent::Black, Absent::White],
            state: RenderState::default(),
            address: [
                None,
                Some(Sampling::clamped(wgpu::FilterMode::Linear)),
                Some(Sampling::clamped(wgpu::FilterMode::Linear)),
                None,
            ],
            // WarpAlpha then Exponent, both the values the source declares
            params: [1.0, 48.0, 0.0, 0.0],
            origin: Origin::BuiltIn,
        },
        Shader {
            name: "ToonShading".into(),
            source: include_str!("ToonShading.wgsl").into(),
            // the base map comes from the file; the ramp is named by the shader itself and is
            // resolved through the texture library, so root order decides which copy wins
            slots: [
                Some(Source::Slot(TextureSlot::Base)),
                Some(Source::Named("ToonRamp.bmp")),
                None,
                None,
            ],
            absent: [Absent::White; SLOTS],
            // the NSF sets no blend or alpha state, so the file's own properties stand
            state: RenderState::default(),
            // TSAMP_AddressU/V = TADDR_Clamp with TEXF_Point, which is what keeps the bands hard
            address: [
                None,
                Some(Sampling::clamped(wgpu::FilterMode::Nearest)),
                None,
                None,
            ],
            params: [0.0; 4],
            origin: Origin::BuiltIn,
        },
        Shader {
            name: "ActionSpecularBand".into(),
            source: include_str!("ActionSpecularBand.wgsl").into(),
            // the gloss map is the base slot, and there is no shader map
            slots: [Some(Source::Slot(TextureSlot::Base)), None, None, None],
            absent: [Absent::White; SLOTS],
            // the pass is additive and draws over whatever already shaded the surface. It sets
            // no alpha test or depth state, so the file's own properties still govern those.
            state: RenderState {
                blend: Some(Some((wgpu::BlendFactor::One, wgpu::BlendFactor::One))),
                alpha_test: None,
                depth_write: None,
            },
            // the sampler mirrors in u, which is the shader's own state rather than the map's
            address: [
                Some(Sampling {
                    address: (wgpu::AddressMode::MirrorRepeat, wgpu::AddressMode::Repeat),
                    filter: wgpu::FilterMode::Linear,
                }),
                None,
                None,
                None,
            ],
            params: [0.0; 4],
            origin: Origin::BuiltIn,
        },
        Shader {
            name: "ActionGameCartoonFX".into(),
            source: include_str!("ActionGameCartoonFX.wgsl").into(),
            // the decal is the base slot. A ramp is often left in shader map 0 from the
            // deprecated outline path, and this technique does not sample it.
            slots: [Some(Source::Slot(TextureSlot::Base)), None, None, None],
            absent: [Absent::White; SLOTS],
            // the technique sets no blend or alpha state, so the file's own properties stand
            state: RenderState::default(),
            address: [None; SLOTS],
            params: [0.0; 4],
            origin: Origin::BuiltIn,
        },
        Shader {
            name: "VCAlphaTextureBlender".into(),
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
            },
            address: [None; SLOTS],
            params: [0.0; 4],
            origin: Origin::BuiltIn,
        },
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
            shader.slots.iter().filter_map(move |slot| match slot {
                Some(Source::Named(file)) => Some((shader.name.as_str(), *file)),
                _ => None,
            })
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
            // a supplied shader inherits the slot mapping of the built in it replaces, since
            // nothing in a WGSL file says which map it wants. An unknown name gets the shader
            // maps, which is what a custom shader reads in every case measured so far.
            let (slots, absent, state, address, params) = match self.by_name.get(&name) {
                Some(existing) => (
                    existing.slots,
                    existing.absent,
                    existing.state,
                    existing.address,
                    existing.params,
                ),
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
                    [0.0; 4],
                ),
            };
            self.by_name.insert(
                name.clone(),
                Shader {
                    name,
                    source,
                    slots,
                    absent,
                    state,
                    address,
                    params,
                    origin: Origin::Directory(path),
                },
            );
            found += 1;
        }
        found
    }
}
