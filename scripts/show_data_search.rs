//! ```cargo
//! [dependencies]
//! bincode = "1.3.3"
//! serde = { version = "1.0.140", features = ["derive"] }
//! ```

use std::fs;

use serde::{Deserialize, Serialize};

const NAME: &str = "Top Gun";

#[derive(Serialize, Deserialize, Debug)]
struct Show {
    pub id: String,
    pub name: String,
    pub release_date: Option<u16>,
    pub rateing: Option<f32>,
    pub rateings: u32,
}

fn main() {
    println!("[*] Reading Data");
    let data = fs::read("./show_data.bin").unwrap();

    println!("[*] Loading Data");
    let data = bincode::deserialize::<Vec<Show>>(&data).unwrap();

    println!("[*] Finding Show");
    let name = NAME.to_lowercase();
    let mut found = data
        .iter()
        .filter(|x| x.name.to_lowercase() == name)
        .collect::<Vec<_>>();

    if found.is_empty() {
        println!("[-] Not Found");
        return;
    }

    found.sort_by(|a, b| a.rateings.cmp(&b.rateings));
    for i in found.iter().rev() {
        println!("{:?}", i);
    }
}
