use std::{fs, path::Path};

pub fn run_test_folder<P: AsRef<Path>>(folder: P, callback: impl Fn(&Path)) {
    for entry in fs::read_dir(folder).expect("couldn't read directory") {
        let entry = entry.unwrap();
        let path = entry.path().canonicalize().unwrap();
        if path.is_dir() {
            callback(path.as_path());
        }
    }
}
