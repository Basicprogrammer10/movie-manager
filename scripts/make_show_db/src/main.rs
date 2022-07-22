use std::collections::HashMap;
use std::fs;

use pbr::ProgressBar;
use serde::Serialize;

#[derive(Serialize)]
struct Show {
    pub id: String,
    pub name: String,
    pub release_date: Option<u16>,
    pub rateing: Option<f32>,
}

fn main() {
    // Load all names and imdb ids
    let mut processing = Vec::new();
    let akas = fs::read_to_string("./akas.tsv").unwrap();
    let akas = akas
        .lines()
        .skip(1)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let mut pb = ProgressBar::new(akas.len() as u64);
    pb.format("[=> ]");
    pb.message("[LOADING AKAS]    ");

    for i in akas {
        pb.inc();
        let mut parts = i.split('\t');
        let id = parts.next().unwrap().to_owned();
        let name = parts.nth(1).unwrap().to_owned();
        processing.push(Show {
            id,
            name,
            release_date: None,
            rateing: None,
        })
    }
    pb.finish_println("[LOADING AKAS] done");

    processing.dedup_by(|a, b| a.name == b.name);

    // Add startYear if avalable
    let basic = fs::read_to_string("./basics.tsv").unwrap();
    let basic = basic
        .lines()
        .skip(1)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
    let mut start_year_map = HashMap::new();
    pb = ProgressBar::new(basic.len() as u64);
    pb.message("[LOADING BASIC]   ");

    for i in basic {
        pb.inc();
        let mut parts = i.split('\t');
        let id = parts.next().unwrap().to_owned();
        let start_year = match parts.nth(4).unwrap_or("\\N") {
            "\\N" => continue,
            x => x.parse().unwrap(),
        };
        start_year_map.insert(id, start_year);
    }

    for i in &mut processing {
        i.release_date = start_year_map.get(&i.id).copied();
    }
    pb.finish_println("[LOADING BASIC] done");

    // Add rateing if avalable
    let ratings = fs::read_to_string("./ratings.tsv").unwrap();
    let ratings = ratings
        .lines()
        .skip(1)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
    let mut rating_map = HashMap::new();
    pb = ProgressBar::new(ratings.len() as u64);
    pb.message("[LOADING RATINGS] ");

    for i in ratings {
        pb.inc();
        let mut parts = i.split('\t');
        let id = parts.next().unwrap().to_owned();
        let avg_ratings = match parts.next() {
            Some(i) => i.parse().unwrap(),
            None => continue,
        };
        rating_map.insert(id, avg_ratings);
    }

    for i in &mut processing {
        i.rateing = rating_map.get(&i.id).copied();
    }
    pb.finish_println("[LOADING RATINGS] done");

    // Export data to binary format
    println!("[*] Exporting");
    fs::write("show_data.bin", bincode::serialize(&processing).unwrap()).unwrap();
    println!("[*] Done!");
}
