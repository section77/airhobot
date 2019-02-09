use log::debug;
use std::path::PathBuf;

pub fn asset_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("assets");
    p.push(name);
    debug!("asset path for {} is {:?}", name, p.as_path());
    p
}
