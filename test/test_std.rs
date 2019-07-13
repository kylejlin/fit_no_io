use fit_no_io as fit;

fn main() {
    let mmap = mmap::from_path("data/garmin_1000.fit");
    fit::run(&mmap);
}
