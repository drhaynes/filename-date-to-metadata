use std::{fs, io};
use std::env::current_dir;
use filetime::FileTime;
use chrono::{NaiveDateTime};
use std::path::Path;

fn main() {
    let dir_path = current_dir().expect("Failed to get current dir");
    visit_dirs(dir_path.as_path(), &modify_file_dates)
        .expect("Failed visiting dirs")
}

fn modify_file_dates(entry: &fs::DirEntry) {
    let path_buf = entry.path();
    let path = path_buf.as_path();
    let parsed_date = extract_date_from_path(path);

    match parsed_date {
        None => {}
        Some(parsed_date) => {
            let timestamp = parsed_date.timestamp();
            let filetime = FileTime::from_unix_time(timestamp, 0);
            filetime::set_file_times(path, filetime, filetime)
                .expect("Could not set file times");
        }
    }
}

fn extract_date_from_path(path: &Path) -> Option<NaiveDateTime> {
    let filename = path
        .file_stem()
        .and_then(|os_str| os_str.to_str())
        .unwrap();

    let date_parts: Vec<&str> = filename.split('_').collect();
    if date_parts.len() < 2 {
        return None;
    }
    let date_str = date_parts[0];
    let time_str = date_parts[1];

    let datetime_str = format!("{} {}", date_str, time_str);
    let format_str = "%Y-%m-%d %H-%M-%S";

    match NaiveDateTime::parse_from_str(&datetime_str, format_str) {
        Ok(date) => Some(date),
        Err(_) => None,
    }
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&fs::DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
