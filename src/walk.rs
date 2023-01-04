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
            if self.name == other.name
                && self.size == other.size
                && self.unique == other.unique
            {
                return true
            }
            else {
                return false
            }
        }
    }
    // Unsure if needed:
    // impl Eq for SizeQuery {}

    pub fn base_comparison<F>(children: &Vec<PathBuf>, comparison: F) -> SizeQuery
        where F: Fn(u64, u64) -> bool {
        let mut result = SizeQuery{name: None, size: 0, unique: true};
        for child in children {
            if child.is_file() {
                let size = child.as_path().metadata();
                if let Ok(size) = size {
                    if size.len() == result.size {
                        result.unique = false;
                    }
                    if comparison(size.len(), result.size) {
                        result.name = Some(child.to_path_buf());
                        result.size = size.len();
                        result.unique = true;
                    }
                }
            }
        }
        result
    }

    pub fn largest_dir(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison(children, |max:u64, child:u64| max > child)
    }

    pub fn largest_file(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison(children, |max: u64, child:u64| max > child)
    }

    pub fn smallest_file(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison(children, |max: u64, child:u64| max < child)
    }

    pub fn smallest_dir(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison(children, |max: u64, child:u64| max < child)
    }
}

pub mod lineage {
    use std::path::Path;
    use std::path::PathBuf;

    pub fn children(path: &Path) -> Vec<PathBuf> {
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

    pub fn recursively_list_contents(path: &PathBuf, children: &mut Vec<PathBuf>) -> () {
        if path.is_dir() {
            for child in path.read_dir().expect("Attempt to read contents of directory has failed!") {
                if let Ok(child) = child {
                    let child_path = child.path();
                    children.push(child_path.to_path_buf());
                    recursively_list_contents(&child_path, children);
                }
            }
        }
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
            1_001..=1_000_000 =>  construct_hr_output(&num_str, "KB"),

            1_000_001..=1_000_000_000 => construct_hr_output(&num_str, "MB"),
            1_000_000_001..=1_000_000_000_000 => construct_hr_output(&num_str, "GB"),
            1_000_000_000_001..=1_000_000_000_000_000 => construct_hr_output(&num_str, "TB"),
            1_000_000_000_000_001.. => construct_hr_output(&num_str, "PB"),
        }
    }
}

