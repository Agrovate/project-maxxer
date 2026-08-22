use std::path::Path;

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
        let path = Path::new(dir);
        let p_name = path.file_name()
                        .and_then(|os_str| os_str.to_str())
                        .unwrap_or("main");

        vec!["uv".into(), "run".into(), p_name.into()]

    }
}


// return arguments to run command: make
impl Runner for Make {
    fn command(&self, _dir:&str) -> Vec<String> {
        vec!["make".into()]
    }
}
