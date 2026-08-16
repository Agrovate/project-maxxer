use std::env;
use std::process::{Command, Stdio};

pub fn run_tmux(pname: &str, project: &str, command: &[String]) {
    // Check whether the session exists.
    let session_exists = Command::new("tmux")
        .args(["has-session", "-t", pname])
        .stderr(Stdio::null())
        .status()
        .expect("failed to check tmux session")
        .success();

    // Create the session if it doesn't exist.
    if !session_exists {
        let status = Command::new("tmux")
            .args([
                "new-session",
                "-d",
                "-s",
                pname,
                "-c",
                project,
            ])
            .status()
            .expect("failed to create tmux session");

        if !status.success() {
            panic!("failed to create tmux session");
        }
    }

    // Get the PATH from the environment in which pm is running.
    //
    // When running:
    //
    //     nix develop
    //     pm
    //
    // this contains the Cargo/Python/etc. paths from the dev shell.
    let path = env::var("PATH")
        .expect("PATH environment variable is not set");

    // Build:
    //
    // bind-key -T prefix o display-popup
    //     -E
    //     -d "#{pane_current_path}"
    //     -h 80%
    //     -w 80%
    //     -e PATH=<pm's PATH>
    //     <command>
    //
    let mut tmux_args = vec![
        "bind-key".to_string(),
        "-T".to_string(),
        "prefix".to_string(),
        "o".to_string(),

        "display-popup".to_string(),
        "-E".to_string(),

        "-d".to_string(),
        "#{pane_current_path}".to_string(),

        "-h".to_string(),
        "80%".to_string(),

        "-w".to_string(),
        "80%".to_string(),

        // Give the popup the PATH from pm directly.
        "-e".to_string(),
        format!("PATH={}", path),
    ];

    // Add the actual command.
    //
    // ["cargo", "run"]
    // ["python", "main.py"]
    // ["npm", "run", "dev"]
    // etc.
    tmux_args.extend(command.iter().cloned());

    // Register Prefix + o.
    let output = Command::new("tmux")
        .args(&tmux_args)
        .output()
        .expect("failed to configure tmux popup");

    if !output.status.success() {
        eprintln!(
            "tmux bind failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Attach to the project session.
    Command::new("tmux")
        .args(["attach-session", "-t", pname])
        .status()
        .expect("failed to attach to tmux session");
}
