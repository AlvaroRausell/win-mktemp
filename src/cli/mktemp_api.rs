use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn mktemp_base(base_dir: &Path) -> PathBuf {
    if (!base_dir.exists()) {
        println!(
            "Base directory {} does not exist, creating...",
            base_dir.to_str().unwrap()
        );
        fs::create_dir_all(base_dir).unwrap();
    }
    let mut rng = thread_rng();
    let target_name: String = rng
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    return Path::new(base_dir).join(target_name);
}
pub fn mktemp_dir(base_dir: &Path) -> String {
    let path = mktemp_base(base_dir);
    fs::create_dir(&path).unwrap();
    path.display().to_string()
}

pub fn mktemp_file(base_dir: &Path) -> String {
    let path = mktemp_base(base_dir);
    fs::File::create(&path).unwrap();
    path.display().to_string()
}
