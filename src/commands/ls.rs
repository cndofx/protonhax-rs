use std::path::PathBuf;

pub fn ls(protonhax_dir: PathBuf) {
    if let Ok(entries) = std::fs::read_dir(&protonhax_dir) {
        for entry in entries {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                println!("{}", entry.path().file_name().unwrap().to_string_lossy());
            }
        }
    }
}
