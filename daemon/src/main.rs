use std::path::Path;
use std::fs::{File,create_dir_all};

use daemonize::Daemonize;

fn main() -> Result<(), String> {

    let no_daemon = std::env::args().any(|arg| arg == "--no-daemon");
    
    if no_daemon {
        return main_loop();
    }
    
    let tmp_path = Path::new("/tmp/hive");

    if !tmp_path.exists() && !tmp_path.is_dir() {
        create_dir_all(tmp_path).unwrap();
    }
    
    let stdout = File::create(tmp_path.join("hive.out")).unwrap();
    let stderr = File::create(tmp_path.join("hive.err")).unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/hive/hive.pid") // Every method except `new` and `start`
        .working_directory("/tmp") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/hive.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/hive.err`.
        .exit_action(on_exit)
        .privileged_action(main_loop);

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => {
            return Err(format!("Error starting daemon, {}",e.to_string()))
        },
    }
    Ok(())
}

fn main_loop() -> Result<(), String> {
    loop {
        println!("running daemon");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    //Ok(())
}

fn on_exit() {
    println!("started daemon")
}