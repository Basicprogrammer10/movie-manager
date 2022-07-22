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
    let akas = akas.lines().skip(1).collect::<Vec<_>>();

    let mut pb = ProgressBar::new(akas.len() as u64);
    pb.format("[=>_]");
    pb.message("[LOADING AKAS]    ");

    for i in akas {
        pb.inc();
        let mut parts = i.split("\t");
        let id = parts.next().unwrap().to_owned();
        let name = parts.nth(1).unwrap().to_owned();
        processing.push(Show::new(id, name))
    }
    pb.finish_println("[LOADING AKAS] done");

    processing.dedup_by(|a, b| a.name == b.name);

    // Add startYear if avalable
    let basic = fs::read_to_string("./basics.tsv").unwrap();
    let basic = basic.lines().skip(1).collect::<Vec<_>>();
    pb = ProgressBar::new(basic.len() as u64);
    pb.message("[LOADING BASIC]   ");

    for i in basic {
        pb.inc();
        let mut parts = i.split("\t");
        let id = parts.next().unwrap().to_owned();
        let start_year = match parts.nth(4).unwrap() {
            "\\N" => continue,
            x => x,
        };

        let val = Some(start_year.parse().unwrap());
        for j in &mut processing {
            if j.release_date.is_none() && j.id == id {
                j.release_date = val;
            }
        }
    }

    // Add rateing if avalable
    let ratings = fs::read_to_string("./ratings.tsv").unwrap();
    let ratings = ratings.lines().skip(1).collect::<Vec<_>>();
    pb = ProgressBar::new(ratings.len() as u64);
    pb.message("[LOADING RATINGS] ");

    for i in ratings {
        pb.inc();
        let mut parts = i.split("\t");
        let id = parts.next().unwrap().to_owned();
        let avg_ratings = parts.next().unwrap();

        let val = Some(avg_ratings.parse().unwrap());
        for j in &mut processing {
            if j.rateing.is_none() && j.id == id {
                j.rateing = val;
            }
        }
    }

    // Export data to binary format
    println!("[*] Exporting");
    fs::write("show_data.bin", bincode::serialize(&processing).unwrap()).unwrap();
    println!("[*] Done!");
}

impl Show {
    fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            release_date: None,
            rateing: None,
        }
    }
}
