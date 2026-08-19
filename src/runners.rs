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

// Selecting with the help of root makers
pub fn select_runner(dir: &str) -> Option<Box<dyn Runner>> {
    if Path::new(dir).join("Cargo.toml").exists() {
        Some(Box::new(Cargo))
    }
    else if Path::new(dir).join("pyproject.toml").exists() {
        Some(Box::new(Uv))
    }
    else if Path::new(dir).join("Makefile").exists() {
        Some(Box::new(Make))
    }
    else {
        None
    }
}


// return arguments to run command: cargo run
impl Runner for Cargo {
    fn command(&self, _dir:&str) -> Vec<String> {
        vec!["cargo".into(),"run".into()]
    }
}

// return arguments to run command: uv run <script>
impl Runner for Uv {
    fn command(&self, dir:&str) -> Vec<String> {
        if let Some(script) = console_script(dir) {
            return vec!["uv".into(), "run".into(), script];
        }

        let files = get_files(dir, "py");

        // find the main.py file or the first python file it gets and run that
        let file = files
            .iter()
            .find(|f| Path::new(f).file_name().and_then(|n| n.to_str()) == Some("main.py"))
            .or_else(|| files.first())
            .cloned()
            .unwrap_or_else(|| "main.py".to_string());
        vec!["uv".into(),"run".into(), file]
    }
}

// get path to the pyproject.toml file,
// get the content present in the pyproject.toml
// split content to project and scripts
// if there is only one script then return the value
// else read the project name and check if name is present in project and if true return name
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


// return arguments to run command: make
impl Runner for Make {
    fn command(&self, _dir:&str) -> Vec<String> {
        vec!["make".into()]
    }
}


fn get_files(path:&str, ext:&str) -> Vec<String> {

    // create a vector of files
    let mut files:Vec<String> = Vec::new();

    // for every path in the directory:
    //      check if it's a file and if yes check if it has a valid extention
    for entry in read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some(ext) {
            files.push(path.to_string_lossy().into());
        }

        // check if it is a directory:
        //      if yes check if it's hidden:
        //          if yes: ignore
        //          else: add the files into the main files Vector
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
