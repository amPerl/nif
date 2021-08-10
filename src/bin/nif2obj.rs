use nif::obj::Obj;
use nif::Nif;
use std::fs::File;
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    for path in collect_paths() {
        dbg!(&path);

        let mut file = File::open(&path)?;
        let nif = Nif::parse(&mut file)?;

        let mut obj = Obj::default();
        obj.visit_nif(&nif, Some("Root".into()));

        let obj_path = path.with_extension("obj");
        let mtl_path = path.with_extension("mtl");

        obj.write_to_files(obj_path, mtl_path)?;
        // println!("{:?}", &obj);
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
