use std::process::{Command, Stdio};

pub fn run_tmux(pname: &str, project: &str, command: &[String]) {
    let session_exists = Command::new("tmux")
        .args(["has-session", "-t", pname])
        .stderr(Stdio::null())
        .status()
        .expect("failed to check tmux session")
        .success();

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

    let path = std::env::var("PATH")
        .expect("PATH environment variable is not set");

    Command::new("tmux")
        .args(["set-environment", "-g", "PATH"])
        .arg(&path)
        .status()
        .expect("failed to update tmux PATH");

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
    ];

    // Append the actual project command.
    tmux_args.extend(command.iter().cloned());

    // Register Prefix + o.
    let status = Command::new("tmux")
        .args(&tmux_args)
        .status()
        .expect("failed to configure tmux popup");

    if !status.success() {
        eprintln!("warning: failed to configure Prefix + o");
    }

    // Attach to the project session.
    Command::new("tmux")
        .args(["attach-session", "-t", pname])
        .status()
        .expect("failed to attach to tmux session");
}
