use std::env;
use std::path::{Path, PathBuf};
use treewalk::walk::{comparison, lineage, format};

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let path = Path::new(&args[1]);

        let mut children: Vec<PathBuf> = Vec::new();
        lineage::recursively_list_contents(&path.to_path_buf(), &mut children);

        let small = comparison::largest_file(children);
        println!("{:?}", small);

        for ancestor in path.ancestors() {
            println!("{}", ancestor.display());
        }
    }
    else {
        println!("No files found.");
    }
}
