use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
   let mut files = vec![];
   crawl_dir(&Path::new("."), &mut files)?;
   println!("{:?}", files);
   println!("il y a {} fichiers dans ce repertoire et ses sous repertoires", files.len());
   Ok(())

}

fn crawl_dir(dir : &Path, files: &mut Vec<String>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path.to_string_lossy().to_string());
        }
        else {
           crawl_dir(&path, files)?;
        }
    }
    Ok(())
}
