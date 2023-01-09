use std::path::Path;
use std::path::PathBuf;
use std::ops::Range;
use crate::walk::format::Units;

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

pub fn largest_dir(children: &Vec<PathBuf>) -> SizeQuery {
    base_extrema_comparison(children, |path| path.is_dir(), false)
}

pub fn largest_file(children: &Vec<PathBuf>) -> SizeQuery {
    base_extrema_comparison(children, |path| path.is_file(), false)
}

pub fn smallest_file(children: &Vec<PathBuf>) -> SizeQuery {
    base_extrema_comparison(children, |path| path.is_file(), true)
}

pub fn smallest_dir(children: &Vec<PathBuf>) -> SizeQuery {
    base_extrema_comparison(children, |path| path.is_dir(), true)
}

fn base_extrema_comparison<F>(children: &Vec<PathBuf>, file_or_dir: F, is_min: bool) -> SizeQuery
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

pub fn size_range(children: &Vec<PathBuf>, range: Range<u64>, units: Units) -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = Vec::new();
    for child in children {
        let meta_child = child.metadata();
        if let Ok(meta_child) = meta_child {
            if range.contains(&(&meta_child.len() / units as u64)) {
                result.push(child.to_path_buf());
            }
        }
    }
    result
}

