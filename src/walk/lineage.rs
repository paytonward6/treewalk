use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

pub fn children(path: &Path) -> Result<Vec<PathBuf>> {
    let mut children: Vec<PathBuf> = Vec::new();
    let path = path.read_dir();

    if let Ok(path) = path {
        for child in path.flatten() {
            let child_path = child.path();
            children.push(child_path);
        }
    } else if let Err(path) = path {
        //panic!("Attempt to read contents of directory has failed!")
        println!("Attempt to read contents of {path} has failed!")
    }

    Ok(children)
}

fn get_all_driver(path: &Path, children: &mut Vec<PathBuf>) {
    if path.is_dir() {
        let path = path.read_dir();
        if let Ok(path) = path {
            for child in path.flatten() {
                let child_path = child.path();
                children.push(child_path.to_path_buf());
                get_all_driver(&child_path, children);
            }
        } else if let Err(error) = path {
            // report and leave out the directories that cannot be read
            eprintln!(
                "{}: Attempt to read contents of {:?} has failed!",
                error.kind(),
                children.last().unwrap()
            );
            children.pop();
        }
    }
}

/// returns a [Vec] of the children of a [Path] (recursively)
pub fn get_all_children(path: &Path) -> Vec<PathBuf> {
    let mut children: Vec<PathBuf> = Vec::new();
    get_all_driver(path, &mut children);
    children
}
