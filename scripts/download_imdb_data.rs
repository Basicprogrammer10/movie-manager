//! ```cargo
//! [dependencies]
//! http_req = "0.8.1"
//! flate2 = "1.0.24"
//! pbr = "1.0.4"
//! ```

use std::fs;
use std::io::prelude::*;

use flate2::read::GzDecoder;
use http_req::request;
use pbr::ProgressBar;

const BASE_URL: &str = "https://datasets.imdbws.com";
const FILES: [[&str; 2]; 3] = [
    ["title.ratings.tsv.gz", "ratings.tsv"],
    ["title.basics.tsv.gz", "basics.tsv"],
    ["title.akas.tsv.gz", "akas.tsv"],
];

fn main() {
    let mut pb = ProgressBar::new((FILES.len() * 3) as u64);
    pb.show_speed = false;
    pb.show_time_left = false;
    pb.format("[=> ]");

    let _ = fs::create_dir("./data");

    for i in FILES {
        pb.message(&format!("Downloading `{}` ", i[0]));
        let mut writer = Vec::new();
        let _ = request::get(&format!("{}/{}", BASE_URL, i[0]), &mut writer).unwrap();
        pb.inc();

        pb.message(&format!("Decompressing `{}` ", i[0]));
        let mut out = Vec::new();
        let mut d = GzDecoder::new(writer.as_slice());
        d.read_to_end(&mut out).unwrap();
        pb.inc();

        pb.message(&format!("Writing `{}` ", i[1]));
        fs::write(format!("./data/{}", i[1]), out).unwrap();
        pb.inc();
    }

    pb.message("Done");
}
