//! Resolves the texture paths in a NIF against directories on disk.
//!
//! A NIF may reference `some/path/thing.bmp` when the file present is `Obj/THING.DDS`. Lookup
//! uses the file stem alone, lowercased, so the directories and extension in the request are
//! ignored.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Indexable extensions, most preferred first. Other files in a texture directory (`.nif`,
/// `.hit`, `.dat`) are skipped so they cannot match a texture stem.
const EXTENSIONS: [&str; 5] = ["dds", "tga", "bmp", "png", "jpg"];

fn rank(path: &Path) -> Option<usize> {
    let extension = path.extension()?.to_str()?.to_ascii_lowercase();
    EXTENSIONS.iter().position(|known| *known == extension)
}

/// The file stem of a request, lower cased. `a\\b\\Thing.BMP` and `Thing.dds` both key on `thing`.
fn key(requested: &str) -> Option<String> {
    let normalised = requested.replace('\\', "/");
    let name = normalised.rsplit('/').next()?;
    let stem = Path::new(name).file_stem()?.to_str()?;
    (!stem.is_empty()).then(|| stem.to_ascii_lowercase())
}

#[derive(Default)]
pub struct TextureLibrary {
    roots: Vec<PathBuf>,
    by_stem: HashMap<String, PathBuf>,
    /// Number of stems matched by more than one file. Reported in the settings window only.
    collisions: usize,
}

impl TextureLibrary {
    pub fn roots(&self) -> &[PathBuf] {
        &self.roots
    }

    pub fn indexed(&self) -> usize {
        self.by_stem.len()
    }

    pub fn collisions(&self) -> usize {
        self.collisions
    }

    /// Adds a root, ignoring one that is already present. Returns whether anything changed.
    pub fn add_root(&mut self, root: PathBuf) -> bool {
        if self.roots.contains(&root) {
            return false;
        }
        self.roots.push(root);
        self.rescan();
        true
    }

    pub fn remove_root(&mut self, index: usize) {
        if index < self.roots.len() {
            self.roots.remove(index);
            self.rescan();
        }
    }

    pub fn rescan(&mut self) {
        self.by_stem.clear();
        self.collisions = 0;
        for root in &self.roots {
            for entry in walkdir::WalkDir::new(root)
                .follow_links(false)
                .into_iter()
                .filter_map(Result::ok)
            {
                if !entry.file_type().is_file() {
                    continue;
                }
                let path = entry.into_path();
                let Some(rank) = rank(&path) else { continue };
                let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                    continue;
                };
                let stem = stem.to_ascii_lowercase();
                match self.by_stem.get(&stem) {
                    Some(existing) => {
                        self.collisions += 1;
                        // prefer the better extension, otherwise keep the first file scanned
                        if rank < self::rank(existing).unwrap_or(usize::MAX) {
                            self.by_stem.insert(stem, path);
                        }
                    }
                    None => {
                        self.by_stem.insert(stem, path);
                    }
                }
            }
        }
    }

    pub fn resolve(&self, requested: &str) -> Option<&Path> {
        self.by_stem.get(&key(requested)?).map(PathBuf::as_path)
    }

    /// Resolves and decodes. `None` means not found or not decodable.
    pub fn load(&self, requested: &str) -> Option<(u32, u32, Vec<u8>)> {
        decode_file(self.resolve(requested)?)
    }
}

fn decode_file(path: &Path) -> Option<(u32, u32, Vec<u8>)> {
    let bytes = std::fs::read(path).ok()?;
    if path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("dds"))
    {
        return crate::dds::decode(&bytes);
    }
    let image = image::load_from_memory(&bytes).ok()?.to_rgba8();
    Some((image.width(), image.height(), image.into_raw()))
}

/// A magenta and grey checker, drawn where a texture could not be loaded.
pub fn placeholder() -> (u32, u32, Vec<u8>) {
    const SIZE: u32 = 16;
    let mut rgba = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let dark = ((x / 4) + (y / 4)) % 2 == 0;
            let pixel: [u8; 4] = if dark {
                [90, 90, 96, 255]
            } else {
                [230, 70, 200, 255]
            };
            rgba.extend_from_slice(&pixel);
        }
    }
    (SIZE, SIZE, rgba)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_request_keys_on_its_stem_alone() {
        assert_eq!(key("thing.bmp").as_deref(), Some("thing"));
        assert_eq!(key("some/path/Thing.BMP").as_deref(), Some("thing"));
        assert_eq!(key(r"some\path\THING.tga").as_deref(), Some("thing"));
        assert_eq!(key("thing").as_deref(), Some("thing"));
    }

    #[test]
    fn a_nameless_request_has_no_key() {
        assert_eq!(key(""), None);
        assert_eq!(key("some/path/"), None);
    }

    #[test]
    fn a_stem_with_dots_keeps_everything_before_the_last_one() {
        assert_eq!(
            key("ob_h_1st_00050a_01.tex.bmp").as_deref(),
            Some("ob_h_1st_00050a_01.tex")
        );
    }

    #[test]
    fn extension_rank_prefers_dds_and_ignores_unknown_files() {
        assert!(rank(Path::new("a.dds")) < rank(Path::new("a.bmp")));
        assert!(rank(Path::new("a.DDS")).is_some());
        assert_eq!(rank(Path::new("a.nif")), None);
        assert_eq!(rank(Path::new("a")), None);
    }
}
