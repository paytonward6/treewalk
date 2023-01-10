<a href="https://crates.io/crates/treewalk"><img alt="Crate Info" src="https://img.shields.io/crates/v/treewalk.svg"/></a>
# treewalk
Common utilities for exploring a file tree

## Example usage
```rust
    use std::path::{Path, PathBuf};
    use treewalk::walk::{comparison, lineage, format, utils};

    fn main() {
        let path = Path::new("./foo/bar");
        let mut children: Vec<PathBuf> = lineage::get_all_children(&path.to_path_buf());

        // or create children from strings via
        let children_from_strs = utils::tree!["./this_file.txt", "./that_file.txt"]

        let small = comparison::largest_file(&children);
        if let Some(file_name) = &small.name {
            println!("{:?}: {:?}", file_name, format::human_readable(small.size));
        }
    }

```
