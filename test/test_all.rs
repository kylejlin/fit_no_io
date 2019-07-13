extern crate mmap;

use fit_no_io as fit;

fn main() {
    for entry in std::fs::read_dir("data").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(s) = &path.extension() {
            if let Some("fit") = s.to_str() {
                dbg!(&path);
                let mmap = mmap::from_path(path);
                fit::run(&mmap);
            }
        }
    }
}
