use std::fs;
use std::path::PathBuf;

const VIDEO_FORMATS: [&str; 3] = ["m4v", "mkv", "mp4"];

pub fn find_movie_file(dir: PathBuf) -> Option<PathBuf> {
    let valid_files = fs::read_dir(dir)
        .unwrap()
        .map(|x| x.unwrap())
        .filter(|x| x.path().is_file())
        .filter(|x| {
            VIDEO_FORMATS.contains(&x.file_name().to_string_lossy().rsplit_once('.').unwrap().1)
        })
        .collect::<Vec<_>>();

    if valid_files.len() > 1 {
        return None;
    }

    valid_files.iter().next().map(|x| x.path())
}
