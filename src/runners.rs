use std::{collections::BTreeMap, fs::read_dir, fs::read_to_string, path::Path};
use serde::Deserialize;

#[derive(Deserialize)]
struct PyProject {
    project: Option<PyProjectTable>,
}

#[derive(Deserialize)]
struct PyProjectTable {
    name: Option<String>,
    scripts: Option<BTreeMap<String, String>>,
}

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
        if let Some(script) = console_script(dir) {
            return vec!["uv".to_string(), "run".to_string(), script];
        }

        let files = get_files(dir, "py");
        let file = files
            .iter()
            .find(|f| Path::new(f).file_name().and_then(|n| n.to_str()) == Some("main.py"))
            .or_else(|| files.first())
            .cloned()
            .unwrap_or_else(|| "main.py".to_string());
        vec!["uv".to_string(),"run".to_string(), file]
    }
}

fn console_script(dir: &str) -> Option<String> {
    let contents = read_to_string(Path::new(dir).join("pyproject.toml")).ok()?;
    let parsed: PyProject = toml::from_str(&contents).ok()?;
    let project = parsed.project?;
    let scripts = project.scripts?;

    if scripts.len() == 1 {
        return scripts.into_keys().next();
    }

    project
        .name
        .as_ref()
        .and_then(|name| scripts.contains_key(name).then(|| name.clone()))
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
        else if path.is_dir() {
            let is_hidden = path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with('.'));
            if !is_hidden {
                let mut sub_files = get_files(path.to_str().unwrap(), ext);
                files.append(&mut sub_files);
            }
        }

    }
    files
}
