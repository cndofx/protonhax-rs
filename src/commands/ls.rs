use std::path::PathBuf;

pub fn ls(protonhax_dir: PathBuf) -> Result<(), anyhow::Error> {
    if let Ok(entries) = std::fs::read_dir(&protonhax_dir) {
        for entry in entries {
            let entry = entry?;
            if entry.metadata()?.is_dir() {
                println!("{}", entry.path().file_name().unwrap().to_string_lossy());
            }
        }
    }
    Ok(())
}
