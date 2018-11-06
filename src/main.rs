use std::ffi::OsString;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

mod file_tree;

fn main() -> std::io::Result<()> {
    let mut files = vec![];
    crawl_dir(&Path::new("."), &mut files)?;
    println!("{:?}", files);
    println!(
        "il y a {} fichiers dans ce repertoire et ses sous repertoires",
        files.len()
    );
    let mut input = files.iter().map(|s| OsString::from(s));
    let trees = file_tree::make_trees(&mut input);

    let mut stdout = io::stdout();
    let mut v = Vec::new();
    for tree in trees {
        tree.print(&mut stdout, &mut v).unwrap();
    }
    Ok(())
}

fn crawl_dir(dir: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path.to_string_lossy().to_string());
        } else {
            crawl_dir(&path, files)?;
        }
    }
    Ok(())
}
