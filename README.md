# [treewalk v0.1.1](https://crates.io/crates/treewalk)
Common utilities for exploring a file tree

## Example usage
```rust
    use std::path::{Path, PathBuf};
    use treewalk::walk::{comparison, lineage, format};

    fn main() {
        let path = Path::new("./foo/bar");

        let mut children: Vec<PathBuf> = Vec::new();
        lineage::recursively_list_contents(&path.to_path_buf(), &mut children);

        let small = comparison::largest_file(children);
        if let Some(file_name) = &small.name {
            println!("{:?}: {:?}", file_name, format::human_readable(small.size));
        }

        for ancestor in path.ancestors() {
            println!("{}", ancestor.display());
        }
    }

```
