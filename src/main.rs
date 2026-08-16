use std::env;
use std::fs::{self};
use std::process::{Command,Stdio};
use std::io::Write;
use std::path::{Path};


fn main() {
    let path = path_return();
    println!("Path: {}",path);

    let projects = dirs_return(&path);

    let project = run_fzf(&projects);
    let p_name = Path::new(&project).file_name().unwrap().to_str().unwrap();
    run_tmux(p_name);


}

fn path_return() -> String {
    match env::var("PROJECT_DIR") {
        Ok(val) => val,
        Err(_) => {
            let mut home = env::var_os("HOME").unwrap().to_string_lossy().to_string();
            home.push_str("/Projects");
            println!("defaulting to {}",home);
            home
        },
    }
}

fn dirs_return(path:&str) -> Vec<String> {
    let mut dirs:Vec<String> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        if entry.as_ref().unwrap().file_type().unwrap().is_dir() {
            dirs.push(entry.unwrap().path().to_string_lossy().to_string());
        }
    }
    dirs
}

fn run_fzf(dirs:&[String]) -> String {
    //find ~/Projects -maxdepth 1 | fzf
    let dirs_ouptut = dirs.join("\n");
    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(mut stdin) = fzf.stdin.take() {
        stdin.write_all(dirs_ouptut.as_bytes()).unwrap();
    }

    let output = fzf.wait_with_output().unwrap().stdout;
    let output_str = String::from_utf8(output);

    output_str.unwrap().trim().to_string()
}

fn run_tmux(pname: &str) {
    let status = Command::new("tmux")
        .arg("has-session")
        .arg("-t")
        .arg(pname)
        .stderr(Stdio::null())
        .status();

    if status.unwrap().success() {
        Command::new("tmux")
            .arg("attach-session")
            .arg("-t")
            .arg(pname)
            .status()
            .unwrap();
    }
    else {
        Command::new("tmux")
            .arg("new-session")
            .arg("-d")
            .arg("-s")
            .arg(pname)
            .status().unwrap();

        Command::new("tmux")
            .arg("attach-session")
            .arg("-t")
            .arg(pname)
            .status()
            .unwrap();
    }
}
