use std::io::Cursor;

use nif::{collectors::single_mesh::Mesh, Nif};

#[test]
fn nif_26_mesh() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/26.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif: Nif = Nif::parse(&mut nif_cursor)?;

    let mut mesh = Mesh::default();
    mesh.add_nif(&nif, 0.0)?;
    assert_eq!(mesh.vertices.len(), 24);
    assert_eq!(mesh.indices.len(), 36);

    Ok(())
}
