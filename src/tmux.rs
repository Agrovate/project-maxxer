use std::process::{Command, Stdio};

pub fn run_tmux(pname: &str, project: &str, command: &[String]) {
    // Check if the tmux session exists
    let session_exists = Command::new("tmux")
        .args(["has-session", "-t", pname])
        .stderr(Stdio::null())
        .status()
        .expect("failed to check tmux session")
        .success();

    // Create the session if it doesn't exist
    if !session_exists {
        Command::new("tmux")
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
    }

    // Prefix + o:
    //
    // display-popup
    //     ↓
    // direnv exec <project> <command>
    //
    // This makes direnv load the project's .envrc even when
    // pm itself was launched from another directory.
    let mut tmux_args = vec![
        "bind-key".to_string(),
        "-T".to_string(),
        "prefix".to_string(),
        "o".to_string(),
        "display-popup".to_string(),
        "-E".to_string(),
        "-d".to_string(),
        project.to_string(),
        "-h".to_string(),
        "80%".to_string(),
        "-w".to_string(),
        "80%".to_string(),
        "direnv".to_string(),
        "exec".to_string(),
        project.to_string(),
    ];

    // Add the actual project command
    //
    // cargo run
    // python main.py
    // npm run dev
    // etc.
    tmux_args.extend(command.iter().cloned());

    Command::new("tmux")
        .args(&tmux_args)
        .status()
        .expect("failed to configure tmux popup");

    // Attach to the project session
    Command::new("tmux")
        .args(["attach-session", "-t", pname])
        .status()
        .expect("failed to attach to tmux session");
}
