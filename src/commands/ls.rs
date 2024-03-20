use crate::state::State;

pub fn ls() -> Result<(), anyhow::Error> {
    let dir = State::base_dir()?;

    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries {
            let entry = entry?;
            if entry.metadata()?.is_dir() {
                println!("{}", entry.path().file_name().unwrap().to_string_lossy());
            }
        }
    }

    Ok(())
}
