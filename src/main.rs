
use dmap::formats::{DmapRecord, RawacfRecord, FitacfRecord, to_file};
mod fitting;
use std::fs::File;
// use std::io::Read;
use std::path::Path;

fn main() {
    let file = File::open("tests/test_files/20210607.1801.00.cly.a.rawacf").expect("Test file not found");
    // let file = File::open(Path::new("tests/test_files/20160316.1945.01.rkn.iqdat"))
    //     .expect("Test file not found");
    // let file = File::open(Path::new("tests/test_files/20110214.map"))
    //     .expect("Test file not found");
    let contents = RawacfRecord::read_records(file).unwrap();

    to_file("tests/test_files/temp.rawacf", &contents).unwrap();
}
