use std::{fs::read_dir, path::Path};



pub trait Runner {
    fn command(&self, dir:&str) -> Vec<String>;
}

struct Cargo;
struct Uv;
struct Make;


pub fn select_runner(dir: &str) -> Option<Box<dyn Runner>> {
    if Path::new(dir).join("Cargo.toml").exists() {
        return Some(Box::new(Cargo));
    }
    else if Path::new(dir).join("pyproject.toml").exists() {
        return Some(Box::new(Uv));
    }
    else if Path::new(dir).join("Makefile").exists() {
        return Some(Box::new(Make));
    }
    None
}


impl Runner for Cargo {
    fn command(&self, _dir:&str) -> Vec<String> {
        vec!["cargo".to_string(),"run".to_string()]
    }
}


impl Runner for Uv {
    fn command(&self, dir:&str) -> Vec<String> {
        let files = get_files(dir, ".py");
        let file = match files.first() {
            Some(s) => s,
            _ => "main.py"
        };
        vec!["uv".to_string(),"run".to_string(), file.to_string()]
    }
}

impl Runner for Make {
    fn command(&self, _dir:&str) -> Vec<String> {
        vec!["make".to_string()]
    }
}


fn get_files(path:&str, ext:&str) -> Vec<String>{
    let mut files:Vec<String> = Vec::new();
    for entry in read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some(ext) {
            files.push(path.to_string_lossy().to_string());
        }

    }
    files
}
