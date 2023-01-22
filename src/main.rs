use std::{path::PathBuf, process::Stdio};

fn main() {
    let no_daemon = std::env::args().any(|arg| arg == "--no-daemon");
    if is_daemon_running().unwrap_or(false) {
        eprintln!("daemon already running");
        return;
    }
    if let Err(e) = spawn_daemon(no_daemon) {
        eprintln!("{}", e);
    }
}

fn spawn_daemon(no_daemon: bool) -> Result<(), String> {
    let cmd = "hive-daemon";
    if no_daemon {
        match std::process::Command::new(cmd).arg("--no-daemon").status() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("error spawning hive-daemon: {}", e)),
        }
    } else {
        match std::process::Command::new(cmd)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("error spawning hive-daemon: {}", e)),
        }
    }
}

fn is_daemon_running() -> Result<bool, String> {
    let proc = PathBuf::from("/proc");

    let entries = match proc.read_dir() {
        Ok(e) => e,
        Err(e) => return Err(e.to_string()),
    };

    for entry in entries.flatten() {
        let dirname = entry.file_name();
        if let Ok(pid) = dirname.to_string_lossy().parse::<u32>() {
            if std::process::id() == pid {
                continue;
            }
            let mut entry_path = entry.path();
            entry_path.push("cmdline");
            if let Ok(cmd) = std::fs::read_to_string(entry_path) {
                let mut args = cmd.split(&[' ', '\0']);
                if let Some(arg0) = args.next() {
                    if arg0.ends_with("hive-daemon") {
                        return Ok(true);
                    }
                }
            }
        }
    }
    Ok(false)
}
