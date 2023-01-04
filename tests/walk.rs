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
    fn test_comparisons() -> () {
        let test_directory = PathBuf::from("./test_files/test_dir1");
        let children: Vec<PathBuf> = lineage::get_all_children(&test_directory);

        let largest_file_target = PathBuf::from("./test_files/test_dir1/file6.txt");
        assert_eq!(
            comparison::largest_file(&children),
            comparison::SizeQuery {
                name: Some(largest_file_target),
                size: 352,
                unique: true,
            }
        );

        let largest_dir_target = PathBuf::from("./test_files/test_dir1/dir1");
        assert_eq!(
            comparison::largest_dir(&children),
            comparison::SizeQuery {
                name: Some(largest_dir_target),
                size: 192,
                unique: true,
            }
        );

        let smallest_file_target = PathBuf::from("./test_files/test_dir1/file9.txt");
        assert_eq!(
            comparison::smallest_file(&children),
            comparison::SizeQuery {
                name: Some(smallest_file_target),
                size: 0,
                unique: false,
            }
        );

        let smallest_dir_target = None;
        assert_eq!(
            comparison::smallest_dir(&children),
            comparison::SizeQuery {
                name: smallest_dir_target,
                size: 0,
                unique: false
            }
        );
    }
}
