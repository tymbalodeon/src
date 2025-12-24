use dirs::home_dir;
use walkdir::WalkDir;

pub fn list() -> Vec<String> {
    let mut repos = WalkDir::new(home_dir().unwrap().join("src"))
        .into_iter()
        .filter_map(|path| {
            if let Ok(path) = path.as_ref() {
                if path.file_type().is_dir() && path.depth() == 3 {
                    if let Some(path) = path.path().to_str() {
                        Some(path.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    repos.sort();

    repos
}
