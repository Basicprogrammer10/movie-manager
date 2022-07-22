use std::fs;
use std::path::Path;

const TITLE_CHARS: &[char] = &['\''];
const MOVIE_FORMATS: &[&str] = &["mp4", ];

pub struct Show {
    pub name: String,
    pub release_year: u16,
    pub description: String,
    pub rateing: f32,
    pub watched: bool,
    pub episodes: Vec<Show>,
}

impl Show {
    pub fn parse_dir<T: AsRef<Path>>(path: T) -> Vec<Show> {
        let mut out = Vec::new();

        for i in fs::read_dir(path).unwrap().map(|x| x.unwrap()) {
            println!(
                "`{}` - `{}`",
                Self::parse_name(i.file_name().to_string_lossy().to_string()),
                i.file_name().to_string_lossy(),
            )
        }

        out
    }

    fn parse_name(raw: String) -> String {
        let chars = raw.chars().collect::<Vec<_>>();
        let mut tokens = Vec::new();
        let mut working = String::new();
        let mut i = 0;

        while i < chars.len() {
            let this = chars[i];

            if this.is_ascii_alphabetic()
                || TITLE_CHARS.contains(&this)
                || (this.is_ascii_digit() && tokens.is_empty())
            {
                working.push(this);
                i += 1;
                continue;
            }

            match this {
                '.' | ' ' => {
                    tokens.push(working.to_owned());
                    working.clear();
                    i += 1;
                    continue;
                }
                _ => break,
            }
        }

        tokens.join(" ")
    }
}
