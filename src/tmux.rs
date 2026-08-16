use std::process::{Command, Stdio};

pub fn run_tmux(pname: &str, project: &str, command: &[String]) {
    // Check if the tmux session already exists
    let status = Command::new("tmux")
        .args(["has-session", "-t", pname])
        .stderr(Stdio::null())
        .status()
        .expect("failed to check tmux session");

    // Create the session if it doesn't exist
    if !status.success() {
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

    let mut tmux_args = vec![
        "bind-key".to_string(),
        "-T".to_string(),
        "prefix".to_string(),
        "o".to_string(),
        "display-popup".to_string(),
        "-E".to_string(),
        "-w".to_string(),
        "80%".to_string(),
        "-h".to_string(),
        "80%".to_string(),
        "-d".to_string(),
        "#{pane_current_path}".to_string(),
    ];

    tmux_args.extend(command.iter().cloned());

    // Register Prefix + o
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
