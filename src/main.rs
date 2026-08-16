mod tmux;
mod runners;

use std::env;
use std::fs::{self};
use std::process::{Command,Stdio};
use std::io::Write;
use std::path::{Path};

use tmux::run_tmux;
use runners::select_runner;


fn main() {
    // start with the path and get all directory paths present
    let path = path_return();
    let projects = dirs_return(&path);

    // get the project directory path and check if empty
    let project = run_fzf(&projects);
    if project.is_empty() {
        return;
    }

    // selects type of project by seeing the root makers
    let runner = select_runner(&project);

    // arguments for the auto running and get working directory name
    let args = runner.unwrap().command(&project);
    let p_name = Path::new(&project).file_name().unwrap().to_str().unwrap();
    run_tmux(p_name, &project,&args); //tmux mode
}


fn path_return() -> String {
    match env::var("PROJECT_DIR") { //Check if env variable is set if not default to ~/Projects
        Ok(val) => val, // Returns the path
        Err(_) => {
            let mut home = env::var_os("HOME").unwrap().to_string_lossy().to_string();
            home.push_str("/Projects");
            home // Returns /home/snow/Projects
        },
    }
}

fn dirs_return(path:&str) -> Vec<String> {
    let mut dirs:Vec<String> = Vec::new();

    // Check every entry in the given directory and push it to the dirs array
    for entry in fs::read_dir(path).unwrap() {
        if entry.as_ref().unwrap().file_type().unwrap().is_dir() {
            dirs.push(entry.unwrap().path().to_string_lossy().to_string());
        }
    }
    dirs
}

// Join all the Directories present into a single String and pipe to fzf
// Simillar to the Command find . -maxdepth 1 | fzf
fn run_fzf(dirs:&[String]) -> String {
    let dirs_ouptut = dirs.join("\n");

    let mut fzf = Command::new("fzf") //Runs the shell command
        .stdin(Stdio::piped()) // allows fzf to take stdin
        .stdout(Stdio::piped()) // allows fzf to take stdout
        .spawn() // spawns a process
        .unwrap();

    let mut stdin = fzf.stdin.take().unwrap(); //takes ownership of stdin
    stdin.write_all(dirs_ouptut.as_bytes()).unwrap(); //puts the input

    let output = fzf.wait_with_output().unwrap().stdout; //recieves the selected dir
    let output_str = String::from_utf8(output);

    output_str.unwrap().trim().to_string()
}

