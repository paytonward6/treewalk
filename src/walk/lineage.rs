use std::path::Path;
use std::path::PathBuf;

pub fn children(path: &Path) -> Vec<PathBuf> {
    let mut children: Vec<PathBuf> = Vec::new();
    for child in path
        .read_dir()
        .expect("Attempt to read contents of directory has failed!")
        .flatten()
    {
        let child_path = child.path();
        children.push(child_path);
    }
    children
}

fn get_all_driver(path: &Path, children: &mut Vec<PathBuf>) {
    if path.is_dir() {
        for child in path
            .read_dir()
            .expect("Attempt to read contents of directory has failed!")
            .flatten()
        {
            let child_path = child.path();
            children.push(child_path.to_path_buf());
            get_all_driver(&child_path, children);
        }
    }
}

pub fn get_all_children(path: &Path) -> Vec<PathBuf> {
    let mut children: Vec<PathBuf> = Vec::new();
    get_all_driver(path, &mut children);
    children
}
