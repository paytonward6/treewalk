#![allow(dead_code)]
#![allow(unused_imports)]
mod treewalk {
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

    #[derive(Debug)]
    pub struct SizeQuery {
        pub name: Option<PathBuf>,
        pub size: u64,
        //uniq: bool,
    }

    //TODO: implement uniq for sizequery
    pub fn largest_file(children: Vec<PathBuf>) -> SizeQuery {
        let mut result = SizeQuery{name: None, size: 0};
        for child in children {
            if child.is_file() {
                let size = child.as_path().metadata();
                if let Ok(size) = size {
                    if size.len() > result.size {
                        result.name = Some(child);
                        result.size = size.len();
                    }
                }
            }
        }
        result
    }

    //TODO: implement uniq for sizequery
    pub fn largest_dir(children: Vec<PathBuf>) -> SizeQuery {
        let mut result = SizeQuery{name: None, size: 0};
        for child in children {
            if child.is_dir() {
                let size = child.as_path().metadata();
                if let Ok(size) = size {
                    if size.len() > result.size {
                        result.name = Some(child);
                        result.size = size.len();
                    }
                }
            }
        }
        result
    }

    pub fn smallest_file(children: Vec<PathBuf>) -> SizeQuery {
        let mut result = SizeQuery{name: None, size: u64::MAX};
        for child in children {
            if child.is_file() {
                let size = child.as_path().metadata();
                if let Ok(size) = size {
                    if size.len() < result.size {
                        result.name = Some(child);
                        result.size = size.len();
                    }
                }
            }
        }
        result
    }

    pub fn smallest_dir(children: Vec<PathBuf>) -> SizeQuery {
        let mut result = SizeQuery{name: None, size: u64::MAX};
        for child in children {
            if child.is_dir() {
                let size = child.as_path().metadata();
                if let Ok(size) = size {
                    if size.len() < result.size {
                        result.name = Some(child);
                        result.size = size.len();
                    }
                }
            }
        }
        result
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

    pub fn digit_len(num: u64) -> u64 {
        let mut first = num;
        let mut i = 0;
        if num == 0 {
            return 0
        }
        while first != 0 {
            first /= 10;
            i += 1;
        }
        i
    }
        //assert_eq!(human_readable(999), "999B");
        //assert_eq!(human_readable(1_000), "1000B");
        //assert_eq!(human_readable(10_000), "10KB");
        //assert_eq!(human_readable(100_000), "100KB");
        //assert_eq!(human_readable(500_000), "500KB");
        //assert_eq!(human_readable(1_000_000), "1000KB");
        //assert_eq!(human_readable(10_000_000), "10MB");
        //assert_eq!(human_readable(100_000_000), "100MB");
        //assert_eq!(human_readable(1_000_000_000), "1000MB");
        //assert_eq!(human_readable(1_000_000_001), "1GB");
        //assert_eq!(human_readable(1_000_000_000_000), "100GB");
        //u64 MAX: 18_446_744_073_709_551_615
    pub fn human_readable(num: u64) -> String {
        let result = String::from("");
        let num_str = String::from(&num.to_string());
        match num {
            ..=1_000 => result + &num_str + "B",
            1_001..=1_000_000 =>  result + &num_str[..(num_str.len() - 3)] + "KB",

            1_000_001..=1_000_000_000 =>  result + &num_str[..(num_str.len() - 6)] + "MB",
            1_000_000_001..=1_000_000_000_000 => result + &num_str[..(num_str.len() - 9)] + "GB",
            1_000_000_000_001..=1_000_000_000_000_000 => result + &num_str[..(num_str.len() - 12)] + "TB",
            1_000_000_000_000_001.. => result + &num_str[..(num_str.len() - 15)] + "PB",
        }
    }

}
    //pub fn thuman_readable(num: u64) -> String {
    //    let result = String::from("");
    //    let num_str = String::from(&num.to_string());
    //    let symbols = ["B", "KB", "MB", "GB"];

    //    for i in 0..4 {
    //        let ti: usize = i;
    //        if i*1000 < num && num < (i+1)*1000 {
    //            return result + &num_str[..(num_str.len() - ti*3)] + symbols[i]
    //        }
    //    }
    //    return String::from("too large")
    //}

fn main() {
    use std::env;
    use std::path::{Path, PathBuf};
    use treewalk::*;

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        //let path = Path::new(&args[1]);

        //let mut children: Vec<PathBuf> = Vec::new();
        //recursively_list_contents(&path.to_path_buf(), &mut children);

        //let small = largest_file(children);
        //if let Some(file_name) = &small.name {
        //    println!("{:?}: {:?}", file_name, small.size);
        //}

        //let num: u64 = 10_000_000;
        //println!("{}", human_readable(num));

        assert_eq!(human_readable(999), "999B");
        assert_eq!(human_readable(1_000), "1000B");
        assert_eq!(human_readable(10_000), "10KB");
        assert_eq!(human_readable(100_000), "100KB");
        assert_eq!(human_readable(500_000), "500KB");
        assert_eq!(human_readable(1_000_000), "1000KB");
        assert_eq!(human_readable(10_000_000), "10MB");
        assert_eq!(human_readable(100_000_000), "100MB");
        assert_eq!(human_readable(1_000_000_000), "1000MB");
        assert_eq!(human_readable(1_000_000_001), "1GB");
        assert_eq!(human_readable(1_000_000_000_000), "1000GB");
        assert_eq!(human_readable(10_000_000_000_000), "10TB");
    }
    else {
        println!("No files found.");
    }

}
