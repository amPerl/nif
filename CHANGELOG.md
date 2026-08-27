# Changelog

## nifty v0.5.0 - 2026-08-27

### Added

- Added a capture mode. `--capture[=DIR]` turns each file through a full circle, saves the preview
  at every step and exits. `--yaw-step`, `--pitch`, `--size` and `--time` control it.
- Added support for sphere and cube mapped environments.
- Added gloss maps, which mask the reflection.
- Added support for drawing through a camera the file carries.
- Added support for the lights a file carries, in place of the viewer's own.

### Changed

- Flip controllers now drive every slot they target rather than only the base slot, for shapes and
  particle systems alike.
- Particles now start in the space of the object their emitter names, for all four emitter kinds,
  rather than at the particle system's own origin.
- A particle system flagged world space now keeps its particles in world space, so a moving node no
  longer drags them along.
- Material colour controllers now drive ambient and self illumination.
- Environment maps now upload once per scene rather than once per shape and pass.
- Framing a shape now uses its position in the current frame rather than its resting position.
- The texture bind group is now built from separately named slot, environment and cube parameters,
  so a caller cannot fill the environment binding by accident.

### Fixed

- Fixed an issue with particles where the sprite was bound to every texture slot, so each particle
  was drawn as its own texture squared plus itself.
- Fixed an issue with particles where the environment slot read the particle's own sprite, so each
  particle added a reflection mapped copy of itself over itself.
- Fixed an issue with the scene bounds where a particle system was measured from its own node
  rather than from its emitter, which pulled the grid off centre.
- Fixed an issue with cube maps where they were uploaded as sRGB while every other texture was
  uploaded in gamma space, making them too dark.

## nif v0.7.0 - 2026-08-27

### Added

- Added light resolution, including specular, the ambient channel and the dimmer, and gathering the
  lights in force on a shape down the scene graph.
- Added gathering of the texture effects in force on a shape.
- Added look at interpolator support.
- Added spawn on death for particles.
- Added `Block::triangles`, which gathers a shape's triangles whichever form stores them.
- Added `anim::material_color_at`, which samples the material channel a colour controller drives.
- Added `System::place_against` and `System::emitter_objects`, so a caller can supply the
  transforms an emitter places against.

### Changed

- **Breaking:** renamed `Matrix33::column_major` to `row_major`. The layout is unchanged; the name
  was wrong.
- **Breaking:** `Particle` gained a `generation` field.
- Particles now start in the space of the object their emitter names, for all four emitter kinds.

### Fixed

- Fixed an issue with morph and shader attribute tracks where they were sampled against the file's
  clock instead of the controller's own local time.
- Fixed an issue with looping emitters where they stopped firing at the end of their own span.
- Fixed an issue with lights where they were resolved in the wrong space for the scene being drawn.

## nif v0.6.0 - 2026-08-26

### Added

- Added the `anim` module, covering transform, alpha, texture transform, flip and shader attribute
  controllers, morph targets, path interpolators and Tbc key interpolation.
- Added particle system simulation, including emission, grow and fade, colour, rotation and gravity
  modifiers, and the engine's own particle size floor.
- Added skinning, placing a shape from its bones.
- Added billboard orientation.
- Added property accumulation down the scene graph.
- Added `BinWrite` support, so a parsed file can be written back.
- Added footer parsing, scene graph traversal, block upcasts and flag accessors.
- Added facet reflection behind a feature.
- Added a strips to triangles iterator and a geometry accessor.

### Changed

- **Breaking:** `NiString::value` is now `Vec<u8>` rather than `String`. Use `to_string_lossy()`.
- **Breaking:** removed around 40 count fields. Use `.len()` or `NiGeometryData::vertex_count()`.
- **Breaking:** `num_uv_sets` and `tspace_flag` are now a single `data_flags` field.
- **Breaking:** `material_data.shader_name`, `tex.translation` and `shader_texture.map` are now
  enum accessors.
- Made counts, header block types and grouped fields derived on write rather than stored, so a
  written file cannot disagree with its blocks.
- Made glam an optional dependency.
- Boxed the largest block variants and preallocated vectors to reduce peak memory.
- Unrecognised enum variants now carry their value instead of failing the parse.
- Moved nif into a workspace.

### Removed

- Removed the userland exporters and parse boilerplate.

### Fixed

- Fixed an issue with rotation order where Euler angles were composed in reverse.
- Fixed an issue with LOD selection where the most detailed level was picked by index instead of by
  range.
- Fixed an issue with hidden nodes where they were not culled along with their subtrees.
- Fixed an issue with string parsing where lengths were read incorrectly.
- Fixed an issue with `NiPalette` where the entry count was ignored.
- Removed several panics from the parse path.

## nifty v0.4.0 - 2026-08-26

### Added

- Added an animation timeline and preview.
- Added drawing for particle systems, with picking.
- Added pluggable custom shaders, one module per technique, covering toon shading, per pixel
  outlines, oily film and `AGCar2` body and glass masking.
- Added shader attribute and colour attribute binding from a shape's own extra data.
- Added an environment light with a control window.
- Added dark and glow texture slots.
- Added axes and a ground grid, keyed to whole units.
- Added 4x MSAA.

### Changed

- The scene is now drawn near the origin so f32 keeps its precision on models placed far out.
- The grid is now kept under the scene rather than at the world origin, so the far plane stays
  close.
- Unsorted blended shapes now draw in traversal order rather than after everything else.
- A technique can now pin the uv set its slots read.
- The far plane now accounts for how far an animation carries geometry.

### Fixed

- Fixed an issue with picking where the ray did not agree with the camera, with animated state, or
  with the space the file uses.
- Fixed an issue with particle picking where a particle was tested as a sphere rather than as the
  quad it draws.
- Fixed an issue with depth comparison where coincident surfaces flickered.
- Fixed an issue with the far plane where it clipped the grid.
- Fixed an issue with the LOD slider where picking flickered.

## nifty v0.3.0 - 2026-08-20

### Added

- Added support for external textures, with texture root priority when a name resolves in more than
  one directory.
- Added a details panel driven by facet reflection rather than hand written rows.
- Added LOD handling.
- Added support for passing multiple NIF files as arguments.

### Changed

- The root node is now expanded by default.

### Fixed

- Fixed an issue with LOD selection where the most detailed level was picked by index instead of by
  range.

## nifty v0.2.0 - 2026-08-20

### Added

- Added rendering for `NiTriStrips`, with a strips to triangles iterator in nif.
- Added ray triangle picking.
- Added basic alpha blending.
- Added texture thumbnails to the hierarchy.

### Fixed

- Fixed an issue with lighting where surfaces were shaded incorrectly.
- Fixed an issue with gamma where colours were too dark.
- Fixed an issue with the hierarchy where selecting a block did not scroll to it.

## nifty v0.1.0 - 2026-08-19

### Added

- Added the nifty inspector.
- Added a workflow that creates a release when the version is bumped.

## nif v0.5.0 - 2024-08-02

### Fixed

- Fixed several JSON serialisation issues and clippy warnings.

## nif v0.4.2 - 2023-11-26

### Added

- Added node tree visiting utilities.

### Changed

- Migrated the parser to binrw.
- Bumped the remaining dependencies.

### Fixed

- Fixed an issue with `NiPlane` where it was defined twice.
- Fixed an issue with LOD nodes where child rotations were applied incorrectly.
- Fixed a build failure caused by a breaking change in `gltf_json`.

## nif v0.4.0 - 2022-02-06

### Added

- Added parsing for basic node blocks, plus billboard, LOD and collision blocks.
- Added a `nif2obj` binary and a glTF export utility.
- Added glam conversions for the vector and matrix types.

### Changed

- Excluded `tests/` from the published crate.
- Fixed the homepage URL in the manifest.
