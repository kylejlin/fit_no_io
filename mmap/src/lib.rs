use memmap::Mmap;
use std::fs::File;
use std::path::Path;

pub fn from_path<T: AsRef<Path>>(path: T) -> Mmap {
    let file = File::open(path).expect("should be able to open path");
    unsafe {
        Mmap::map(&file).expect("should be able to create memory map")
    }
}