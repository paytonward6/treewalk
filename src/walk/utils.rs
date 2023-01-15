#[allow(unused_imports)]
use std::path::PathBuf;
/// returns [`Vec`] of [`PathBuf`]s with which to operate on
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use treewalk::walk::utils;
/// let test_vec: Vec<PathBuf> = vec!["./foo", "./bar"]
///     .iter()
///     .map(|path| PathBuf::from(path))
///     .collect();
/// assert_eq!(test_vec, utils::tree!["./foo", "./bar"]);
/// ```
///
#[macro_export]
macro_rules! tree {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<PathBuf> = Vec::new();
            $(
                temp_vec.push(PathBuf::from($x));
            )*
            temp_vec
        }
    };
}
pub use tree;

/// returns the number of sub-directories of a given path
pub fn num_dirs(children: &Vec<PathBuf>) -> usize {
    children.iter().filter(|path| path.is_dir()).count()
}

/// returns the number of sub-files of a given path
pub fn num_files(children: &Vec<PathBuf>) -> usize {
    children.iter().filter(|path| path.is_file()).count()
}
