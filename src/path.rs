use std::env;
use std::path::{PathBuf, Path};

pub struct PathParser <'a> {
    pub parsers: Vec<&'a Fn(&PathBuf) -> Option<(usize, PathBuf)>>,
}

impl<'a> PathParser<'a> {
    pub fn path(&self, len: usize) -> Option<String> {
        let pwd = env::current_dir().unwrap();
        let mut path: (usize, PathBuf) = (len, pwd.clone());

        for parser in &self.parsers {
            match parser(&pwd) {
                Some(v) => {
                    path = v;
                    break;
                },
                None => continue
            }
        }

        let mut it = path.1.iter();
        let mut size = path.1.iter().count();
        size -= 1;
        let mut hold = PathBuf::from(it.next().unwrap());

        if size > path.0 {
            hold.push("...");
        }

        while size > len  {
            it.next();
            size -= 1;
        }
        hold = hold.join(it.as_path());

        return Some(String::from(hold.to_str().unwrap()));
    }
    pub fn git(pwd: &PathBuf) -> Option<(usize, PathBuf)> {
        // will explode if root is git
        for p in pwd.ancestors() {
            let mut git_dir = p.to_path_buf();
            git_dir.push(".git");
            if git_dir.exists() {
                let path = pwd.strip_prefix(p.parent().unwrap()).unwrap();
                let path = PathBuf::from("\u{e0a0}").join(path);
                return Some((3, path.to_path_buf()));
            }
        }
        None
    }
    pub fn default(pwd: &PathBuf) -> Option<(usize, PathBuf)> {
        match env::var("HOME") {
            Ok(home) => {
                match pwd.strip_prefix(PathBuf::from(home)) {
                    Ok(stripped) => {
                        Some((2, Path::new("~/").join(stripped)))
                    }
                    Err(_) => return None
                }
            },
            Err(_) => None
        }
    }
}
