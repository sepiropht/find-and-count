use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
extern crate colored;

use colored::*;
const VERTICAL_CHAR: char = '│';
const HORIZONTAL_STR: &'static str = "├──";
const LAST_HORIZONTAL_STR: &'static str = "└──";
const REPLACEMENT_CHAR: char = '?';



#[derive(Debug, PartialEq, Eq)]
pub struct FileTree {
    name: OsString,
    frame_work: String,
    childs: Vec<FileTree>,
}

fn print_line<W: Write>(output: &mut W, lasts: &[bool], name: &OsStr, frame_work: &str) -> io::Result<()> {
    let name: String = name
        .to_string_lossy()
        .chars()
        .map(|c| if c.is_control() { REPLACEMENT_CHAR } else { c })
        .collect();

    if lasts.len() > 0 {
        for last in &lasts[..lasts.len() - 1] {
            let c = if *last { ' ' } else { VERTICAL_CHAR };
            write!(output, "{}   ", c)?;
        }
        if *lasts.last().unwrap() {
            write!(output, "{} ", LAST_HORIZONTAL_STR)?;
        } else {
            write!(output, "{} ", HORIZONTAL_STR,)?;
        }
    }
    if frame_work == "react" {
         writeln!(output, "{} : {}", name, frame_work.color("blue"))?;

    } else {
        writeln!(output, "{} : {}", name, frame_work.color("red"))?; 
    }
   

    Ok(())
}

fn extract_framework(path: &Path) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    if contents.contains("react") {
        return Ok("react".to_string()); 
    }  
    if contents.contains("angular") {
     return Ok("angular".to_string());
    } else {
        return Ok("".to_string());
    }
}

impl FileTree {
    pub fn print<W: Write>(&self, out: &mut W, lasts: &mut Vec<bool>) -> io::Result<()> {
        print_line(out, &lasts[..], &*self.name, &*self.frame_work)?;
        lasts.push(false);
        for (i, child) in self.childs.iter().enumerate() {
            if i + 1 == self.childs.len() {
                *lasts.last_mut().unwrap() = true;
            }
            child.print(out, lasts)?;
        }
        lasts.pop();
        Ok(())
    }

    fn add<'a, I: Iterator<Item = &'a OsStr>>(&mut self, name_iter: &mut I, abs_path: String) {
        if let Some(c) = name_iter.next() {
            let mut found = false;
            for child in &mut self.childs {
                if &*child.name == c {
                    child.add(name_iter, abs_path.clone());
                    found = true;
                    break;
                }
            }

            if !found {
                
                let new_child = FileTree {
                    name: c.to_os_string(),
                    frame_work: extract_framework(&Path::new(&abs_path)).expect("putain fonctionne !!!"),
                    childs: vec![],
                };
                
                self.childs.push(new_child);
                self.childs.last_mut().unwrap().add(name_iter, abs_path);
            }
        }
    }
}

pub fn make_trees<I, O>(input: &mut I) -> Vec<FileTree>
where
    I: Iterator<Item = O>,
    O: AsRef<OsStr>,
{
    let mut pseudo_root = FileTree {
        name: OsString::new(),
        frame_work: "".to_string(),
        childs: vec![],
    };

    for line in input {
        let path = Path::new(&line);
        //println!("merde {:?}", path);
        let mut components = path.components().map(|c| c.as_os_str());
        pseudo_root.add(&mut components, path.to_string_lossy().to_string());
    }

    pseudo_root.childs
}

#[cfg(test)]
mod tests {
    use super::{make_trees, print_line, FileTree};
    use std::ffi::{OsStr, OsString};

    fn test_single_tree_creation(lines: &[&str], expected_tree: FileTree) {
        let trees = make_trees(&mut lines.iter());
        assert_eq!(1, trees.len());
        assert_eq!(expected_tree, trees[0]);
    }

    #[test]
    fn test_tree_creation1() {
        let lines = ["a", "a/b", "a/b/c/d", "a/b/e"];
        let e = FileTree {
            name: OsString::from("e"),
            childs: vec![],
        };
        let d = FileTree {
            name: OsString::from("d"),
            childs: vec![],
        };
        let c = FileTree {
            name: OsString::from("c"),
            childs: vec![d],
        };
        let b = FileTree {
            name: OsString::from("b"),
            childs: vec![c, e],
        };
        let expected_tree = FileTree {
            name: OsString::from("a"),
            childs: vec![b],
        };

        test_single_tree_creation(&lines, expected_tree);
    }

    #[test]
    fn test_tree_creation2() {
        let lines = ["a", "a/b/e", "a/b", "a/b/c/d"];
        let e = FileTree {
            name: OsString::from("e"),
            childs: vec![],
        };
        let d = FileTree {
            name: OsString::from("d"),
            childs: vec![],
        };
        let c = FileTree {
            name: OsString::from("c"),
            childs: vec![d],
        };
        let b = FileTree {
            name: OsString::from("b"),
            childs: vec![e, c],
        };
        let expected_tree = FileTree {
            name: OsString::from("a"),
            childs: vec![b],
        };

        test_single_tree_creation(&lines, expected_tree);
    }

    #[test]
    fn test_trees_creation() {
        let lines = ["a", "a/b", "c/d"];
        let d = FileTree {
            name: OsString::from("d"),
            childs: vec![],
        };
        let c = FileTree {
            name: OsString::from("c"),
            childs: vec![d],
        };
        let b = FileTree {
            name: OsString::from("b"),
            childs: vec![],
        };
        let a = FileTree {
            name: OsString::from("a"),
            childs: vec![b],
        };

        let trees = make_trees(&mut lines.iter());
        assert_eq!(2, trees.len());
        assert_eq!(a, trees[0]);
        assert_eq!(c, trees[1]);
    }

    #[test]
    fn test_print_line() {
        let name = OsStr::new("abc\ndef");

        let mut output1 = vec![];
        print_line(&mut output1, &[], name).unwrap();
        assert_eq!(b"abc?def\n", &*output1);

        let mut output2 = vec![];
        print_line(&mut output2, &[true, false, true], name).unwrap();
        assert_eq!("    │   └── abc?def\n".as_bytes(), &*output2);

        let mut output3 = vec![];
        print_line(&mut output3, &[true, false, false], name).unwrap();
        assert_eq!("    │   ├── abc?def\n".as_bytes(), &*output3);
    }

}
