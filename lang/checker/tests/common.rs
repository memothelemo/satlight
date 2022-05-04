use std::{fs, path::Path};

pub fn run_test_folder<P: AsRef<Path>>(folder: P, callback: &impl Fn(&Path)) {
    for entry in fs::read_dir(folder).expect("couldn't read directory") {
        let entry = entry.unwrap();
        let path = entry.path().canonicalize().unwrap();

        // check if that file extension is lun only
        let file_ext = entry.path();
        if let Some(file_ext) = file_ext.extension() {
            if file_ext == "lun" {
                callback(&path);
            }
        }
        if path.is_dir() {
            run_test_folder(path, callback);
        }
    }
}
