use std::process::{Command, Stdio};

pub fn run_tmux(pname: &str, project: &str, command: &[String]) {
    // Check if the session exists
    let status = Command::new("tmux")
        .args(["has-session", "-t", pname])
        .stderr(Stdio::null())
        .status()
        .expect("failed to check tmux session");

    // Create session if it doesn't exist
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

    // Turn:
    //
    // ["cargo", "run"]
    //
    // into:
    //
    // cargo run
    //
    // Or:
    //
    // ["python", "main.py"]
    //
    // into:
    //
    // python main.py
    //
    let command = command
        .iter()
        .map(|arg| {
            // Basic shell escaping
            if arg.contains(' ')
                || arg.contains('"')
                || arg.contains('\'')
                || arg.contains('$')
                || arg.contains('`')
            {
                format!("'{}'", arg.replace('\'', "'\\''"))
            } else {
                arg.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    // Build the tmux binding
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
        "sh".to_string(),
        "-c".to_string(),
        command,
    ];

    let output = Command::new("tmux")
        .args(&tmux_args)
        .output()
        .expect("failed to configure tmux popup");

    if !output.status.success() {
        eprintln!(
            "tmux error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Attach to session
    Command::new("tmux")
        .args(["attach-session", "-t", pname])
        .status()
        .expect("failed to attach to tmux session");
}
