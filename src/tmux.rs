use std::process::{Command, Stdio};

pub fn run_tmux(pname: &str, project: &str,_args: &[String]) {
    let status = Command::new("tmux")
        .args(["has-session", "-t", pname])
        .stderr(Stdio::null())
        .status()
        .expect("failed to check tmux session");

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
    let pre_args = ["bind-key".to_string(),"-T".to_string(),"prefix".to_string(),"o".to_string(),"display-popup".to_string(),"-E".to_string(),"-w".to_string(),"80%".to_string(),"-h".to_string(),"80%".to_string(),"-d".to_string(),"#{pane_current_path}".to_string(),];
    let args: Vec<String> = pre_args.iter().chain(_args.iter()).cloned().collect();
    Command::new("tmux")
        .args(args)
        .status()
        .expect("failed to configure tmux popup");

    Command::new("tmux")
        .args(["attach-session", "-t", pname])
        .status()
        .expect("failed to attach to tmux session");
}
