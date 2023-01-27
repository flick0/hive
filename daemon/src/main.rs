use std::fs::{create_dir_all, File};
use std::path::Path;

use calloop::EventLoop;
use daemonize::Daemonize;

use anyhow::{Context,Result,bail};

pub mod render;
pub mod util;

fn main() -> Result<()> {
    let no_daemon = std::env::args().any(|arg| arg == "--no-daemon");

    if no_daemon {
        run_main_loop().with_context(||"failed to run main loop outside daemon")?;
    }

    // make daemon

    let tmp_path = Path::new("/tmp/hive");

    if !tmp_path.exists() && !tmp_path.is_dir() {
        create_dir_all(tmp_path).unwrap();
    }

    let stdout = File::create(tmp_path.join("hive.out")).unwrap();
    let stderr = File::create(tmp_path.join("hive.err")).unwrap();

    let daemonize: Daemonize<Result<()>> = Daemonize::new()
        .pid_file("/tmp/hive/hive.pid") // Every method except `new` and `start`
        .working_directory("/tmp") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .group(2) // or group id.
        .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/hive.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/hive.err`.
        .exit_action(||println!("started daemon"))
        .privileged_action(run_main_loop);

    match daemonize.start(){
        Ok(_) => return Ok(()),
        Err(e) => {
            bail!("failed to start daemon: {}",e)
        }
    };
}

fn run_main_loop() -> Result<()> {
    let (exec, sched) = calloop::futures::executor()?;

    let mut event_loop = EventLoop::try_new()?;
    let handle = event_loop.handle();

    handle
        .insert_source(exec, |evt, _metadata, _shared| {
            // Print the value of the async block ie. the return value.
            println!("Async block ended with: {:#?}", evt);
        })
        .map_err(|e| e.error)?;

    sched.schedule(main_loop()).unwrap();

    println!("Starting event loop. Use Ctrl-C to exit.");
    event_loop.run(None, &mut (), |_| {})?;
    println!("Event loop ended.");

    Ok(())
}

async fn main_loop() -> Result<()> {
    println!("started async daemon woooooo");
    std::fs::read("lalala").with_context(||"failed to read file oooo")?;
    Ok(())
}
