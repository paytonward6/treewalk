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

    impl SizeQuery {
        fn new(name: Option<PathBuf>, size: u64, unique: bool) -> Self {
            Self { name, size, unique }
        }

        fn start_min() -> Self {
            Self {
                name: None,
                size: u64::MAX,
                unique: true,
            }
        }

        fn start_max() -> Self {
            Self {
                name: None,
                size: u64::MIN,
                unique: true,
            }
        }

        fn update(&mut self, name: Option<PathBuf>, size: u64, unique: bool) {
            self.name = name;
            self.size = size;
            self.unique = unique;
        }
    }

    impl PartialEq for SizeQuery {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name && self.size == other.size && self.unique == other.unique
        }
    }

    fn base_comparison<F>(children: &Vec<PathBuf>, file_or_dir: F, is_min: bool) -> SizeQuery
    where
        F: Fn(&Path) -> bool,
    {
        let mut result = if let true = is_min {
            SizeQuery::start_min()
        } else {
            SizeQuery::start_max()
        };
        for child in children {
            if file_or_dir(child) {
                let meta_child = child.as_path().metadata();
                if let Ok(meta_child) = meta_child {
                    if meta_child.len() == result.size {
                        result.unique = false;
                    } else if !is_min && (meta_child.len() > result.size) {
                        result.update(Some(child.to_path_buf()), meta_child.len(), true)
                    } else if is_min && (meta_child.len() < result.size) {
                        result.update(Some(child.to_path_buf()), meta_child.len(), true)
                    }
                }
            }
        }
        result
    }

    pub fn largest_dir(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison(children, |path| path.is_dir(), false)
    }

    pub fn largest_file(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison(children, |path| path.is_file(), false)
    }

    pub fn smallest_file(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison(children, |path| path.is_file(), true)
    }

    pub fn smallest_dir(children: &Vec<PathBuf>) -> SizeQuery {
        base_comparison(children, |path| path.is_dir(), true)
    }
}

pub mod lineage {
    use std::path::Path;
    use std::path::PathBuf;

    pub fn children(path: &Path) -> Vec<PathBuf> {
        let mut children: Vec<PathBuf> = Vec::new();
        for child in path
            .read_dir()
            .expect("Attempt to read contents of directory has failed!")
            .flatten()
        {
            let child_path = child.path();
            children.push(child_path);
        }
        children
    }

    fn get_all_driver(path: &Path, children: &mut Vec<PathBuf>) {
        if path.is_dir() {
            for child in path
                .read_dir()
                .expect("Attempt to read contents of directory has failed!")
                .flatten()
            {
                let child_path = child.path();
                children.push(child_path.to_path_buf());
                get_all_driver(&child_path, children);
            }
        }
    }

    pub fn get_all_children(path: &Path) -> Vec<PathBuf> {
        let mut children: Vec<PathBuf> = Vec::new();
        get_all_driver(path, &mut children);
        children
    }
}

pub mod format {
    use std::collections::HashMap;
    fn construct_hr_output(num: &u64, unit: &str) -> String {
        let units = HashMap::from([
            ("B", 0),
            ("KB", 3),
            ("MB", 6),
            ("GB", 9),
            ("TB", 12),
            ("PB", 15),
        ]);
        let num_to_unit = num / (10u64.pow(units[unit]));
        format!("{:.2}{}", num_to_unit, unit)
        //num_str[..(num_str.len() - units[unit])].to_string() + unit
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
        //let num_str = String::from(&num.to_string());
        match num {
            ..=1_000 => result + &num.to_string() + "B",
            1_001..=1_000_000 => construct_hr_output(&num, "KB"),

            1_000_001..=1_000_000_000 => construct_hr_output(&num, "MB"),
            1_000_000_001..=1_000_000_000_000 => construct_hr_output(&num, "GB"),
            1_000_000_000_001..=1_000_000_000_000_000 => construct_hr_output(&num, "TB"),
            1_000_000_000_000_001.. => construct_hr_output(&num, "PB"),
        }
    }
}
