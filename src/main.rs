use std::env;
use std::path::{Path, PathBuf};
use treewalk::walk::{comparisons, lineage, format};

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let path = Path::new(&args[1]);

        let mut children: Vec<PathBuf> = Vec::new();
        lineage::recursively_list_contents(&path.to_path_buf(), &mut children);

        let small = comparisons::largest_file(children);
        if let Some(file_name) = &small.name {
            println!("{:?}: {:?}", file_name, format::human_readable(small.size));
        }

        for ancestor in path.ancestors() {
            println!("{}", ancestor.display());
        }
    }
    else {
        println!("No files found.");
    }
}
