use nif::gltf::Gltf;
use nif::Nif;
use std::fs::File;
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    for path in collect_paths() {
        dbg!(&path);

        let mut file = File::open(&path)?;
        let nif = Nif::parse(&mut file)?;

        let mut gltf = Gltf::new();

        let name = path
            .file_name()
            .expect("weird filename")
            .to_str()
            .expect("weird filename");

        gltf.visit_nif(&nif, Some(name), name);

        let gltf_path = path.with_extension("gltf");
        gltf.write_to_files(gltf_path)?;
        // println!("{:?}", &gltf);
    }
    Ok(())
}

fn collect_paths() -> Vec<PathBuf> {
    let mut nif_paths = Vec::new();
    for arg in std::env::args() {
        if arg.to_lowercase().ends_with(".nif") {
            let path = Path::new(&arg);
            if path.exists() {
                nif_paths.push(PathBuf::from(path));
            }
        }
    }
    nif_paths
}
