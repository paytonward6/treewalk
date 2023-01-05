#![allow(dead_code)]
#![allow(unused_imports)]

pub mod comparison {
    use std::path::Path;
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct SizeQuery {
        pub name: Option<PathBuf>,
        pub size: u64,
        pub unique: bool,
    }

    impl PartialEq for SizeQuery {
        fn eq(&self, other: &Self) -> bool {
            if self.name == other.name && self.size == other.size && self.unique == other.unique {
                return true;
            } else {
                return false;
            }
        }
    }
    // Unsure if needed:
    // impl Eq for SizeQuery {}

    pub fn base_comparison_file<F>(
        children: &Vec<PathBuf>,
        comparison: F,
        check_min: bool,
    ) -> SizeQuery
    where
        F: Fn(u64, u64) -> bool,
    {
        let mut result = if let true = check_min {
            SizeQuery {
                name: None,
                size: u64::MAX,
                unique: true,
            }
        } else {
            SizeQuery {
                name: None,
                size: u64::MIN,
                unique: true,
            }
        };
        for child in children {
            if child.is_file() {
                let meta_child = child.as_path().metadata();
                if let Ok(meta_child) = meta_child {
                    if meta_child.len() == result.size {
                        result.unique = false;
                        result.name = Some(child.to_path_buf());
                    } else if comparison(meta_child.len(), result.size) {
                        result.name = Some(child.to_path_buf());
                        result.size = meta_child.len();
                        result.unique = true;
                    }
                }
            }
        }
        result
    }

    fn base_comparison_dir<F>(children: &Vec<PathBuf>, comparison: F, check_min: bool) -> SizeQuery
    where
        F: Fn(u64, u64) -> bool,
    {
        let mut result = if let true = check_min {
            SizeQuery {
                name: None,
                size: u64::MAX,
                unique: true,
            }
        } else {
            SizeQuery {
                name: None,
                size: u64::MIN,
                unique: true,
            }
        };
        for child in children {
            if child.is_dir() {
                let meta_child = child.as_path().metadata();
                if let Ok(meta_child) = meta_child {
                    if meta_child.len() == result.size {
                        result.unique = false;
                    }
                    if comparison(meta_child.len(), result.size) {
                        result.name = Some(child.to_path_buf());
                        result.size = meta_child.len();
                        result.unique = true;
                    }
                }
            }
        }
        result
    }

    pub fn largest_dir(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison_dir(children, |child: u64, max: u64| child > max, false)
    }

    pub fn largest_file(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison_file(children, |child: u64, max: u64| child > max, false)
    }

    pub fn smallest_file(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison_file(children, |child: u64, min: u64| child < min, true)
    }

    pub fn smallest_dir(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison_dir(children, |child: u64, min: u64| child < min, true)
    }
}

pub mod lineage {
    use std::path::Path;
    use std::path::PathBuf;

    pub fn children(path: &Path) -> Vec<PathBuf> {
        let mut children: Vec<PathBuf> = Vec::new();
        if path.is_dir() {
            for child in path
                .read_dir()
                .expect("Attempt to read contents of directory has failed!")
            {
                if let Ok(child) = child {
                    let child_path = child.path();
                    children.push(child_path);
                }
            }
        }
        children
    }

    fn get_all_driver(path: &PathBuf, children: &mut Vec<PathBuf>) -> () {
        if path.is_dir() {
            for child in path
                .read_dir()
                .expect("Attempt to read contents of directory has failed!")
            {
                if let Ok(child) = child {
                    let child_path = child.path();
                    children.push(child_path.to_path_buf());
                    get_all_driver(&child_path, children);
                }
            }
        }
    }

    pub fn get_all_children(path: &PathBuf) -> Vec<PathBuf> {
        let mut children: Vec<PathBuf> = Vec::new();
        get_all_driver(path, &mut children);
        children
    }
}

pub mod format {
    use std::collections::HashMap;
    fn construct_hr_output(num_str: &str, unit: &str) -> String {
        let units = HashMap::from([
            ("B", 0),
            ("KB", 3),
            ("MB", 6),
            ("GB", 9),
            ("TB", 12),
            ("PB", 15),
        ]);
        num_str[..(num_str.len() - units[unit])].to_string() + unit
    }

    /// ```
    /// use treewalk::walk::format;
    /// assert_eq!(format::human_readable(1_000), "1000B");
    /// assert_eq!(format::human_readable(10_000_000), "10MB");
    /// assert_eq!(format::human_readable(100_000_000), "100MB");
    /// assert_eq!(format::human_readable(1_000_000_000), "1000MB");
    /// assert_eq!(format::human_readable(1_000_000_001), "1GB");
    /// assert_eq!(format::human_readable(1_000_000_000_000), "1000GB");
    /// assert_eq!(format::human_readable(10_000_000_000_000), "10TB");
    /// ```
    pub fn human_readable(num: u64) -> String {
        let result = String::from("");
        let num_str = String::from(&num.to_string());
        match num {
            ..=1_000 => result + &num_str + "B",
            1_001..=1_000_000 => construct_hr_output(&num_str, "KB"),

            1_000_001..=1_000_000_000 => construct_hr_output(&num_str, "MB"),
            1_000_000_001..=1_000_000_000_000 => construct_hr_output(&num_str, "GB"),
            1_000_000_000_001..=1_000_000_000_000_000 => construct_hr_output(&num_str, "TB"),
            1_000_000_000_000_001.. => construct_hr_output(&num_str, "PB"),
        }
    }
}
