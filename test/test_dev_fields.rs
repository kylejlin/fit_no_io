use fit_no_io as fit;
use std::path::PathBuf;

fn main() {
    let mmap = mmap::from_path("data/working_wahoo_dev_fields.fit_");
    fit::run(&mmap);
}
