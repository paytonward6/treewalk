#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::path::{Path, PathBuf};
use treewalk::walk::{comparison, lineage, format};

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use treewalk::walk::{comparison, lineage, format};
    #[test]
    fn test_format_human_readable() {
        assert_eq!(format::human_readable(999), "999B");
        assert_eq!(format::human_readable(1_000), "1000B");
        assert_eq!(format::human_readable(10_000), "10KB");
        assert_eq!(format::human_readable(100_000), "100KB");
        assert_eq!(format::human_readable(500_000), "500KB");
        assert_eq!(format::human_readable(1_000_000), "1000KB");
        assert_eq!(format::human_readable(10_000_000), "10MB");
        assert_eq!(format::human_readable(100_000_000), "100MB");
        assert_eq!(format::human_readable(1_000_000_000), "1000MB");
        assert_eq!(format::human_readable(1_000_000_001), "1GB");
        assert_eq!(format::human_readable(1_000_000_000_000), "1000GB");
        assert_eq!(format::human_readable(10_000_000_000_000), "10TB");
    }

    #[test]
    fn test_comparison_largest_dir() -> () {
        let test_directory = PathBuf::from("./test_files/test_dir1");
        let mut children: Vec<PathBuf> = Vec::new();
        lineage::recursively_list_contents(&test_directory, &mut children);

        let largest_file_target = PathBuf::from("./test_files/test_dir1/file6.txt");
        assert_eq!(
            comparison::largest_file(&children),
            comparison::SizeQuery {
                name: Some(largest_file_target),
                size: 352,
                unique: true,
            }
        );
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let path = Path::new(&args[1]);

        let mut children: Vec<PathBuf> = Vec::new();
        lineage::recursively_list_contents(&path.to_path_buf(), &mut children);

        let small = comparison::largest_file(&children);
        if let Some(file_name) = &small.name {
            println!("{:?}: {:?}", file_name, small.size);
        }

    }
    else {
        println!("No files found.");
    }
}
