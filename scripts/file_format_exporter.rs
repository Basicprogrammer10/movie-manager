//! ```cargo
//! [dependencies]
//! glob = "0.3.0"
//! ```

use glob::glob;

fn main() {
    let mut formats = Vec::new();
    for i in glob("D:\\Movies\\**\\*").unwrap().map(|x| x.unwrap()) {
        if !i.is_file() {
            continue;
        }

        let format = i.file_name().unwrap().to_string_lossy();
        if let Some(i) = format.rsplitn(2, '.').next() {
            formats.push(i.to_string());
        }
    }

    formats.sort();
    formats.dedup();
    println!("{:?}", formats);
}
