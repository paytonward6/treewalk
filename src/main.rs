use std::fs;
use std::io;
use std::env;
use std::path::Path;
use std::path::PathBuf;

fn children(path: &Path) -> Vec<PathBuf> {
    let mut children: Vec<PathBuf> = Vec::new();
    if path.is_dir() {
        for child in path.read_dir().expect("Attempt to read contents of directory has failed!") {
            if let Ok(child) = child {
                let child_path = child.path();
                children.push(child_path);
            }
        }
    }
    children
}

#[derive(Debug)]
struct SizeQuery {
    item: Option<PathBuf>,
    size: u64,
    //uniq: bool,
}

//TODO: implement uniq for sizequery
fn largest_file(children: Vec<PathBuf>) -> SizeQuery {
    let mut result = SizeQuery{item: None, size: 0};
    for child in children {
        if child.is_file() {
            let size = child.as_path().metadata();
            if let Ok(size) = size {
                if size.len() > result.size {
                    result.item = Some(child);
                    result.size = size.len();
                }
            }
        }
    }
    result
}

//TODO: implement uniq for sizequery
fn largest_dir(children: Vec<PathBuf>) -> SizeQuery {
    let mut result = SizeQuery{item: None, size: 0};
    for child in children {
        if child.is_dir() {
            let size = child.as_path().metadata();
            if let Ok(size) = size {
                if size.len() > result.size {
                    result.item = Some(child);
                    result.size = size.len();
                }
            }
        }
    }
    result
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let path = Path::new(&args[1]);
        let f = largest_file(children(path));
        println!("{:?}: {:?}", f.item.unwrap(), f.size);

        let d = largest_(children(path));
        println!("{:?}: {:?}", d.item.unwrap(), d.size);
    } else {
        println!("No files found.");
    }
}
