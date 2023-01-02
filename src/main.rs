use std::env;
use std::path::{Path, PathBuf};
use treewalk::walk::{comparisons, lineage};

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let path = Path::new(&args[1]);

        let mut children: Vec<PathBuf> = Vec::new();
        lineage::recursively_list_contents(&path.to_path_buf(), &mut children);

        let children = lineage::children(path);
        println!("{:?}", children);
        let small = comparisons::largest_file(children);
        if let Some(file_name) = &small.name {
            println!("{:?}: {:?}", file_name, small.size);
        }

        let num: u64 = 10_000_000;
        println!("{}", comparisons::human_readable(num));

        assert_eq!(comparisons::human_readable(999), "999B");
        assert_eq!(comparisons::human_readable(1_000), "1000B");
        assert_eq!(comparisons::human_readable(10_000), "10KB");
        assert_eq!(comparisons::human_readable(100_000), "100KB");
        assert_eq!(comparisons::human_readable(500_000), "500KB");
        assert_eq!(comparisons::human_readable(1_000_000), "1000KB");
        assert_eq!(comparisons::human_readable(10_000_000), "10MB");
        assert_eq!(comparisons::human_readable(100_000_000), "100MB");
        assert_eq!(comparisons::human_readable(1_000_000_000), "1000MB");
        assert_eq!(comparisons::human_readable(1_000_000_001), "1GB");
        assert_eq!(comparisons::human_readable(1_000_000_000_000), "1000GB");
        assert_eq!(comparisons::human_readable(10_000_000_000_000), "10TB");
    }
    else {
        println!("No files found.");
    }

}
