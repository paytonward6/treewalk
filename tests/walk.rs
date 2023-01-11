#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::path::{Path, PathBuf};
use treewalk::walk::{comparison, format, lineage};

#[cfg(test)]
mod tests {
    use format::Units;
    use std::path::{Path, PathBuf};
    use treewalk::walk::{comparison, format, lineage, utils};
    #[test]
    fn test_format_human_readable() {
        assert_eq!(format::human_readable(999, false), "999B");
        assert_eq!(format::human_readable(1_000, false), "1000B");
        assert_eq!(format::human_readable(10_000, false), "10KB");
        assert_eq!(format::human_readable(100_000, false), "100KB");
        assert_eq!(format::human_readable(500_000, false), "500KB");
        assert_eq!(format::human_readable(1_000_000, false), "1000KB");
        assert_eq!(format::human_readable(10_000_000, false), "10MB");
        assert_eq!(format::human_readable(100_000_000, false), "100MB");
        assert_eq!(format::human_readable(1_000_000_000, false), "1000MB");
        assert_eq!(format::human_readable(1_000_000_001, false), "1GB");
        assert_eq!(format::human_readable(1_000_000_000_000, false), "1000GB");
        assert_eq!(format::human_readable(10_000_000_000_000, false), "10TB");
    }

    #[test]
    fn test_comparisons() {
        let children_children: Vec<PathBuf> =
            lineage::get_all_children(&PathBuf::from("./tests/test_files/children"));
        let test_children_children = [
            "./tests/test_files/children/file2.txt",
            "./tests/test_files/children/file3.txt",
            "./tests/test_files/children/file1.txt",
            "./tests/test_files/children/file4.txt",
            "./tests/test_files/children/file5.txt",
            "./tests/test_files/children/dir2",
            "./tests/test_files/children/dir2/file2.txt",
            "./tests/test_files/children/dir2/file3.txt",
            "./tests/test_files/children/dir2/file1.txt",
            "./tests/test_files/children/dir2/file4.txt",
            "./tests/test_files/children/dir2/file5.txt",
            "./tests/test_files/children/dir3",
            "./tests/test_files/children/dir1",
            "./tests/test_files/children/dir1/file2.txt",
            "./tests/test_files/children/dir1/file3.txt",
            "./tests/test_files/children/dir1/file1.txt",
            "./tests/test_files/children/dir1/file4.txt",
        ];

        assert_eq!(
            children_children,
            test_children_children.map(|path| PathBuf::from(path))
        );

        let test_comparison_directory = PathBuf::from("./tests/test_files/test_dir1");
        let comparison_children: Vec<PathBuf> =
            lineage::get_all_children(&test_comparison_directory);

        comparison::total_size(&comparison_children, Units::KB);

        // BEGIN EXTREMA COMPARISONS
        {
            let largest_file_target = PathBuf::from("./tests/test_files/test_dir1/file6.txt");
            assert_eq!(
                comparison::largest_file(&comparison_children),
                comparison::SizeQuery {
                    name: Some(largest_file_target),
                    size: 352,
                    unique: true,
                }
            );

            let largest_dir_target = PathBuf::from("./tests/test_files/test_dir1/dir1");
            assert_eq!(
                comparison::largest_dir(&comparison_children),
                comparison::SizeQuery {
                    name: Some(largest_dir_target),
                    size: 192,
                    unique: true,
                }
            );

            let smallest_file_target = PathBuf::from("./tests/test_files/test_dir1/file2.txt");
            assert_eq!(
                comparison::smallest_file(&comparison_children),
                comparison::SizeQuery {
                    name: Some(smallest_file_target),
                    size: 0,
                    unique: false,
                }
            );

            let smallest_dir_target = PathBuf::from("./tests/test_files/test_dir1/dir2");
            assert_eq!(
                comparison::smallest_dir(&comparison_children),
                comparison::SizeQuery {
                    name: Some(smallest_dir_target),
                    size: 64,
                    unique: false
                }
            );

            //let test_comparison_directory = PathBuf::from("/Users/payton/Code/Linux/");
            //let comparison_children: Vec<PathBuf> = lineage::get_all_children(&test_comparison_directory);
            //let size_range_target = ["./tests/test_files/test_dir1/file4.txt", "./tests/test_files/test_dir1/file6.txt"];
            //assert_eq!(comparison::size_range(&comparison_children, 2..4, Units::GB), size_range_target.map(|path| PathBuf::from(path)));
        } // END EXTREMA COMPARISON

        // BEGIN RANGE COMPARISON
        {
            let size_range_target = [
                "./tests/test_files/test_dir1/file4.txt",
                "./tests/test_files/test_dir1/file6.txt",
            ];
            assert_eq!(
                comparison::size_range(&comparison_children, 200..400, Units::B),
                size_range_target.map(|path| PathBuf::from(path))
            );
        } // END RANGE COMPARISON
    }
    #[test]
    fn test_utils() {
        let test_vec: Vec<PathBuf> = vec!["./foo", "./bar"]
            .iter()
            .map(|path| PathBuf::from(path))
            .collect();
        assert_eq!(test_vec, utils::tree!["./foo", "./bar"]);
        println!("{:?}", test_vec);
    }
}
