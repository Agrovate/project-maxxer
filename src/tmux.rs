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

    let mut tmux_args = vec![
        "bind-key".to_string(),
        "-T".to_string(),
        "prefix".to_string(),
        "o".to_string(),
        "display-popup".to_string(),
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

    tmux_args.extend(command.iter().cloned());

    Command::new("tmux")
        .args(&tmux_args)
        .status()
        .expect("failed to configure tmux popup");

    Command::new("tmux")
        .args(["attach-session", "-t", pname])
        .status()
        .expect("failed to attach to tmux session");
}
